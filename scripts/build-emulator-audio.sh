#!/bin/sh
# Build the opt-in audio experiment without packaging or installing it.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
MPC_MOD_FEATURES=emulator-audio exec sh vendor/mpc-sample-sdk/build-mod.sh \
    --manifest-path "$root/Cargo.toml" \
    --package arpeggigator --name arpeggigator-audio-lab \
    --output-dir "$root/build/audio-lab" \
    --require-symbol g_arp_audio_on --require-symbol g_arp_audio_off
