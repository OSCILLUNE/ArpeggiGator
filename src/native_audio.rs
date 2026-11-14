//! Experimental native audio path for the virtual MPC running 1.3.0.12.

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering::*};

use mpc_sample_sdk::{NoteEvent, NoteFlow, hook, image, platform::clock};

use crate::{
    PAD_MASK, PREVIEW_ENABLED, PREVIEW_STATE, Pattern, Playback, ScheduledEvent, StepRate, tempo,
};

const RENDER_OFFSET: usize = 0x18dbcc0;
const RENDER_VTABLE: usize = 0x6e3b220;
const ENGINE_OFFSET: usize = 0x5c0;
const ENGINE_VTABLE: usize = 0x6e7f2d0;
const DISPATCH_OFFSET: usize = 0x211b090;
const PITCH_OFFSET: usize = 0x27e4400;
const PAD_SOURCE: u32 = 0x7fff_fffa;

static DISPATCH: AtomicUsize = AtomicUsize::new(0);
static PITCH: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
static RENDER_ACTIVE: AtomicBool = AtomicBool::new(false);
static PADS: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];
static SERIAL: AtomicU32 = AtomicU32::new(0);
static PATTERN: AtomicUsize = AtomicUsize::new(0);
static RATE: AtomicUsize = AtomicUsize::new(StepRate::Sixteenth as usize);

#[unsafe(no_mangle)]
static g_arp_audio_on: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static g_arp_audio_off: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static g_arp_audio_last: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy)]
struct Key {
    note: u8,
    pitch: i8,
    velocity: u16,
    serial: u32,
}

impl Key {
    fn unpack(bits: u64) -> Self {
        Self {
            note: bits as u8,
            pitch: (bits >> 8) as i8,
            velocity: (bits >> 16) as u16,
            serial: (bits >> 32) as u32,
        }
    }
}

struct State {
    playback: Playback,
    inputs: [u64; 16],
    voice: Option<Key>,
    pattern: Pattern,
    engine: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            playback: Playback::default(),
            inputs: [0; 16],
            voice: None,
            pattern: Pattern::Up,
            engine: 0,
        }
    }
}

static mut STATE: Option<State> = None;

pub fn begin_press(pad: usize) {
    PADS[pad].store(0, Release);
}

pub fn set_pattern(pattern: Pattern) {
    PATTERN.store(pattern as usize, Release);
}

pub fn set_rate(rate: StepRate) {
    RATE.store(rate as usize, Release);
}

pub fn capture(event: &mut NoteEvent<'_>) -> Option<NoteFlow> {
    if !READY.load(Acquire)
        || !event.note_on
        || event.source != u64::from(PAD_SOURCE)
        || event.context != 0
        || event.note >= 128
    {
        return None;
    }
    let pad = event.physical_pad?;
    let index = usize::try_from(pad.pad).ok().filter(|&pad| pad < 16)?;
    let pitch = i8::try_from(pad.pitch?).ok()?;
    let state = PREVIEW_STATE.load(Acquire);
    if state & PREVIEW_ENABLED == 0 || tempo::current_bpm().is_none() {
        return None;
    }
    if state & (1 << index) != 0 {
        let serial = SERIAL.fetch_add(1, Relaxed).wrapping_add(1);
        let velocity = (event.velocity.clamp(0.0, 1.0) * f32::from(u16::MAX)).round() as u16;
        let bits = (u64::from(serial) << 32)
            | (u64::from(velocity.max(1)) << 16)
            | (u64::from(pitch as u8) << 8)
            | u64::from(event.note);
        PADS[index].store(bits, Release);
    }
    Some(NoteFlow::Consume)
}

pub fn install() {
    let base = image::base();
    let signatures = [
        (
            RENDER_OFFSET,
            [0xd503233f, 0xd10383ff, 0xa9077bfd, 0x9101c3fd],
        ),
        (
            DISPATCH_OFFSET + 4,
            [0xd10303ff, 0xa9067bfd, 0x910183fd, 0xa90753f3],
        ),
        (
            PITCH_OFFSET + 4,
            [0xa9bf7bfd, 0x2a0103e4, 0x2a0203e5, 0x910003fd],
        ),
    ];
    // The two existing SDK hooks have displaced their first instruction.
    if base == 0
        || !signatures.into_iter().all(|(offset, words)| {
            words
                .into_iter()
                .enumerate()
                .all(|(i, word)| image::read_u32(base + offset + i * 4) == Some(word))
        })
    {
        return;
    }
    let Some((dispatch, pitch)) = mpc_sample_sdk::sites::note_trampolines() else {
        return;
    };
    DISPATCH.store(dispatch, Relaxed);
    PITCH.store(pitch, Relaxed);
    // Startup only, before any render callback can run.
    let mut hooks = match hook::HookEngine::new(
        // SAFETY: SDK already verified this firmware during installation.
        unsafe { image::Firmware::from_base(base) },
    ) {
        Ok(hooks) => hooks,
        Err(_) => return,
    };
    // SAFETY: the signature above verifies the exact render entry.
    if let Ok(original) = unsafe { hooks.prepare(RENDER_OFFSET, render as *const () as usize) } {
        ORIGINAL.store(original, Relaxed);
        if hooks.seal().is_ok() && hooks.commit().is_ok() {
            // The hooks and their trampolines live for the duration of the MPC process.
            return;
        }
    }
    hooks.abort();
}

type Render = unsafe extern "C" fn(usize, usize, usize);

