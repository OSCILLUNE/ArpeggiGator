#!/usr/bin/env python3
"""Emulator-only probe of the delayed PadEvent -> repaint -> on_paint path.

This script is for the experiment/repaint-dispatch-probe branch only. It sends
one deliberate generated note from on_paint using input/properties captured
250 ms earlier. Never use this experiment on a physical MPC Sample.
"""
import argparse
import json
from pathlib import Path
import struct
import subprocess
import sys
import time
import urllib.request


ROOT = Path(__file__).resolve().parents[1]
PANEL_DEFAULT = "http://127.0.0.1:8765"
PROBE_SIZE = 72
PROBE_FORMAT = "<QQQQiiIIIIIIII"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lab", type=Path, required=True,
                        help="path to the virtual-mpc-sample checkout")
    parser.add_argument("--panel", default=PANEL_DEFAULT)
    parser.add_argument("--hold-seconds", type=float, default=2.0)
    args = parser.parse_args()

    sys.path.insert(0, str(args.lab))
    import lab

    def ssh(command, timeout=30):
        return subprocess.check_output(
            ["ssh", *lab.ssh_base(), "root@" + lab.HOST, command],
            timeout=timeout,
        )

    def control(name, action, value=None):
        body = {"name": name, "action": action}
        if value is not None:
            body["value"] = value
        request = urllib.request.Request(
            args.panel + "/control",
            json.dumps(body).encode(),
            {"Content-Type": "application/json"},
        )
        with urllib.request.urlopen(request, timeout=5) as response:
            result = json.load(response)
        assert result.get("ok"), f"panel rejected {name} {action}: {result}"
        time.sleep(0.12)

    def tap(name):
        control(name, "press", 127)
        control(name, "release")

    def read_probe(pid, mod_base, symbol):
        address = mod_base + symbol
        data = ssh(
            f"dd if=/proc/{pid}/mem bs=1 skip={address} "
            f"count={PROBE_SIZE} 2>/dev/null"
        )
        assert len(data) == PROBE_SIZE, (
            f"could not read g_probe: received {len(data)} of {PROBE_SIZE} bytes"
        )
        values = struct.unpack(PROBE_FORMAT, data)
        keys = (
            "pad_thread", "paint_thread", "input_ptr", "properties_ptr",
            "source", "pad", "velocity_bits", "note_on_attempts",
            "note_on_succeeded", "note_off_attempts", "invalid_contexts",
            "worker_started", "worker_finished", "worker_timeouts",
        )
        return dict(zip(keys, values))

    pid_before = int(ssh('pidof "MPC Sample"').decode().strip().split()[0])
    maps = ssh(f"cat /proc/{pid_before}/maps").decode().splitlines()
    mod_mapping = next(
        (line for line in maps if "/run/arpeggigator.so" in line),
        None,
    )
    if mod_mapping is None:
        raise RuntimeError(
            "arpeggigator.so is not mapped; preload the experiment branch build first"
        )
    mod_base = int(mod_mapping.split()[0].split("-")[0], 16)

    elf = ROOT / "build" / "arpeggigator.elf"
    if not elf.is_file():
        raise RuntimeError(f"missing {elf}; run the experiment branch build first")
    symbols = {}
    for line in subprocess.check_output(["nm", "-a", str(elf)], text=True).splitlines():
        fields = line.split()
        if len(fields) >= 3:
            try:
                symbols[fields[-1]] = int(fields[0], 16)
            except ValueError:
                pass
    if "g_probe" not in symbols:
        raise RuntimeError("g_probe symbol not found in build/arpeggigator.elf")

    before = read_probe(pid_before, mod_base, symbols["g_probe"])

    # Start from the Sample page with 16 Levels selected. This experiment assumes
    # 16 Levels was OFF before this tap; use a fresh emulator app session.
    tap("SAMPLE")
    tap("16 LEVELS")

    # SHIFT+B1 is the agreed ArpeggiGator enable chord. B1 is SCREEN 1.
    control("SHIFT", "press")
    control("SCREEN 1", "press")
    control("SCREEN 1", "release")
    control("SHIFT", "release")

    # Keep a physical pad down long enough for the 250 ms delayed probe and its
    # 120 ms gate to complete. The original pad event is still forwarded.
    control("PAD 1", "press", 127)
    time.sleep(args.hold_seconds)
    control("PAD 1", "release")
    time.sleep(0.5)

    pid_after = int(ssh('pidof "MPC Sample"').decode().strip().split()[0])
    if pid_after != pid_before:
        raise RuntimeError(
            f"MPC Sample restarted during probe (PID {pid_before} -> {pid_after})"
        )

    after = read_probe(pid_after, mod_base, symbols["g_probe"])
    restarts = ssh("systemctl show ac50 -p NRestarts").decode().strip()

    delta_fields = (
        "note_on_attempts", "note_on_succeeded", "note_off_attempts",
        "invalid_contexts", "worker_started", "worker_finished", "worker_timeouts",
    )
    deltas = {name: after[name] - before[name] for name in delta_fields}
    same_thread = after["pad_thread"] == after["paint_thread"]
    pointers_valid = after["input_ptr"] != 0 and after["properties_ptr"] != 0

    checks = {
        "pad_and_paint_same_thread": same_thread,
        "captured_input_and_properties_nonzero": pointers_valid,
        "one_delayed_note_on_attempt": deltas["note_on_attempts"] == 1,
        "native_note_on_returned_true": deltas["note_on_succeeded"] == 1,
        "one_matching_note_off_attempt": deltas["note_off_attempts"] == 1,
        "no_invalid_context": deltas["invalid_contexts"] == 0,
        "worker_finished": deltas["worker_finished"] == 1,
        "no_worker_timeout": deltas["worker_timeouts"] == 0,
        "mpc_process_unchanged": pid_after == pid_before,
        "no_systemd_restart": restarts == "NRestarts=0",
    }

    print("ArpeggiGator repaint dispatch probe")
    print(f"pad thread:   {after['pad_thread']:#x}")
    print(f"paint thread: {after['paint_thread']:#x}")
    print(f"input ptr:    {after['input_ptr']:#x}")
    print(f"properties:   {after['properties_ptr']:#x}")
    print(f"source/pad:   {after['source']}/{after['pad']}")
    print(f"velocity:     {struct.unpack('<f', struct.pack('<I', after['velocity_bits']))[0]:.3f}")
    for name, passed in checks.items():
        print(f"{'PASS' if passed else 'FAIL'}: {name}")
    print(f"counter deltas: {deltas}")
    print(f"service: {restarts}")

    if not all(checks.values()):
        raise SystemExit(1)
    print("PASS: delayed note-on and matching note-off returned on the measured path")


if __name__ == "__main__":
    main()
