//! Experimental native audio path for the virtual MPC running 1.3.0.12.

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering::*};

use mpc_sample_sdk::{NoteEvent, NoteFlow, hook, image, platform::clock};

use crate::{
    BarLength, ChordMood, EngineMode, MelodyVariant, MutationIntensity, OrbitAtmosphere,
    OrbitDynamics, PAD_MASK, PREVIEW_ENABLED, PREVIEW_STATE, Pattern, Playback, SHORTCUT_PADS,
    ScheduledEvent, StepRate, VoicingStyle, tempo, transport_clock,
    transport_playback::TransportPlayback,
};

const RENDER_OFFSET: usize = 0x18dbcc0;
const RENDER_VTABLE: usize = 0x6e3b220;
const ENGINE_OFFSET: usize = 0x5c0;
const ENGINE_VTABLE: usize = 0x6e7f2d0;
const DISPATCH_OFFSET: usize = 0x211b090;
const PITCH_OFFSET: usize = 0x27e4400;
const KILL_ALL_VOICES: usize = 0x2118a20;
const PAD_SOURCE: u32 = 0x7fff_fffa;

unsafe fn kill_all_voices(engine: usize) {
    if engine == 0 {
        return;
    }
    let base = image::base();
    unsafe {
        let kill_all =
            core::mem::transmute::<usize, unsafe extern "C" fn(usize)>(base + KILL_ALL_VOICES);
        kill_all(engine);
    }
}

static DISPATCH: AtomicUsize = AtomicUsize::new(0);
static PITCH: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
static RENDER_ACTIVE: AtomicBool = AtomicBool::new(false);
static PADS: [AtomicU64; 16] = [const { AtomicU64::new(0) }; 16];
static SERIAL: AtomicU32 = AtomicU32::new(0);
static PATTERN: AtomicUsize = AtomicUsize::new(0);
static RATE: AtomicUsize = AtomicUsize::new(StepRate::Sixteenth as usize);
static ADDITIONAL_OCTAVES: AtomicUsize = AtomicUsize::new(0);
static PANIC: AtomicBool = AtomicBool::new(false);
pub static ACTIVE_SAMPLE_PAD: AtomicU8 = AtomicU8::new(0xff);
pub static ACTIVE_SAMPLE_NOTE: AtomicU8 = AtomicU8::new(36);

pub fn set_active_sample_pad(pad: u8, note: u8) {
    ACTIVE_SAMPLE_PAD.store(pad, Release);
    ACTIVE_SAMPLE_NOTE.store(note, Release);
}

#[allow(dead_code)]
pub fn active_sample_pad() -> Option<u8> {
    let pad = ACTIVE_SAMPLE_PAD.load(Acquire);
    if pad < 16 {
        Some(pad)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn active_sample_note() -> u8 {
    ACTIVE_SAMPLE_NOTE.load(Acquire)
}

pub fn is_transport_playing() -> bool {
    g_arp_transport_mode.load(Acquire) != 0
}

#[unsafe(no_mangle)]
static g_arp_audio_on: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static g_arp_audio_off: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static g_arp_audio_last: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static g_arp_transport_pulse: AtomicU64 = AtomicU64::new(0);
#[unsafe(no_mangle)]
static g_arp_transport_mode: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static g_arp_transport_resets: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static g_arp_mode: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Key {
    pad: u8,
    note: u8,
    pitch: i8,
    velocity: u16,
    serial: u32,
}

impl Key {
    fn unpack(bits: u64, pad: u8) -> Self {
        Self {
            pad,
            note: bits as u8,
            pitch: (bits >> 8) as i8,
            velocity: (bits >> 16) as u16,
            serial: (bits >> 32) as u32,
        }
    }
}

struct State {
    playback: Playback,
    transport_playback: TransportPlayback,
    transport_clock: transport_clock::UnwrapClock,
    synced: bool,
    inputs: [u64; 16],
    voice: Option<Key>,
    pattern: Pattern,
    engine: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            playback: Playback::default(),
            transport_playback: TransportPlayback::new(StepRate::Sixteenth, 960).unwrap(),
            transport_clock: transport_clock::UnwrapClock::default(),
            synced: false,
            inputs: [0; 16],
            voice: None,
            pattern: Pattern::Up,
            engine: 0,
        }
    }
}

