#!/usr/bin/env python3
"""Build an isolated, emulator-only one-pad MIDI proof of concept.

This deliberately never changes the checked-out SDK, the autorun card, or a
running MPC. The output is for the virtual lab only, not physical hardware.
"""
from pathlib import Path
import argparse
import os
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / 'build' / 'private-midi-poc'
SDK_REL = Path('vendor/mpc-sample-sdk')
parser = argparse.ArgumentParser()
parser.add_argument('--contention', action='store_true', help='emulator-only: block the UI for 5 seconds on SHIFT+B3')
args = parser.parse_args()


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise RuntimeError(f'Expected exactly one occurrence of {old!r}; SDK changed')
    return text.replace(old, new, 1)


with tempfile.TemporaryDirectory(prefix='arpeggigator-midi-poc-') as tmp:
    scratch = Path(tmp)
    for item in ('Cargo.toml', 'Cargo.lock'):
        shutil.copy2(ROOT / item, scratch / item)
    shutil.copytree(ROOT / 'src', scratch / 'src')
    shutil.copytree(ROOT / SDK_REL, scratch / SDK_REL)

    sdk = scratch / SDK_REL / 'src/midi.rs'
    data = sdk.read_text()
    data = replace_once(data, 'pub const REPLAY_SOURCE: u64 = 0x8000_0000;',
                        'pub const REPLAY_SOURCE: u64 = 0x7fff_fffa;')
    data = replace_once(data, '    note: u8,\n}',
                        '    note: u8,\n    velocity: [u8; 4],\n}')
    data = replace_once(data, '        note: 0xff,\n    };',
                        '        note: 0xff,\n        velocity: [0; 4],\n    };')
    data = replace_once(data, '    saved.mask = u16::from_le_bytes',
                        '    saved.velocity.copy_from_slice(&event[EVENT_VELOCITY..EVENT_VELOCITY + 4]);\n    saved.mask = u16::from_le_bytes')
    data = replace_once(data, '    event[EVENT_NOTE] = saved.note;',
                        '    event[EVENT_NOTE] = saved.note;\n    event[EVENT_VELOCITY..EVENT_VELOCITY + 4].copy_from_slice(&saved.velocity);')
    data = replace_once(data, '    /// The context the SDK forwards with.', '''    /// Emulator-only test: turn a borrowed marker into Note Off.
    pub fn test_replay_off(&mut self, pad: u8, row: i32, context: usize) -> bool {
        if !self.replay_as_pad(pad, row, context) { return false; }
        self.event[EVENT_VELOCITY..EVENT_VELOCITY + 4].copy_from_slice(&0_f32.to_le_bytes());
        true
    }

    /// The context the SDK forwards with.''')
    sdk.write_text(data)

    lib = scratch / 'src/lib.rs'
    data = lib.read_text()
    data = replace_once(data, 'static SHIFT_ACTIVE:', """#[unsafe(no_mangle)]
pub static g_poc_events: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
pub static g_poc_last: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
pub static g_poc_during_block: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
pub static g_poc_pad: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
pub static g_poc_pitch: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
pub static g_poc_pad_count: AtomicU32 = AtomicU32::new(0);
static POC_BLOCK_REQUEST: AtomicBool = AtomicBool::new(false);
static POC_BLOCK_ACTIVE: AtomicBool = AtomicBool::new(false);
static SHIFT_ACTIVE:""")
    data = replace_once(data, '#[derive(Default)]\nstruct ArpeggiGator', """fn poc_intercept(event: &mut mpc_sample_sdk::NoteEvent<'_>) -> Option<mpc_sample_sdk::NoteFlow> {
    if event.note_on {
        if let Some(pad) = event.physical_pad {
            g_poc_pad.store(pad.pad as u32, Ordering::Release);
            g_poc_pitch.store(pad.pitch.unwrap_or(i32::MIN) as u32, Ordering::Release);
            g_poc_pad_count.fetch_add(1, Ordering::AcqRel);
        }
    }
    // This identifier and mapping are only valid for the measured emulator.
    if event.source != 0x60ec_0937 || !event.note_on || (event.note != 36 && event.note != 35) {
        return None;
    }
    let is_off = event.note == 35;
    g_poc_events.fetch_add(1, Ordering::AcqRel);
    if POC_BLOCK_ACTIVE.load(Ordering::Acquire) {
        g_poc_during_block.fetch_add(1, Ordering::AcqRel);
    }
    g_poc_last.store(if is_off { 2 } else { 1 }, Ordering::Release);
    let ok = if is_off { event.test_replay_off(0, 0, 0) } else { event.replay_as_pad(0, 0, 0) };
    Some(if ok { mpc_sample_sdk::NoteFlow::Replayed } else { mpc_sample_sdk::NoteFlow::Forward })
}

#[derive(Default)]
struct ArpeggiGator""")
    data = replace_once(data, '    fn surface_observer(&self)', """    fn note_interceptor(&self) -> Option<mpc_sample_sdk::NoteInterceptor> {
        Some(poc_intercept)
    }

    fn surface_observer(&self)""")
    if args.contention:
        data = replace_once(data, '    fn on_button(&mut self, event: ButtonEvent) -> ButtonFlow {', """    fn on_button(&mut self, event: ButtonEvent) -> ButtonFlow {
        if event.code == BUTTON_B3 && event.pressed && event.shift {
            POC_BLOCK_REQUEST.store(true, Ordering::Release);
        }""")
        data = replace_once(data, '    fn on_paint(&mut self, graphics: &mut mpc_sample_sdk::draw::Graphics, event: PaintEvent) {', """    fn on_paint(&mut self, graphics: &mut mpc_sample_sdk::draw::Graphics, event: PaintEvent) {
        if POC_BLOCK_REQUEST.swap(false, Ordering::AcqRel) {
            POC_BLOCK_ACTIVE.store(true, Ordering::Release);
            std::thread::sleep(std::time::Duration::from_secs(5));
            POC_BLOCK_ACTIVE.store(false, Ordering::Release);
        }""")
    lib.write_text(data)
    OUTPUT.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    # The workspace can use a rustup directory override. Preserve its selected
    # toolchain when building the copied project from a temporary directory.
    if 'RUSTUP_TOOLCHAIN' not in env and shutil.which('rustup'):
        selected = subprocess.check_output(['rustup', 'show', 'active-toolchain'],
                                           cwd=ROOT, text=True).split()[0]
        env['RUSTUP_TOOLCHAIN'] = selected
    env.setdefault('CARGO_NET_OFFLINE', 'true')
    env.setdefault('ZIG_GLOBAL_CACHE_DIR', str(Path(tempfile.gettempdir()) / 'arpeggigator-zig-cache'))
    subprocess.run(['sh', str(scratch / SDK_REL / 'build-mod.sh'), '--manifest-path',
                    str(scratch / 'Cargo.toml'), '--package', 'arpeggigator', '--name',
                    'arpeggigator', '--output-dir', str(OUTPUT), '--require-symbol',
                    'g_probe', '--require-symbol', 'g_poc_events'], cwd=scratch, env=env, check=True)
    subprocess.run(['zig', 'cc', '-target', 'aarch64-linux-gnu.2.28', '-O2',
                    str(ROOT / 'experiments/private-midi/seq_probe.c'), '-o',
                    str(OUTPUT / 'seq_probe'), '-ldl'], env=env, check=True)
    print(f'Emulator-only proof of concept: {OUTPUT}')
