#!/usr/bin/env python3
"""One test step for the 0.0.6 staged build: control + full-panel PNG + state.

Usage:
  python3 scripts/ag6_step.py LABEL '[{"name":"16 LEVELS","action":"press"}]'
  PANEL=http://127.0.0.1:8766 python3 scripts/ag6_step.py QA_LABEL '[]'

Evidence per step: _screen.png (whole emulator panel, 1200x950),
_state.json (GET /state), _control.txt (POST /control replies),
_service.txt (SSH: hash, NRestarts, diag tail). Nothing is committed.
"""
import argparse
import datetime
import json
import os
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
PANEL = os.environ.get("PANEL", "http://127.0.0.1:8765")
PORT = os.environ.get("PORT", "2022")
LAB = os.environ.get(
    "LAB", "/Users/fafi/Downloads/virtual-mpc-sample"
)
KEY = f"{LAB}/out/keys/id_ed25519"
CHROME = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"

ap = argparse.ArgumentParser()
ap.add_argument("label")
ap.add_argument("actions", nargs="?", default="[]")
args = ap.parse_args()

actions = json.loads(args.actions)
stamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
out = ROOT / "docs" / "logs" / "ag6" / f"{stamp}_{args.label}"
out.mkdir(parents=True, exist_ok=True)

replies = []
for action in actions:
    result = subprocess.run(
        ["curl", "-s", "-X", "POST", f"{PANEL}/control",
         "-H", "Content-Type: application/json", "-d", json.dumps(action)],
        capture_output=True, text=True, timeout=15,
    )
    replies.append({"action": action, "reply": result.stdout.strip(), "rc": result.returncode})
(out / "_control.txt").write_text(json.dumps(replies, indent=2) + "\n")

state = subprocess.run(
    ["curl", "-s", "--max-time", "10", f"{PANEL}/state"],
    capture_output=True, text=True, timeout=20,
)
try:
    parsed = json.loads(state.stdout)
    (out / "_state.json").write_text(json.dumps(parsed, indent=2) + "\n")
    summary = {
        "connected": parsed.get("connected"),
        "midi_counts": parsed.get("midi", {}).get("counts"),
        "rear_levels": parsed.get("rear", {}).get("levels"),
        "sdcard": parsed.get("sdcard"),
        "last_midi": parsed.get("midi", {}).get("log", [])[-3:],
    }
except Exception:
    (out / "_state.json").write_text(state.stdout)
    summary = {"raw": state.stdout[:200]}

shot = subprocess.run([
    CHROME, "--headless", "--disable-gpu", "--hide-scrollbars",
    "--window-size=1200,950", f"--screenshot={out / '_screen.png'}", PANEL,
], capture_output=True, text=True, timeout=60)

service = subprocess.run([
    "ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=5", "-i", KEY,
    "-p", PORT, "root@127.0.0.1",
    "sha256sum /run/arpeggigator.so 2>/dev/null; "
    "systemctl show ac50 -p NRestarts,MainPID,ActiveState,Environment; "
    "tail -3 /tmp/arpeggigator-6-events.log 2>/dev/null; "
    "journalctl -u ac50 --no-pager 2>/dev/null | grep -ciE 'double free|ABRT|SEGV|dumped'",
], capture_output=True, text=True, timeout=30)
(out / "_service.txt").write_text(service.stdout + service.stderr)

print(json.dumps({"step": str(out), "summary": summary,
                  "screen_rc": shot.returncode,
                  "service": service.stdout.strip().splitlines()[:8]}, indent=2))
