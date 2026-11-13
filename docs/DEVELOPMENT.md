# Development checklist

- [ ] Pin and document the exact MPC Sample SDK revision.
- [ ] Confirm firmware 1.3.0.12 fingerprint using a verified device build.
- [ ] Validate 16 Levels pad press/release event semantics.
- [ ] Validate generated MIDI note-on/note-off routing.
- [ ] Identify a documented transport/tempo clock source (no NOTE REPEAT dependency).
- [ ] Implement and unit-test the pattern engine.
- [ ] Implement note lifecycle cleanup (disable, release, stop, unload).
- [ ] Bind SHIFT+B1 and SHIFT+B2 only in 16 Levels.
- [ ] Add compact ARP status overlay.
- [ ] Build and test on physical MPC Sample.
- [ ] Publish installable release only after hardware validation.