static mut STATE: Option<State> = None;

pub fn begin_press(_pad: usize) {
    // No-op: capture() atomically stores fresh key bits for each press.
    // Zeroing PADS here races against capture() if NoteEvent arrives first.
}

static MODE: AtomicUsize = AtomicUsize::new(0);
static MELODY_BARS: AtomicUsize = AtomicUsize::new(0);
static MELODY_VARIANT: AtomicUsize = AtomicUsize::new(0);
static MUTATOR_INTENSITY: AtomicUsize = AtomicUsize::new(0);
static MUTATOR_VARIANT: AtomicUsize = AtomicUsize::new(0);
static MUTATOR_FROZEN: AtomicBool = AtomicBool::new(false);
static CHORD_MOOD: AtomicUsize = AtomicUsize::new(0);
static VOICING_STYLE: AtomicUsize = AtomicUsize::new(0);
static ORBIT_ATMOSPHERE: AtomicUsize = AtomicUsize::new(0);
static ORBIT_DYNAMICS: AtomicUsize = AtomicUsize::new(0);

pub fn set_mode(mode: EngineMode) {
    MODE.store(mode as usize, Release);
    g_arp_mode.store(mode as u32, Release);
}

pub fn set_melody_bars(bars: BarLength) {
    MELODY_BARS.store(bars as usize, Release);
}

pub fn set_melody_variant(variant: MelodyVariant) {
    MELODY_VARIANT.store(variant as usize, Release);
}

pub fn set_mutator_intensity(intensity: MutationIntensity) {
    MUTATOR_INTENSITY.store(intensity as usize, Release);
}

pub fn set_mutator_variant(variant: MelodyVariant) {
    MUTATOR_VARIANT.store(variant as usize, Release);
}

pub fn set_mutator_frozen(frozen: bool) {
    MUTATOR_FROZEN.store(frozen, Release);
}

pub fn set_chord_mood(mood: ChordMood) {
    CHORD_MOOD.store(mood as usize, Release);
}

pub fn set_voicing_style(style: VoicingStyle) {
    VOICING_STYLE.store(style as usize, Release);
}

pub fn set_orbit_atmosphere(atmosphere: OrbitAtmosphere) {
    ORBIT_ATMOSPHERE.store(atmosphere as usize, Release);
}

pub fn set_orbit_dynamics(dynamics: OrbitDynamics) {
    ORBIT_DYNAMICS.store(dynamics as usize, Release);
}

pub fn set_pattern(pattern: Pattern) {
    PATTERN.store(pattern as usize, Release);
}

pub fn set_rate(rate: StepRate) {
    RATE.store(rate as usize, Release);
}

pub fn set_additional_octaves(octaves: u8) {
    ADDITIONAL_OCTAVES.store(usize::from(octaves.min(3)), Release);
}

pub fn panic() {
    PANIC.store(true, Release);
    ACTIVE_SAMPLE_PAD.store(0xff, Release);
    ACTIVE_SAMPLE_NOTE.store(36, Release);
}