unsafe extern "C" fn render(renderer: usize, block: usize, midi: usize) {
    let original = ORIGINAL.load(Relaxed);
    // SAFETY: ORIGINAL is populated from the guarded trampoline before patching.
    let forward = unsafe { core::mem::transmute::<usize, Render>(original) };
    // SAFETY: the live renderer object and its native engine are borrowed from
    // this callback. The firmware constructor owns the engine at +0x5c0.
    unsafe {
        let base = image::base();
        if (renderer as *const usize).read() == base + RENDER_VTABLE {
            let engine = ((renderer + ENGINE_OFFSET) as *const usize).read();
            if engine != 0
                && (engine as *const usize).read() == base + ENGINE_VTABLE
                && !RENDER_ACTIVE.swap(true, Acquire)
            {
                let state =
                    (&mut *core::ptr::addr_of_mut!(STATE)).get_or_insert_with(State::default);
                state.tick(engine);
                READY.store(true, Release);
                RENDER_ACTIVE.store(false, Release);
            }
        }
        forward(renderer, block, midi);
    }
}

impl State {
    unsafe fn tick(&mut self, engine: usize) {
        if self.engine != engine {
            // A replacement engine owns different voices. Never call an old pointer.
            *self = Self {
                engine,
                ..Self::default()
            };
            for pad in &PADS {
                pad.store(0, Release);
            }
        }

        let preview = PREVIEW_STATE.load(Acquire);
        let enabled = preview & PREVIEW_ENABLED != 0;
        let bpm = tempo::current_bpm();
        // SAFETY: the only native argument is the live engine from this render.
        unsafe {
            let off = self.playback.set_enabled(enabled && bpm.is_some());
            self.output(engine, off);
            let pattern = Pattern::ALL[PATTERN.load(Acquire)];
            if self.pattern != pattern {
                self.playback.set_pattern(pattern);
                self.pattern = pattern;
            }
            let off = self.playback.set_rate(StepRate::ALL[RATE.load(Acquire)]);
            self.output(engine, off);
            let mut pending = [(0, 0); 16];
            let mut count = 0;
            for (pad, (input, slot)) in self.inputs.iter_mut().zip(&PADS).enumerate() {
                let next = if enabled && bpm.is_some() && preview & PAD_MASK & (1 << pad) != 0 {
                    slot.load(Acquire)
                } else {
                    0
                };
                if *input == next {
                    continue;
                }
                if *input != 0 {
                    let off = self.playback.release(pad as u8);
                    if off.is_some()
                        && let Some(key) = self.voice.take()
                    {
                        send(engine, key, false);
                    }
                }
                *input = next;
                if next != 0 {
                    pending[count] = (pad, next);
                    count += 1;
                }
            }
            pending[..count].sort_unstable_by_key(|&(_, key)| Key::unpack(key).serial);
            for &(pad, bits) in &pending[..count] {
                let off = self.playback.press(
                    pad as u8,
                    f32::from(Key::unpack(bits).velocity) / f32::from(u16::MAX),
                );
                debug_assert!(off.is_none());
            }
            let now = clock::now_ms().max(0) as u64 * 1000;
            for event in self.playback.tick(now, bpm) {
                self.output(engine, event);
            }
        }
    }

    unsafe fn output(&mut self, engine: usize, event: Option<ScheduledEvent>) {
        // SAFETY: callers supply a live engine from this render callback.
        unsafe {
            match event {
                Some(ScheduledEvent::NoteOn(step)) => {
                    let key = Key::unpack(self.inputs[usize::from(step.pad)]);
                    send(engine, key, true);
                    self.voice = Some(key);
                }
                Some(ScheduledEvent::NoteOff { .. }) => {
                    if let Some(key) = self.voice.take() {
                        send(engine, key, false);
                    }
                }
                None => {}
            }
        }
    }
}

/// Inline pad event as constructed by the guarded firmware's 0x27e54d0.
#[repr(C, align(8))]
struct PadPacket([u8; 56]);

unsafe fn send(engine: usize, key: Key, on: bool) {
    let mut packet = PadPacket([0; 56]);
    packet.0[8] = 3;
    let velocity = if on {
        f32::from(key.velocity) / f32::from(u16::MAX)
    } else {
        0.0
    };
    packet.0[16..20].copy_from_slice(&velocity.to_le_bytes());
    packet.0[0x16..0x1e].copy_from_slice(&0x0064_3200_0000_7878_u64.to_le_bytes());
    packet.0[0x30] = key.note;
    packet.0[0x32] = 100;
    packet.0[0x33] = 197;
    // SAFETY: the ABI and offsets are checked at startup. The native calls
    // borrow this stack event, with the same layout as the stock constructor.
    unsafe {
        let pitch = core::mem::transmute::<usize, unsafe extern "C" fn(usize, u64, u64, i32)>(
            PITCH.load(Relaxed),
        );
        let dispatch = core::mem::transmute::<
            usize,
            unsafe extern "C" fn(usize, usize, *mut PadPacket, u32),
        >(DISPATCH.load(Relaxed));
        pitch(
            packet.0.as_mut_ptr().add(0x14) as usize,
            0,
            0,
            i32::from(key.pitch),
        );
        dispatch(engine, 0, &mut packet, PAD_SOURCE);
    }
    if on {
        g_arp_audio_last.store(
            u32::from(key.note) | (u32::from(key.pitch as u8) << 8),
            Relaxed,
        );
        g_arp_audio_on.fetch_add(1, Relaxed);
    } else {
        g_arp_audio_off.fetch_add(1, Relaxed);
    }
}
