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

if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found. Install Rust with rustup or add ~/.cargo/bin to PATH." >&2
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
    --firmware-sha256 44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f \
    --display-name "ArpeggiGator"
