#!/bin/sh
# Build and package ArpeggiGator through the SDK's canonical build script.
set -eu
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
