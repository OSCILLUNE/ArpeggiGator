# Build

Run from the repository root:

    sh scripts/build.sh

The wrapper resolves the SDK checkout from Cargo metadata and runs the SDK's canonical build-mod.sh with the MPC Sample 1.3.0.12 firmware hash guard.

The output is written to build/. Do not install the current prototype on a device: automatic note retriggering and tempo synchronization have not yet been implemented or validated.