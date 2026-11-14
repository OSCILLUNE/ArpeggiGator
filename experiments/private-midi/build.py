#!/usr/bin/env python3
"""Build an isolated, emulator-only one-pad MIDI proof of concept.

This deliberately never changes the checked-out SDK, the autorun card, or a
running MPC. The output is for the virtual lab only, not physical hardware.
"""
from pathlib import Path
import os
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / 'build' / 'private-midi-poc'
SDK_REL = Path('vendor/mpc-sample-sdk')


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
    data = replace_once(data, 'static SHIFT_ACTIVE:', '''#[unsafe(no_mangle)]
pub static g_poc_events: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
pub static g_poc_last: AtomicU32 = AtomicU32::new(0);
static SHIFT_ACTIVE:''')
    data = replace_once(data, '    fn surface_observer(&self)', '''    fn on_note(&mut self, event: &mut mpc_sample_sdk::NoteEvent) -> mpc_sample_sdk::NoteFlow {
        // Only the virtual lab's explicitly named proof-of-concept port.
        if event.source == 0x60ec_0937 && event.note_on && (event.note == 36 || event.note == 35) {
            let is_off = event.note == 35;
            g_poc_events.fetch_add(1, Ordering::AcqRel);
            g_poc_last.store(if is_off { 2 } else { 1 }, Ordering::Release);
            let ok = if is_off { event.test_replay_off(0, 0, 0) } else { event.replay_as_pad(0, 0, 0) };
            if ok { return mpc_sample_sdk::NoteFlow::Replayed; }
        }
        mpc_sample_sdk::NoteFlow::Forward
    }

    fn surface_observer(&self)''')
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
