#!/bin/sh
# Build the native-audio engine as a standalone .so for A/B experiments.
# This is NOT the package build: scripts/build.sh produces the SD card ZIP.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
MPC_MOD_FEATURES=native-audio exec sh vendor/mpc-sample-sdk/build-mod.sh \
    --manifest-path "$root/Cargo.toml" \
    --package arpeggigator --name arpeggigator-native-lab \
    --output-dir "$root/build/native-lab" \
    --require-symbol g_arp_audio_on --require-symbol g_arp_audio_off \
    --require-symbol GATOR_NATIVE_AUDIO
