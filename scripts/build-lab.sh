#!/bin/sh
# Build the emulator/QA lab artifact from the SAME source and SAME feature as
# the physical package. One version, two artifacts:
#   scripts/build.sh     -> build/arpeggigator.zip      (SD card / physical MPC)
#   scripts/build-lab.sh -> build/arpeggigator-lab.so   (lab.py preload on QA/DEV)
# This script intentionally does NOT pack a ZIP and does NOT sync a card.
set -eu
if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
elif [ -d "$HOME/.cargo/bin" ]; then
    PATH="$HOME/.cargo/bin:$PATH"
    export PATH
fi
if ! command -v cargo >/dev/null 2>&1 && command -v rustup >/dev/null 2>&1; then
    RUSTUP_CARGO=$(rustup which cargo 2>/dev/null || true)
    [ -z "$RUSTUP_CARGO" ] || PATH="$(dirname "$RUSTUP_CARGO"):$PATH"
    export PATH
fi
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
sdk=$(
    cargo metadata --format-version 1 --manifest-path Cargo.toml |
    python3 -c 'import json,pathlib,sys; d=json.load(sys.stdin); print(pathlib.Path(next(p for p in d["packages"] if p["name"]=="mpc-sample-sdk")["manifest_path"]).parent)'
)
exec env MPC_MOD_FEATURES=native-audio sh "$sdk/build-mod.sh" \
    --manifest-path "$root/Cargo.toml" \
    --package arpeggigator --name arpeggigator-lab \
    --output-dir "$root/build/lab" \
    --require-symbol g_probe --require-symbol GATOR_NATIVE_AUDIO
