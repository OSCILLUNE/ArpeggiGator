#!/usr/bin/env python3
"""Update only ArpeggiGator's autorun library on the running emulator card."""
import hashlib
import importlib.util
from pathlib import Path
import subprocess
import sys
import zipfile

root = Path(__file__).resolve().parents[1]
lab_path = Path.home() / 'Downloads/virtual-mpc-sample/lab.py'
if not lab_path.is_file():
    print('Emulator card not updated: virtual MPC lab not found.', file=sys.stderr)
    sys.exit(1)
spec = importlib.util.spec_from_file_location('mpc_lab', lab_path)
lab = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lab)
package = root / 'build/arpeggigator.zip'
with zipfile.ZipFile(package) as archive:
    library = archive.read('autorun/arpeggigator.so')
    script = archive.read('autorun/autorun.sh')
digest = hashlib.sha256(library).hexdigest()
script_digest = hashlib.sha256(script).hexdigest()
ssh = ['ssh', *lab.ssh_base(), 'root@' + lab.HOST]
# Require the expected mounted card and the exact generated autorun script.
# The staging file is in the same directory, so rename is atomic on the card.
command = f'''set -eu
D=/media/MPCSAMPLE/autorun
mountpoint -q /media/MPCSAMPLE
[ -f "$D/arpeggigator.so" ] && [ -f "$D/autorun.sh" ]
[ "$(sha256sum "$D/autorun.sh" | cut -d' ' -f1)" = "{script_digest}" ]
cat > "$D/.arpeggigator.so.new"
[ "$(sha256sum "$D/.arpeggigator.so.new" | cut -d' ' -f1)" = "{digest}" ]
chmod 755 "$D/.arpeggigator.so.new"
mv -f "$D/.arpeggigator.so.new" "$D/arpeggigator.so"
sync
sha256sum "$D/arpeggigator.so"
'''
try:
    result = subprocess.run(ssh + [command], input=library, capture_output=True, timeout=30)
except (OSError, subprocess.TimeoutExpired) as exc:
    print(f'Emulator card not updated: {exc}', file=sys.stderr)
    sys.exit(1)
if result.returncode:
    print('Emulator card not updated: emulator unavailable or card/autorun did not match.', file=sys.stderr)
    if result.stderr:
        print(result.stderr.decode(errors='replace').strip(), file=sys.stderr)
    sys.exit(1)
print('Emulator autorun updated:', result.stdout.decode().strip())
