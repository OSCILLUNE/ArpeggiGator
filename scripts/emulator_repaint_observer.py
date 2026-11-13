#!/usr/bin/env python3
"""Safe emulator-only measurement of repeated MPC repaint-to-paint delivery.

It requests 128 repaints at 4 ms intervals while a pad is held, then reads
callback timing diagnostics. It never invokes native note hooks and does not
test playback.
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
PROBE_SIZE = 96
PROBE_FORMAT = "<QQQiiIIIIIIi4xQQQI4x"
REPAINT_REQUESTS = 128
REPAINT_INTERVAL_US = 4_000
PROBE_KEYS = (
    "pad_thread", "paint_thread", "worker_thread", "source", "pad", "velocity_bits",
    "repaint_requests", "repaint_callbacks", "worker_started",
    "worker_finished", "worker_timeouts", "callback_latency_ms",
    "paint_interval_min_us", "paint_interval_max_us",
    "paint_interval_sum_us", "paint_interval_count",
)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lab", type=Path, required=True,
                        help="path to the virtual-mpc-sample checkout")
    parser.add_argument("--panel", default=PANEL_DEFAULT)
    parser.add_argument("--hold-seconds", type=float, default=2.0)
    args = parser.parse_args()

    if args.hold_seconds < 1.0:
        raise ValueError("--hold-seconds must be at least 1.0")

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
        return dict(zip(PROBE_KEYS, values))

    def read_thread_name(pid, tid):
        name = ssh(
            f"cat /proc/{pid}/task/{int(tid)}/comm 2>/dev/null || true"
        ).decode().strip()
        return name or "<thread exited>"

    pid_before = int(ssh('pidof "MPC Sample"').decode().strip().split()[0])
    maps = ssh(f"cat /proc/{pid_before}/maps").decode().splitlines()
    mod_mapping = next(
        (line for line in maps if "arpeggigator.so" in line),
        None,
    )
    if mod_mapping is None:
        raise RuntimeError(
            "arpeggigator.so is not mapped; preload the safe observer or install its package"
        )
    mod_base = int(mod_mapping.split()[0].split("-")[0], 16)

    elf = ROOT / "build" / "arpeggigator.elf"
    if not elf.is_file():
        raise RuntimeError(f"missing {elf}; run scripts/build.sh first")
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
    restarts_before = ssh("systemctl show ac50 -p NRestarts --value").decode().strip()

    # Start from a known page and enable the observer.
    tap("SAMPLE")
    tap("16 LEVELS")
    control("SHIFT", "press")
    control("SCREEN 1", "press")
    control("SCREEN 1", "release")
    control("SHIFT", "release")

    control("PAD 1", "press", 127)
    time.sleep(args.hold_seconds)
    control("PAD 1", "release")
    time.sleep(0.6)

    pid_after = int(ssh('pidof "MPC Sample"').decode().strip().split()[0])
    after = read_probe(pid_after, mod_base, symbols["g_probe"])
    restarts_after = ssh("systemctl show ac50 -p NRestarts --value").decode().strip()
    service = ssh("systemctl is-active ac50").decode().strip()

    delta_fields = (
        "repaint_requests", "repaint_callbacks", "worker_started",
        "worker_finished", "worker_timeouts",
    )
    deltas = {name: after[name] - before[name] for name in delta_fields}
    interval_count = after["paint_interval_count"]
    interval_mean_us = (
        after["paint_interval_sum_us"] / interval_count if interval_count else 0
    )

    checks = {
        "pad_press_started_one_worker": deltas["worker_started"] == 1,
        "bounded_repaint_burst_completed": deltas["repaint_requests"] == REPAINT_REQUESTS,
        "multiple_paint_callbacks_received": deltas["repaint_callbacks"] >= 2,
        "interpaint_interval_measured": interval_count >= 1,
        "worker_finished": deltas["worker_finished"] == 1,
        "no_handoff_timeout": deltas["worker_timeouts"] == 0,
        "mpc_process_unchanged": pid_after == pid_before,
        "ac50_service_active": service == "active",
        "no_new_systemd_restart": restarts_after == restarts_before,
    }

    same_thread = (
        after["pad_thread"] != 0
        and after["paint_thread"] != 0
        and after["pad_thread"] == after["paint_thread"]
    )
    worker_paint_same_thread = (
        after["worker_thread"] != 0
        and after["paint_thread"] != 0
        and after["worker_thread"] == after["paint_thread"]
    )
    thread_names = {
        "pad": read_thread_name(pid_after, after["pad_thread"]),
        "paint": read_thread_name(pid_after, after["paint_thread"]),
        "worker": read_thread_name(pid_after, after["worker_thread"]),
    }
    print("ArpeggiGator repaint cadence observer")
    print(f"pad thread ID/name:       {after['pad_thread']} / {thread_names['pad']}")
    print(f"paint thread ID/name:     {after['paint_thread']} / {thread_names['paint']}")
    print(f"worker thread ID/name:    {after['worker_thread']} / {thread_names['worker']}")
    print(f"pad/paint thread match:   {same_thread} (diagnostic only)")
    print(f"worker/paint thread match: {worker_paint_same_thread} (diagnostic only)")
    print(f"paint is process leader:   {after['paint_thread'] == pid_after} (diagnostic only)")
    print(f"source/pad:               {after['source']}/{after['pad']}")
    print(
        "velocity:                 "
        f"{struct.unpack('<f', struct.pack('<I', after['velocity_bits']))[0]:.3f}"
    )
    print(f"request cadence:          {REPAINT_INTERVAL_US / 1000:.1f} ms")
    print(f"paint callbacks:          {deltas['repaint_callbacks']}")
    print(f"first callback latency:   {after['callback_latency_ms']} ms")
    print(
        "paint intervals:          "
        f"count={interval_count}, min={after['paint_interval_min_us']} us, "
        f"mean={interval_mean_us:.1f} us, max={after['paint_interval_max_us']} us"
    )
    for name, passed in checks.items():
        print(f"{'PASS' if passed else 'FAIL'}: {name}")
    print(f"counter deltas: {deltas}")
    print(f"systemd restarts: {restarts_before} -> {restarts_after}")

    if not all(checks.values()):
        raise SystemExit(1)

    print(
        "PASS: repeated paint callbacks observed during the repaint burst. "
        "Thread equality is diagnostic only; pointer lifetime and note playback "
        "remain untested."
    )


if __name__ == "__main__":
    main()
