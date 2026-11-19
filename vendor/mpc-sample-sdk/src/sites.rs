//! The hook sites: the only code in the SDK that runs inside the app.
//!
//! Each site decodes its arguments into an event, calls the mod, and then either
//! forwards the app's own displaced function or swallows the event.  The
//! originals are stored here and never leave this module.

use core::{
    cell::Cell,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::draw::{self, Graphics, Repaint};
use crate::event::{
    self, ButtonEvent, ButtonFlow, CcEvent, CcFlow, EncoderEvent, EncoderFlow, NoteFlow,
    NoteInterceptor, PadAction, PadEvent, PadFlow, PaintEvent, SurfaceEvent, SurfaceFlow,
};
use crate::hook;
use crate::image;
use crate::loader::with_mod;
use crate::midi::{self, NoteEvent};
use crate::platform::{control, extract, page, sample, writer};

/// One site: its image offset and the handler to put there.
struct Site {
    offset: usize,
    handler: *const (),
}

/// Held by `note_on` and `note_off` so a mod can see them by name.
static NOTE_ON_OK: AtomicBool = AtomicBool::new(false);
/// Held by `note_off`.
static NOTE_OFF_OK: AtomicBool = AtomicBool::new(false);

/// Whether both note hooks are installed.
#[must_use]
pub fn notes_ready() -> bool {
    NOTE_ON_OK.load(Ordering::Acquire) && NOTE_OFF_OK.load(Ordering::Acquire)
}

/// The displaced function of every site, zero when it was not installed.
static mut ORIGINALS: [usize; 18] = [0; 18];

/// The displaced function for one site.
///
/// # Safety
///
/// `T` must be the exact signature of the hooked function.
unsafe fn original<T: Copy>(index: usize) -> Option<T> {
    // SAFETY: the table has 18 entries and `index` is a site number.
    let address = unsafe {
        core::ptr::addr_of!(ORIGINALS)
            .cast::<usize>()
            .add(index)
            .read()
    };
    // SAFETY: the caller names the signature of the displaced function.
    (address != 0).then(|| unsafe { core::mem::transmute_copy::<usize, T>(&address) })
}

/// Install every site and return whether any of them took.
///
/// # Safety
/// `base` must be the verified firmware image. Run once before app threads start.
pub unsafe fn install(base: usize) -> bool {
    let mut installed = 0;
    for (index, site) in table().iter().enumerate() {
        // SAFETY: every offset is from the tested image and every handler has
        // the site's signature.  This only prepares the trampoline; the site
        // stays untouched until `hook::commit`.
        let trampoline = unsafe { hook::prepare(base, site.offset, site.handler as usize) };
        // SAFETY: only the loader writes the table, before the app threads run.
        unsafe {
            core::ptr::addr_of_mut!(ORIGINALS)
                .cast::<usize>()
                .add(index)
                .write(trampoline.unwrap_or(0));
        }
        installed += usize::from(trampoline.is_some());
    }
    if installed == 0 {
        hook::abort();
        return false;
    }
    // Seal the code pages first, then publish every branch.  A failure here
    // restores any site already patched and leaves the app unmodified.
    let committed = hook::commit();
    if !committed && !hook::touched() {
        // SAFETY: no branch is live, so the trampoline table is not observable.
        unsafe { core::ptr::addr_of_mut!(ORIGINALS).write([0; 18]) };
        return false;
    }
    // A rollback failure can leave a branch live even though installation
    // reports failure.  Its handler still forwards through ORIGINALS, so finish
    // the bookkeeping and let the loader refuse the mod; the app keeps working.
    // SAFETY: the site table has this entry.
    if unsafe { original::<usize>(3) }.is_some() {
        NOTE_ON_OK.store(true, Ordering::Release);
    }
    // SAFETY: the site table has this entry.
    if unsafe { original::<usize>(5) }.is_some() {
        NOTE_OFF_OK.store(true, Ordering::Release);
    }
    // SAFETY: the trampolines are the app's own functions.
    unsafe {
        extract::record(
            original::<usize>(11).unwrap_or(0),
            original::<usize>(12).unwrap_or(0),
        );
        control::record(original::<usize>(15).unwrap_or(0));
    }
    install_paint(base);
    committed
}

/// The site table, in install order.
fn table() -> [Site; 18] {
    [
        Site {
            offset: 0x0154_5a50,
            handler: sixteen_ctor as *const (),
        },
        Site {
            offset: 0x0214_72f0,
            handler: sixteen_proc as *const (),
        },
        Site {
            offset: 0x0140_dbc0,
            handler: data as *const (),
        },
        Site {
            offset: 0x01e2_7360,
            handler: note_on as *const (),
        },
        Site {
            offset: 0x023a_ff60,
            handler: surface as *const (),
        },
        Site {
            offset: 0x01e2_70d0,
            handler: note_off as *const (),
        },
        Site {
            offset: 0x0159_f230,
            handler: cc as *const (),
        },
        Site {
            offset: 0x027e_4400,
            handler: pitch as *const (),
        },
        Site {
            offset: 0x0159_f2f0,
            handler: buttons as *const (),
        },
        Site {
            offset: 0x0114_9880,
            handler: repaint as *const (),
        },
        Site {
            offset: 0x00f5_35c0,
            handler: str_assign as *const (),
        },
        Site {
            offset: 0x0143_ed40,
            handler: extract::ctor_entry as *const (),
        },
        Site {
            offset: 0x0143_f0b0,
            handler: extract::exec_entry as *const (),
        },
        Site {
            offset: 0x01d8_aff0,
            handler: writer_entry as *const (),
        },
        Site {
            offset: 0x0211_b090,
            handler: note as *const (),
        },
        Site {
            offset: 0x0196_8020,
            handler: control_entry as *const (),
        },
        Site {
            offset: 0x01df_3330,
            handler: sample_service_ctor as *const (),
        },
        Site {
            offset: 0x0159_f910,
            handler: lighting as *const (),
        },
    ]
}

/// The AC50 per-pad lighting callback: apply the mod's override, then forward.
///
/// The callback receives the sender, a pointer to the native `PadLight`
/// (`RGB | brightness_float << 32`) and the zero-based pad.
///
/// # Safety
///
/// The app calls this with its own sender, `PadLight` and pad index.
unsafe extern "C" fn lighting(sender: usize, light: *mut u64, pad: u8) {
    let native = if light.is_null() {
        0
    } else {
        // SAFETY: the app passes its live PadLight for this callback.
        unsafe { *light }
    };
    crate::platform::lighting::observe(sender, pad, native);
    if !light.is_null()
        && let Some(lit) = crate::platform::lighting::override_light(pad)
    {
        // SAFETY: the callback owns this temporary PadLight.
        unsafe { *light = lit };
    }
    // SAFETY: site 17 is the app's lighting callback; forward its arguments.
    if let Some(forward) = unsafe { original::<unsafe extern "C" fn(usize, *mut u64, u8)>(17) } {
        // SAFETY: forwards the app's callback arguments unchanged.
        unsafe { forward(sender, light, pad) };
    }
}

/// Send one already-packed `PadLight` through the original callback.
pub(crate) fn send_light(sender: usize, light: u64, pad: u8) -> bool {
    // SAFETY: site 17 is the installed lighting callback.
    let Some(forward) = (unsafe { original::<unsafe extern "C" fn(usize, *mut u64, u8)>(17) })
    else {
        return false;
    };
    let mut light = light;
    // SAFETY: the original callback converts and sends this live `PadLight`.
    unsafe { forward(sender, core::ptr::from_mut(&mut light), pad) };
    true
}

unsafe extern "C" fn sample_service_ctor(
    service: usize,
    first: usize,
    second: usize,
    third: usize,
    fourth: usize,
) {
    // SAFETY: site 16 is this constructor and the app supplies its arguments.
    let forward =
        unsafe { original::<unsafe extern "C" fn(usize, usize, usize, usize, usize)>(16) };
    if let Some(forward) = forward {
        // SAFETY: this forwards the original constructor arguments unchanged.
        unsafe { forward(service, first, second, third, fourth) };
        sample::remember_service(service);
    }
}

/// The app's 16 Levels view constructor: pass it through.
///
/// # Safety
///
/// The app calls this with its own arguments.
unsafe extern "C" fn sixteen_ctor(a: usize, b: usize, c: usize, d: usize) -> usize {
    // SAFETY: the original is the app's own constructor.
    unsafe {
        original::<unsafe extern "C" fn(usize, usize, usize, usize) -> usize>(0)
            .map_or(0, |forward| forward(a, b, c, d))
    }
}

/// The app's 16 Levels note handler: pass it through.
///
/// # Safety
///
/// The app calls this with its own arguments.
unsafe extern "C" fn sixteen_proc(app: usize, index: i32, out: usize) {
    // SAFETY: the original is the app's own handler.
    unsafe {
        if let Some(forward) = original::<unsafe extern "C" fn(usize, i32, usize)>(1) {
            forward(app, index, out);
        }
    }
}

/// The encoder: let the mod decide, then forward.
///
/// # Safety
///
/// The app calls this with its own surface and delta.
unsafe extern "C" fn data(surface: usize, delta: i32) {
    let event = EncoderEvent { surface, delta };
    let flow = with_mod(|module| module.on_encoder(event)).unwrap_or(EncoderFlow::Forward);
    if flow == EncoderFlow::Forward {
        // SAFETY: the original is the app's own handler.
        unsafe {
            if let Some(forward) = original::<unsafe extern "C" fn(usize, i32)>(2) {
                forward(surface, delta);
            }
        }
    }
}

// Only scalar pad information is kept, and only while the app's current
// synchronous pad-on call is active on this thread. Nested calls restore the
// previous scope; no input, properties, screen, or engine pointers are saved.
thread_local! {
    static CURRENT_PHYSICAL_PAD: Cell<Option<midi::PhysicalPad>> = const { Cell::new(None) };
}

struct PhysicalPadScope(Option<midi::PhysicalPad>);

impl PhysicalPadScope {
    fn enter(pad: i32, velocity: f32) -> Self {
        let previous = CURRENT_PHYSICAL_PAD.replace(Some(midi::PhysicalPad {
            pad,
            velocity,
            pitch: None,
        }));
        Self(previous)
    }
}

impl Drop for PhysicalPadScope {
    fn drop(&mut self) {
        CURRENT_PHYSICAL_PAD.set(self.0);
    }
}

fn remember_physical_pitch(pitch: i32) {
    CURRENT_PHYSICAL_PAD.with(|slot| {
        slot.set(slot.get().map(|mut pad| {
            pad.pitch = Some(pitch);
            pad
        }));
    });
}

/// A pad press: let the mod decide, then forward.
///
/// # Safety
///
/// The app calls this with its own input object.
unsafe extern "C" fn note_on(
    input: usize,
    source: i32,
    pad: i32,
    properties: usize,
    velocity: f32,
) {
    let event = PadEvent {
        action: PadAction::Press,
        input,
        source,
        pad,
        properties,
        velocity,
    };
    let flow = with_mod(|module| module.on_pad(event)).unwrap_or(PadFlow::Forward);
    if flow == PadFlow::Forward {
        // SAFETY: the original is the app's own handler.
        unsafe {
            if let Some(forward) = original::<unsafe extern "C" fn(usize, i32, i32, usize, f32)>(3)
            {
                let _scope = crate::loader::note_interceptor()
                    .map(|_| PhysicalPadScope::enter(pad, velocity));
                forward(input, source, pad, properties, velocity);
            }
        }
    }
}

/// A control-surface event: let the mod decide, then forward.
///
/// # Safety
///
/// The app calls this with its own surface.
unsafe extern "C" fn surface(surface: usize, code: i32, payload: usize) {
    let mut raw = [0_u8; 3];
    let decoded = payload != 0 && image::read(payload, &mut raw);
    let event = SurfaceEvent {
        surface,
        code,
        payload,
        status: if decoded { raw[0] } else { 0 },
        kind: if decoded { raw[0] & 0xf0 } else { 0 },
        note: if decoded { raw[1] } else { 0 },
        velocity: if decoded { raw[2] } else { 0 },
    };
    crate::loader::observe_surface(event);
    let flow = with_mod(|module| module.on_surface(event)).unwrap_or(SurfaceFlow::Forward);
    if flow == SurfaceFlow::Forward {
        // SAFETY: the original is the app's own handler.
        unsafe {
            if let Some(forward) = original::<unsafe extern "C" fn(usize, i32, usize)>(4) {
                forward(surface, code, payload);
            }
        }
    }
}

/// A pad release: let the mod decide, then forward.
///
/// # Safety
///
/// The app calls this with its own input object.
unsafe extern "C" fn note_off(input: usize, source: i32, pad: i32) {
    let event = PadEvent {
        action: PadAction::Release,
        input,
        source,
        pad,
        properties: 0,
        velocity: 0.0,
    };
    let flow = with_mod(|module| module.on_pad(event)).unwrap_or(PadFlow::Forward);
    if flow == PadFlow::Forward {
        // SAFETY: the original is the app's own handler.
        unsafe {
            if let Some(forward) = original::<unsafe extern "C" fn(usize, i32, i32)>(5) {
                forward(input, source, pad);
            }
        }
    }
}

/// A control change: let the mod decide, then forward.
///
/// # Safety
///
/// The app calls this with its own event.
unsafe extern "C" fn cc(event: usize, raw: u32, low: f32, range: f32) {
    let control = if event == 0 {
        0xffff
    } else {
        // SAFETY: the app's event keeps its control byte at +8.
        u32::from(unsafe { *(event as *const u8).add(8) })
    };
    let message = CcEvent {
        event,
        control,
        value: raw,
        low,
        range,
    };
    let flow = with_mod(|module| module.on_cc(message)).unwrap_or(CcFlow::Forward);
    if flow == CcFlow::Forward {
        // SAFETY: the original is the app's own handler.
        unsafe {
            if let Some(forward) = original::<unsafe extern "C" fn(usize, u32, f32, f32)>(6) {
                forward(event, raw, low, range);
            }
        }
    }
}

/// The 16 Levels pitch setter: let the mod shift the applied offset.
///
/// # Safety
///
/// The app calls this with its own arguments.
unsafe extern "C" fn pitch(app: usize, a: u64, b: u64, applied: i32) {
    let offset = with_mod(|module| module.on_pitch(applied)).unwrap_or(applied);
    if crate::loader::note_interceptor().is_some() {
        remember_physical_pitch(offset);
    }
    // SAFETY: the original is the app's own setter.
    unsafe {
        if let Some(forward) = original::<unsafe extern "C" fn(usize, u64, u64, i32)>(7) {
            forward(app, a, b, offset);
        }
    }
}

/// A button: let the mod decide, then forward.
///
/// # Safety
///
/// The app calls this with its own event object.
unsafe extern "C" fn buttons(event: usize) -> i32 {
    let code = if event == 0 {
        0
    } else {
        // SAFETY: the app's event keeps its key code at +8.
        u32::from(unsafe { *(event as *const u8).add(8) })
    };
    let value = if event == 0 {
        0
    } else {
        // SAFETY: and its value at +9.
        u32::from(unsafe { *(event as *const u8).add(9) })
    };
    let pressed = value == event::button::PRESSED;
    // The event's first word is its context; `+0x10` there is the surface.
    let surface = if event == 0 {
        0
    } else {
        image::read_pointer(event)
            .and_then(|context| image::read_pointer(context.wrapping_add(0x10)))
            .unwrap_or(0)
    };
    let mut bytes = [0_u8; 16];
    if event != 0 {
        // SAFETY: the app's button event is at least this long.
        unsafe { core::ptr::copy_nonoverlapping(event as *const u8, bytes.as_mut_ptr(), 16) };
    }
    let message = ButtonEvent {
        event,
        bytes,
        code,
        value,
        pressed,
        shift: event::shift(),
        erase: event::erase(),
        surface,
    };
    let flow = with_mod(|module| module.on_button(message)).unwrap_or(ButtonFlow::Forward);
    // Track the hold state after the mod has seen the event, so it sees the
    // state that was in effect when the key was pressed.
    if code == event::button::SHIFT {
        event::set_shift(pressed);
    }
    if code == event::button::ERASE {
        event::set_erase(pressed);
    }
    if flow == ButtonFlow::Forward {
        // SAFETY: the original is the app's own handler.
        unsafe {
            if let Some(forward) = original::<unsafe extern "C" fn(usize)>(8) {
                forward(event);
            }
        }
    }
    0
}

/// A repaint: pass it through.
///
/// # Safety
///
/// The app calls this with its own component and rectangles.
unsafe extern "C" fn repaint(component: usize, a: u64, b: u64) {
    // SAFETY: the original is the app's own repaint.
    unsafe {
        if let Some(forward) = original::<Repaint>(9) {
            forward(component, a, b);
        }
    }
}

/// A label assignment: let the mod rename it, then forward.
///
/// # Safety
///
/// `text` must be a NUL-terminated string.
unsafe extern "C" fn str_assign(dest: usize, text: *const u8) {
    let mut chosen = text;
    if !text.is_null() {
        // SAFETY: the caller promises a NUL-terminated string.
        let label = unsafe { core::ffi::CStr::from_ptr(text.cast()) };
        if let Some(replacement) = with_mod(|module| module.on_label(label)).flatten() {
            chosen = replacement.as_ptr().cast();
        }
    }
    // SAFETY: the original is the app's own assignment.
    unsafe {
        if let Some(forward) = original::<unsafe extern "C" fn(usize, *const u8)>(10) {
            forward(dest, chosen);
        }
    }
}

/// The app's writer: hand the block to the substitute.
///
/// # Safety
///
/// The app calls this with its own block.
unsafe extern "C" fn writer_entry(closure: usize, block: *mut u8) {
    // SAFETY: the original is the app's own writer.
    let forward = unsafe { original::<writer::Writer>(13) };
    // SAFETY: the caller passed the app's arguments.
    unsafe { writer::substitute(closure, block, forward) };
}

/// A MIDI note: let the mod decide, then forward (and restore).
///
/// # Safety
///
/// The app calls this with its own event.
unsafe extern "C" fn note(app: usize, context: usize, event: *mut u8, source: usize) {
    // SAFETY: the original is the app's own setter, when the site is installed.
    let pitch_off = unsafe { original::<midi::PitchOffset>(7) };
    // The event is the app's writable object for the duration of this hook.
    let storage: &mut [u8] = if event.is_null() {
        &mut []
    } else {
        // SAFETY: the app promises exclusive writability for one event.
        unsafe { core::slice::from_raw_parts_mut(event, midi::EVENT_BYTES) }
    };
    // SAFETY: the site supplies the app's own displaced pitch setter.
    let mut message = unsafe { NoteEvent::new(app, context, storage, source as u64, pitch_off) };
    let interceptor = crate::loader::note_interceptor();
    if interceptor.is_some() {
        message.physical_pad = CURRENT_PHYSICAL_PAD.get();
    }
    let flow = choose_note_flow(&mut message, interceptor, |message| {
        with_mod(|module| module.on_note(message)).unwrap_or(NoteFlow::Forward)
    });
    match flow {
        NoteFlow::Forward => {
            message.restore();
            // SAFETY: the original is the app's own dispatcher.
            unsafe {
                if let Some(forward) =
                    original::<unsafe extern "C" fn(usize, usize, usize, usize)>(14)
                {
                    forward(app, context, message.pointer() as usize, source);
                }
            }
        }
        NoteFlow::Replayed => {
            // SAFETY: the original is the app's own dispatcher.
            unsafe {
                if let Some(forward) =
                    original::<unsafe extern "C" fn(usize, usize, usize, usize)>(14)
                {
                    forward(
                        app,
                        message.forward_context(),
                        message.pointer() as usize,
                        usize::try_from(message.forward_source()).unwrap_or(usize::MAX),
                    );
                }
            }
            message.restore();
        }
        NoteFlow::Consume => message.restore(),
    }
}

/// The control-surface setter: learn the surface and pass the call on.
///
/// # Safety
///
/// The app calls this with its own objects.
unsafe extern "C" fn control_entry(
    control: usize,
    surface: usize,
    id: i32,
    op: i32,
    value: f32,
) -> usize {
    // SAFETY: the app's own objects.
    unsafe { control::hook(control, surface, id, op, value) }
}

/// The app's paint, displaced by the SDK.
static mut ORIGINAL_PAINT: Option<unsafe extern "C" fn(usize, usize)> = None;

/// The page's paint: let the app draw, then the mod.
///
/// # Safety
///
/// The app calls this with its own component and graphics.
unsafe extern "C" fn page_paint(component: usize, graphics: usize) {
    // SAFETY: the original is the app's own paint.
    unsafe {
        if let Some(original) = *core::ptr::addr_of!(ORIGINAL_PAINT) {
            original(component, graphics);
        }
    }
    draw::set_painted(component);
    let sample_page = page::is_sample_page(component);
    if sample_page {
        page::set_screen(component);
    }
    // SAFETY: the app is painting this live component on the UI thread.
    let chop_mode = sample_page && unsafe { page::chop_mode(component) };
    let event = PaintEvent {
        component,
        graphics,
        sample_page,
        chop_mode,
    };
    let mut handle = Graphics::new(graphics);
    with_mod(|module| module.on_paint(&mut handle, event));
}

// Try the real-time path first; it never accesses the module mutex.
fn choose_note_flow(
    message: &mut NoteEvent<'_>,
    interceptor: Option<NoteInterceptor>,
    fallback: impl FnOnce(&mut NoteEvent<'_>) -> NoteFlow,
) -> NoteFlow {
    interceptor
        .and_then(|handle| handle(message))
        .unwrap_or_else(|| fallback(message))
}

/// Install the page paint over the app's vtable slot.
fn install_paint(base: usize) -> bool {
    let address = base + 0x6e2_12a0;
    // SAFETY: the pointer is only compared, never dereferenced.
    if unsafe { !image::mapped(address as *const u8) } {
        return false;
    }
    // SAFETY: the address is inside the app's screen vtable.
    let slot = unsafe { (address as *mut usize).add(27) };
    // SAFETY: see above.
    let original = unsafe { *slot };
    if original == 0 {
        return false;
    }
    // SAFETY: the original is the app's own paint.
    unsafe {
        ORIGINAL_PAINT =
            core::mem::transmute::<usize, Option<unsafe extern "C" fn(usize, usize)>>(original);
    }
    let page = address & !0xfff;
    // SAFETY: the page is inside the app's image.
    let unlocked = unsafe {
        libc::mprotect(
            page as *mut libc::c_void,
            0x1000,
            libc::PROT_READ | libc::PROT_WRITE,
        )
    } == 0;
    if !unlocked {
        return false;
    }
    // SAFETY: the page is writable now.
    unsafe {
        *slot = page_paint as *const () as usize;
        libc::mprotect(page as *mut libc::c_void, 0x1000, libc::PROT_READ);
    }
    true
}

/// Displaced native dispatcher and pitch setter, when both hooks are installed.
///
/// The addresses are firmware functions. Calling them requires a live engine
/// or property object and the owning firmware context.
#[must_use]
pub fn note_trampolines() -> Option<(usize, usize)> {
    // SAFETY: both indices refer to immutable entries after SDK installation.
    unsafe { Some((original::<usize>(14)?, original::<usize>(7)?)) }
}

/// The app's own repaint, for a mod that needs to call it directly.
#[must_use]
pub fn repaint_function() -> Option<Repaint> {
    // SAFETY: the original is the app's own repaint.
    unsafe { original::<Repaint>(9) }
}

/// Ask the app to repaint one component.
///
/// # Safety
///
/// `component` must be a live component on its owning UI thread.
pub unsafe fn repaint_component(component: usize) {
    // SAFETY: the original is the app's own repaint.
    if let Some(forward) = unsafe { original::<Repaint>(9) }
        && component != 0
    {
        // SAFETY: the caller guarantees the live component and the app's own
        // repaint is the displaced entry.
        unsafe { forward(component, 0, (2000_u64 << 32) | 0x7d0) };
    }
}

/// The app's own button dispatcher, for a mod that replays an event.
#[must_use]
pub fn buttons_original() -> Option<unsafe extern "C" fn(usize)> {
    // SAFETY: written once at install.
    unsafe { original::<unsafe extern "C" fn(usize)>(8) }
}

/// Replay one key press/release on a captured button event.
///
/// Returns false when no event was captured or the button hook is missing.
///
/// # Safety
///
/// `event` must be the first 16 bytes of a button event captured during the
/// current callback, whose context and surface are still live.  Replaying it
/// later would dereference a stale app object.
#[must_use]
pub unsafe fn replay_button(event: &[u8; 16], key: u8) -> bool {
    let Some(original) = buttons_original() else {
        return false;
    };
    // A blank event has no context pointer; replaying it would dereference zero.
    if event[..8] == [0; 8] {
        return false;
    }
    let mut bytes = *event;
    bytes[8] = key;
    bytes[9] = 0x7f;
    // SAFETY: the captured event is the app's own and the original is its own
    // dispatcher.
    unsafe { original(bytes.as_ptr() as usize) };
    bytes[9] = 0;
    // SAFETY: as above.
    unsafe { original(bytes.as_ptr() as usize) };
    true
}

/// Send a note-on through the app's own input path.
///
/// Returns false when the note hook is not installed.
///
/// This bypasses the mod's pad policy and is intended for a controlled internal
/// selection, followed by [`send_note_off`].
///
/// # Safety
///
/// `input` and `properties` must be the app's live input objects on their owning
/// UI thread, and `source`/`pad`/`velocity` must be valid for the firmware.
#[must_use]
pub unsafe fn send_note_on(
    input: usize,
    source: i32,
    pad: i32,
    properties: usize,
    velocity: f32,
) -> bool {
    // SAFETY: the original is the app's own note-on handler.
    if let Some(forward) =
        // SAFETY: the pointer is the app's displaced note-on entry.
        unsafe { original::<unsafe extern "C" fn(usize, i32, i32, usize, f32)>(3) }
    {
        // SAFETY: the app's own call.
        unsafe { forward(input, source, pad, properties, velocity) };
        true
    } else {
        false
    }
}

/// Send a note-off through the app's own input path.
///
/// # Safety
///
/// `input` must be the app's live input object on its owning UI thread, and
/// `source`/`pad` must identify the note-on this releases.
pub unsafe fn send_note_off(input: usize, source: i32, pad: i32) {
    // SAFETY: the original is the app's own handler.
    if let Some(forward) = unsafe { original::<unsafe extern "C" fn(usize, i32, i32)>(5) } {
        // SAFETY: the app's own call.
        unsafe { forward(input, source, pad) };
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CURRENT_PHYSICAL_PAD, PhysicalPadScope, choose_note_flow, notes_ready,
        remember_physical_pitch, table,
    };
    use crate::{NoteEvent, NoteFlow};

    fn consume(_event: &mut NoteEvent<'_>) -> Option<NoteFlow> {
        Some(NoteFlow::Consume)
    }

    fn ignore(_event: &mut NoteEvent<'_>) -> Option<NoteFlow> {
        None
    }

    #[test]
    fn physical_pad_scope_tracks_pitch_and_restores_nested_context() {
        assert!(CURRENT_PHYSICAL_PAD.get().is_none());
        {
            let _outer = PhysicalPadScope::enter(3, 0.75);
            remember_physical_pitch(-4);
            let outer = CURRENT_PHYSICAL_PAD.get().unwrap();
            assert_eq!(
                (outer.pad, outer.velocity, outer.pitch),
                (3, 0.75, Some(-4))
            );
            {
                let _inner = PhysicalPadScope::enter(8, 0.5);
                remember_physical_pitch(6);
                let inner = CURRENT_PHYSICAL_PAD.get().unwrap();
                assert_eq!((inner.pad, inner.velocity, inner.pitch), (8, 0.5, Some(6)));
            }
            assert_eq!(CURRENT_PHYSICAL_PAD.get(), Some(outer));
        }
        assert!(CURRENT_PHYSICAL_PAD.get().is_none());
    }

    #[test]
    fn physical_pad_scope_is_thread_local_and_ignores_pitch_outside_scope() {
        remember_physical_pitch(7);
        assert!(CURRENT_PHYSICAL_PAD.get().is_none());
        let _scope = PhysicalPadScope::enter(4, 0.5);
        std::thread::spawn(|| {
            assert!(CURRENT_PHYSICAL_PAD.get().is_none());
            remember_physical_pitch(-2);
            assert!(CURRENT_PHYSICAL_PAD.get().is_none());
            let _scope = PhysicalPadScope::enter(9, 1.0);
            remember_physical_pitch(3);
            assert_eq!(CURRENT_PHYSICAL_PAD.get().unwrap().pitch, Some(3));
        })
        .join()
        .unwrap();
        let pad = CURRENT_PHYSICAL_PAD.get().unwrap();
        assert_eq!((pad.pad, pad.velocity, pad.pitch), (4, 0.5, None));
    }

    #[test]
    fn realtime_note_interceptor_runs_before_fallback_without_a_module_lock() {
        let mut storage = [0_u8; 64];
        // SAFETY: the storage is exclusively borrowed and no setter is used.
        let mut event = unsafe { NoteEvent::new(0, 0, &mut storage, 0, None) };
        let lock = std::sync::Mutex::new(());
        let guard = lock.lock().unwrap();
        let flow = choose_note_flow(&mut event, Some(consume), |_| {
            assert!(lock.try_lock().is_ok(), "the fallback must not run");
            NoteFlow::Forward
        });
        assert_eq!(flow, NoteFlow::Consume);
        drop(guard);
        assert_eq!(
            choose_note_flow(&mut event, Some(ignore), |_| NoteFlow::Forward),
            NoteFlow::Forward
        );
        assert_eq!(
            choose_note_flow(&mut event, None, |_| NoteFlow::Consume),
            NoteFlow::Consume
        );
    }

    #[test]
    fn the_table_has_one_entry_per_site() {
        assert_eq!(table().len(), 18);
        assert!(!notes_ready());
    }
}
