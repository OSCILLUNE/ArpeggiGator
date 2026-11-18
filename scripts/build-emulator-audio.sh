#!/bin/sh
# Compatibility wrapper. The feature was renamed: `emulator-audio` was a
# misleading name for the real note engine. Use scripts/build-native-audio.sh.
set -eu
exec sh "$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)/scripts/build-native-audio.sh" "$@"
