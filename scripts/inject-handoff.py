#!/usr/bin/env python3
"""Add the pre-insertion handoff to the SD package without touching autorun/.

The handoff lives at the ZIP root, outside `autorun/`, so it does not affect
`autorun.check` (which covers only autorun.json + autorun.sh + the SDK salt).
Run AFTER scripts/build.sh and BEFORE scripts/verify-v070.py.
"""
import pathlib
import zipfile

root = pathlib.Path(__file__).resolve().parents[1]
package = root / "build/arpeggigator.zip"
handoff = root / "docs/HANDOFF-0.0.7.1-FINAL.md"
name = "HANDOFF-PRZED-WLOZENIEM-0.0.7.1.md"

with zipfile.ZipFile(package) as z:
    entries = [(info, z.read(info)) for info in z.infolist()]
names = {info.filename for info, _ in entries}
if any(n.startswith("autorun/") is False and n != name for n in names):
    keep = sorted(n for n in names if not n.startswith("autorun/") and n != name)
    raise SystemExit(f"unexpected non-autorun entries already present: {keep}")

temporary = package.with_suffix(".zip.handoff")
with zipfile.ZipFile(temporary, "w") as z:
    for info, data in entries:
        z.writestr(info, data)
    z.write(handoff, name)
temporary.replace(package)
print(f"injected {name} into {package}")