pub fn capture(event: &mut NoteEvent<'_>) -> Option<NoteFlow> {
    if !READY.load(Acquire)
        || event.source != u64::from(PAD_SOURCE)
        || event.context != 0
        || event.note >= 128
    {
        return None;
    }
    let state = PREVIEW_STATE.load(Acquire);
    if state & PREVIEW_ENABLED == 0 || tempo::current_bpm().is_none() {
        return None;
    }

    if let Some(pad) = event.physical_pad {
        let index = usize::try_from(pad.pad).ok().filter(|&pad| pad < 16)?;
        if SHORTCUT_PADS.load(Acquire) & (1 << index) != 0 {
            return Some(NoteFlow::Consume);
        }
        if crate::BACKGROUND_PLAYBACK_ACTIVE.load(Acquire) {
            // On SAMPLE screen outside 16 Levels:
            // Forward pad strike to firmware so MPC switches active sample pad in UI.
            // Dynamically update active sample pad & note for background playback.
            if event.note_on {
                set_active_sample_pad(index as u8, event.note);
            }
            return None;
        }
        if event.note_on {
            if ACTIVE_SAMPLE_PAD.load(Acquire) == 0xff {
                ACTIVE_SAMPLE_NOTE.store(event.note, Release);
            }
            let pitch = pad.pitch.and_then(|p| i8::try_from(p).ok()).unwrap_or(0);
            let serial = SERIAL.fetch_add(1, Relaxed).wrapping_add(1);
            let velocity = (event.velocity.clamp(0.0, 1.0) * f32::from(u16::MAX)).round() as u16;
            let bits = (u64::from(serial) << 32)
                | (u64::from(velocity.max(1)) << 16)
                | (u64::from(pitch as u8) << 8)
                | u64::from(event.note);
            PADS[index].store(bits, Release);
        }
        Some(NoteFlow::Consume)
    } else if event.note_off || !event.note_on {
        if crate::BACKGROUND_PLAYBACK_ACTIVE.load(Acquire) {
            return None;
        }
        // Any pad NoteOff from the pad source when ARP is active must be consumed
        // so the firmware engine does not prematurely close voices managed by ArpeggiGator.
        Some(NoteFlow::Consume)
    } else {
        None
    }
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

        let panic_requested = PANIC.swap(false, Acquire);
        if panic_requested {
            ACTIVE_SAMPLE_PAD.store(0xff, Release);
            ACTIVE_SAMPLE_NOTE.store(36, Release);
            unsafe {
                if let Some(key) = self.voice.take() {
                    send(engine, key, false);
                }
                for p in 0..16_u8 {
                    if self.inputs[usize::from(p)] != 0 {
                        let key = Key::unpack(self.inputs[usize::from(p)], p);
                        send(engine, key, false);
                        self.inputs[usize::from(p)] = 0;
                        PADS[usize::from(p)].store(0, Release);
                    }
                    let key0 = Key {
                        pad: p,
                        note: 0,
                        pitch: 0,
                        velocity: 0,
                        serial: 0,
                    };
                    send(engine, key0, false);
                    let key_std = Key {
                        pad: p,
                        note: 36 + p,
                        pitch: 0,
                        velocity: 0,
                        serial: 0,
                    };
                    send(engine, key_std, false);
                }
                kill_all_voices(engine);
            }
            let _ = self.playback.set_mode(EngineMode::Off);
        }

        let preview = PREVIEW_STATE.load(Acquire);
        let enabled = preview & PREVIEW_ENABLED != 0;
        let bpm = tempo::current_bpm();
        let mode = match MODE.load(Acquire) {
            1 => EngineMode::Arp,
            2 => EngineMode::Melody,
            3 => EngineMode::Mutator,
            4 => EngineMode::Acordator,
            5 => EngineMode::Orbitator,
            _ => EngineMode::Off,
        };
        let transport_snapshot = transport_clock::snapshot(engine, image::base());
        let synced = enabled && bpm.is_some() && transport_snapshot.is_some_and(|s| s.playing);

        // SAFETY: the only native argument is the live engine from this render.
        unsafe {
            if self.synced != synced {
                let off = self.playback.set_mode(EngineMode::Off);
                self.output(engine, off);
                let off = self.transport_playback.stop();
                self.output(engine, off);
                self.inputs = [0; 16];
                self.synced = synced;
                self.transport_clock = transport_clock::UnwrapClock::default();
                g_arp_transport_mode.store(u32::from(synced), Release);
            }

            let active_mode = if enabled && bpm.is_some() && !panic_requested {
                mode
            } else {
                EngineMode::Off
            };

            let rate_idx = RATE.load(Acquire) % StepRate::ALL.len();
            let rate = StepRate::ALL[rate_idx];

            if synced {
                let off = self.transport_playback.set_mode(active_mode);
                self.output(engine, off);
                let off = self.transport_playback.set_rate(rate);
                self.output(engine, off);

                if mode == EngineMode::Arp {
                    let pattern_idx = PATTERN.load(Acquire) % Pattern::ALL.len();
                    self.transport_playback.set_pattern(Pattern::ALL[pattern_idx]);
                    self.transport_playback
                        .set_additional_octaves((ADDITIONAL_OCTAVES.load(Acquire) % 4) as u8);
                } else if mode == EngineMode::Melody {
                    let bars_idx = MELODY_BARS.load(Acquire) % BarLength::ALL.len();
                    self.transport_playback.set_melody_bars(BarLength::ALL[bars_idx]);
                    let var_idx = MELODY_VARIANT.load(Acquire) % MelodyVariant::ALL.len();
                    self.transport_playback.set_melody_variant(MelodyVariant::ALL[var_idx]);
                } else if mode == EngineMode::Mutator {
                    let int_idx = MUTATOR_INTENSITY.load(Acquire) % MutationIntensity::ALL.len();
                    self.transport_playback
                        .set_mutator_intensity(MutationIntensity::ALL[int_idx]);
                    self.transport_playback
                        .set_mutator_frozen(MUTATOR_FROZEN.load(Acquire));
                    let var_idx = MUTATOR_VARIANT.load(Acquire) % MelodyVariant::ALL.len();
                    self.transport_playback.set_mutator_variant(MelodyVariant::ALL[var_idx]);
                } else if mode == EngineMode::Acordator {
                    let mood_idx = CHORD_MOOD.load(Acquire) % ChordMood::ALL.len();
                    self.transport_playback.set_chord_mood(ChordMood::ALL[mood_idx]);
                    let style_idx = VOICING_STYLE.load(Acquire) % VoicingStyle::ALL.len();
                    self.transport_playback.set_voicing_style(VoicingStyle::ALL[style_idx]);
                } else if mode == EngineMode::Orbitator {
                    let atmo_idx = ORBIT_ATMOSPHERE.load(Acquire) % OrbitAtmosphere::ALL.len();
                    self.transport_playback.set_orbit_atmosphere(OrbitAtmosphere::ALL[atmo_idx]);
                    let dyn_idx = ORBIT_DYNAMICS.load(Acquire) % OrbitDynamics::ALL.len();
                    self.transport_playback.set_orbit_dynamics(OrbitDynamics::ALL[dyn_idx]);
                }

                for pad in 0..16 {
                    let next = if enabled && preview & PAD_MASK & (1 << pad) != 0 {
                        PADS[pad].load(Acquire)
                    } else {
                        0
                    };
                    if self.inputs[pad] != next {
                        self.inputs[pad] = next;
                        let vel = if next != 0 {
                            Some(f32::from(Key::unpack(next, pad as u8).velocity) / f32::from(u16::MAX))
                        } else {
                            None
                        };
                        self.transport_playback.set_pad(pad as u8, vel);
                    }
                }

                let now = clock::now_ms().max(0) as u64 * 1000;
                let snapshot = transport_snapshot.unwrap();
                let (pulse, reset) =
                    self.transport_clock.observe(snapshot, now / 1000, bpm.unwrap());
                if reset {
                    g_arp_transport_resets.fetch_add(1, Relaxed);
                    let off = self.transport_playback.stop();
                    self.output(engine, off);
                }
                g_arp_transport_pulse.store(pulse, Release);
                if let Some(events) = self.transport_playback.tick(pulse) {
                    for event in events {
                        self.output(engine, event);
                    }
                }
                return;
            }

            // Free-running playback while transport is stopped (preview mode)
            let off = self.playback.set_mode(active_mode);
            self.output(engine, off);
            if active_mode == EngineMode::Off
                && let Some(key) = self.voice.take()
            {
                send(engine, key, false);
                kill_all_voices(engine);
            }
            if mode == EngineMode::Arp {
                let pattern_idx = PATTERN.load(Acquire) % Pattern::ALL.len();
                let pattern = Pattern::ALL[pattern_idx];
                if self.pattern != pattern {
                    self.playback.set_pattern(pattern);
                    self.pattern = pattern;
                }
                self.playback
                    .set_additional_octaves((ADDITIONAL_OCTAVES.load(Acquire) % 4) as u8);
            } else if mode == EngineMode::Melody {
                let bars_idx = MELODY_BARS.load(Acquire) % BarLength::ALL.len();
                self.playback.set_melody_bars(BarLength::ALL[bars_idx]);
                let var_idx = MELODY_VARIANT.load(Acquire) % MelodyVariant::ALL.len();
                self.playback.set_melody_variant(MelodyVariant::ALL[var_idx]);
            } else if mode == EngineMode::Mutator {
                let int_idx = MUTATOR_INTENSITY.load(Acquire) % MutationIntensity::ALL.len();
                self.playback
                    .set_mutator_intensity(MutationIntensity::ALL[int_idx]);
                self.playback
                    .set_mutator_frozen(MUTATOR_FROZEN.load(Acquire));
                let var_idx = MUTATOR_VARIANT.load(Acquire) % MelodyVariant::ALL.len();
                self.playback.set_mutator_variant(MelodyVariant::ALL[var_idx]);
            } else if mode == EngineMode::Acordator {
                let mood_idx = CHORD_MOOD.load(Acquire) % ChordMood::ALL.len();
                self.playback.set_chord_mood(ChordMood::ALL[mood_idx]);
                let style_idx = VOICING_STYLE.load(Acquire) % VoicingStyle::ALL.len();
                self.playback.set_voicing_style(VoicingStyle::ALL[style_idx]);
            } else if mode == EngineMode::Orbitator {
                let atmo_idx = ORBIT_ATMOSPHERE.load(Acquire) % OrbitAtmosphere::ALL.len();
                self.playback.set_orbit_atmosphere(OrbitAtmosphere::ALL[atmo_idx]);
                let dyn_idx = ORBIT_DYNAMICS.load(Acquire) % OrbitDynamics::ALL.len();
                self.playback.set_orbit_dynamics(OrbitDynamics::ALL[dyn_idx]);
            }
            let off = self.playback.set_rate(rate);
            self.output(engine, off);
            let mut pending = [(0, 0); 16];
            let mut count = 0;
            let mut releases = [None; 16];
            let mut release_count = 0;
            for pad in 0..16 {
                let next = if enabled && bpm.is_some() && preview & PAD_MASK & (1 << pad) != 0 {
                    PADS[pad].load(Acquire)
                } else {
                    0
                };
                if self.inputs[pad] == next {
                    continue;
                }
                if self.inputs[pad] != 0 {
                    if let Some(off) = self.playback.release(pad as u8) {
                        releases[release_count] = Some(off);
                        release_count += 1;
                    }
                }
                self.inputs[pad] = next;
                if next != 0 {
                    pending[count] = (pad, next);
                    count += 1;
                }
            }
            for off in &releases[..release_count] {
                self.output(engine, *off);
            }
            pending[..count].sort_unstable_by_key(|&(pad, bits)| Key::unpack(bits, pad as u8).serial);
            for &(pad, bits) in &pending[..count] {
                let off = self.playback.press(
                    pad as u8,
                    f32::from(Key::unpack(bits, pad as u8).velocity) / f32::from(u16::MAX),
                );
                if off.is_some() {
                    self.output(engine, off);
                }
            }
            let now = clock::now_ms().max(0) as u64 * 1000;
            for event in self.playback.tick(now, bpm) {
                self.output(engine, event);
            }
            if self.playback.selected() == 0
                && let Some(key) = self.voice.take()
            {
                send(engine, key, false);
                kill_all_voices(engine);
            }
        }
    }

    unsafe fn output(&mut self, engine: usize, event: Option<ScheduledEvent>) {
        // SAFETY: callers supply a live engine from this render callback.
        unsafe {
            match event {
                Some(ScheduledEvent::NoteOn(step)) => {
                    // Strict monophonic voice guarantee: close any sounding voice before opening a new one.
                    if let Some(old_key) = self.voice.take() {
                        send(engine, old_key, false);
                    }

                    let raw_bits = self.inputs[usize::from(step.pad)];
                    if raw_bits == 0 {
                        return;
                    }
                    let mut key = Key::unpack(raw_bits, step.pad);
                    if key.velocity == 0 {
                        return;
                    }

                    let active_pad = ACTIVE_SAMPLE_PAD.load(Acquire);
                    if active_pad < 16 {
                        key.pad = active_pad;
                        key.note = ACTIVE_SAMPLE_NOTE.load(Acquire);
                    }

                    // Protect against extreme pitch transposition. Firmware 16 Levels range is [-24, +24].
                    let total_pitch = i32::from(key.pitch) + i32::from(step.transpose);
                    key.pitch = total_pitch.clamp(-24, 24) as i8;

                    send(engine, key, true);
                    crate::diag::note_on(key.pad, key.note, key.pitch);
                    self.voice = Some(key);
                }
                Some(ScheduledEvent::NoteOff { pad }) => {
                    if let Some(key) = self.voice.take() {
                        send(engine, key, false);
                    } else if self.inputs[usize::from(pad)] != 0 {
                        let mut key = Key::unpack(self.inputs[usize::from(pad)], pad);
                        let active_pad = ACTIVE_SAMPLE_PAD.load(Acquire);
                        if active_pad < 16 {
                            key.pad = active_pad;
                            key.note = ACTIVE_SAMPLE_NOTE.load(Acquire);
                        }
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
struct PadPacket([u8; 128]);

unsafe fn send(engine: usize, key: Key, on: bool) {
    let mut packet = PadPacket([0; 128]);
    let tag: u32 = 3 | ((u32::from(key.pad) & 0xf) << 12);
    packet.0[8..12].copy_from_slice(&tag.to_le_bytes());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_preserves_pad_index_and_builds_firmware_tag() {
        for pad in 0..16_u8 {
            let note = 36 + pad;
            let pitch = -5_i8;
            let velocity = 50000_u16;
            let serial = 42_u32;
            let bits = (u64::from(serial) << 32)
                | (u64::from(velocity) << 16)
                | (u64::from(pitch as u8) << 8)
                | u64::from(note);

            let key = Key::unpack(bits, pad);
            assert_eq!(key.pad, pad);
            assert_eq!(key.note, note);
            assert_eq!(key.pitch, pitch);
            assert_eq!(key.velocity, velocity);
            assert_eq!(key.serial, serial);

            let tag: u32 = 3 | ((u32::from(key.pad) & 0xf) << 12);
            assert_eq!(tag & 0xff, 3);
            assert_eq!((tag >> 12) & 0xf, u32::from(pad));
        }
    }

    #[test]
    fn active_sample_pad_updates_and_resets() {
        set_active_sample_pad(0xff, 36);
        assert_eq!(active_sample_pad(), None);
        assert_eq!(active_sample_note(), 36);

        set_active_sample_pad(4, 40);
        assert_eq!(active_sample_pad(), Some(4));
        assert_eq!(active_sample_note(), 40);

        panic();
        assert_eq!(active_sample_pad(), None);
        assert_eq!(active_sample_note(), 36);
    }
}
