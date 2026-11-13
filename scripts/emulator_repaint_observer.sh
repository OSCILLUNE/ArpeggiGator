#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
LAB="$HOME/Downloads/virtual-mpc-sample"

if [ ! -f "$LAB/lab.py" ]; then
    printf 'Emulator repository not found: %s\n' "$LAB" >&2
    exit 1
fi

sh "$ROOT/scripts/build.sh"
python3 "$LAB/lab.py" autorun "$ROOT/build/arpeggigator.zip"
python3 "$ROOT/scripts/emulator_repaint_observer.py" --lab "$LAB"
