#!/bin/sh
# Build and package ArpeggiGator through the SDK's canonical build script.
set -eu

# Non-interactive shells launched through "sh" do not necessarily load rustup's
# PATH setup. Load it explicitly so cargo is available to the metadata step.
if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
elif [ -d "$HOME/.cargo/bin" ]; then
    PATH="$HOME/.cargo/bin:$PATH"
    export PATH
fi

# Homebrew-installed rustup does not necessarily install the cargo shim in
# ~/.cargo/bin. Ask rustup for the active toolchain's real cargo binary.
if ! command -v cargo >/dev/null 2>&1 && command -v rustup >/dev/null 2>&1; then
    RUSTUP_CARGO=$(rustup which cargo 2>/dev/null || true)
    if [ -n "$RUSTUP_CARGO" ] && [ -x "$RUSTUP_CARGO" ]; then
        PATH="$(dirname "$RUSTUP_CARGO"):$PATH"
        export PATH
    fi
fi

# Last-resort discovery for a toolchain installed by rustup but not selected
# as its active default. Prefer the project's previously tested Rust 1.97.1.
if ! command -v cargo >/dev/null 2>&1; then
    for CANDIDATE in "$HOME"/.rustup/toolchains/1.97.1-*/bin/cargo "$HOME"/.rustup/toolchains/stable-*/bin/cargo; do
        if [ -x "$CANDIDATE" ]; then
            PATH="$(dirname "$CANDIDATE"):$PATH"
            export PATH
            break
        fi
    done
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found. Install a Rust toolchain with rustup." >&2
    exit 127
fi
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

SDK_DIR=$(
    cargo metadata --format-version 1 --manifest-path Cargo.toml |
    python3 -c '
import json
import pathlib
import sys

data = json.load(sys.stdin)
sdk = next((p for p in data["packages"] if p["name"] == "mpc-sample-sdk"), None)
if sdk is None:
    raise SystemExit("mpc-sample-sdk dependency not found")
print(pathlib.Path(sdk["manifest_path"]).parent)
'
)

[ -x "$SDK_DIR/build-mod.sh" ] || {
    echo "SDK build-mod.sh not found: $SDK_DIR/build-mod.sh" >&2
    exit 1
}

exec "$SDK_DIR/build-mod.sh" \
    --manifest-path "$ROOT/Cargo.toml" \
    --package arpeggigator \
    --name arpeggigator \
    --output-dir "$ROOT/build" \
    --pack \
    --require-symbol g_probe \
    --firmware-sha256 44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f \
    --display-name "ArpeggiGator"
