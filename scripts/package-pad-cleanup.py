#!/usr/bin/env python3
"""Add the emulator-tested service cleanup to the SDK autorun package."""
import hashlib
from pathlib import Path
import zipfile

root = Path(__file__).resolve().parents[1]
package = root / 'build/arpeggigator.zip'
with zipfile.ZipFile(package) as z:
    entries = {info.filename: (info, z.read(info)) for info in z.infolist()}
name = 'autorun/autorun.sh'
info, script = entries[name]
needle = b'systemctl daemon-reload\n'
if script.count(needle) != 1:
    raise SystemExit('unexpected SDK autorun script; refusing to modify package')
addition = (b'cp "$DIR/pads-off.sh" /run/arpeggigator-pads-off.sh || exit 0\n'
            b'chmod 755 /run/arpeggigator-pads-off.sh\n'
            b"printf '[Service]\\nExecStopPost=-/run/arpeggigator-pads-off.sh\\n' > /run/systemd/system/ac50.service.d/zz-arpeggigator-pads-off.conf\n")
script = script.replace(needle, addition + needle)
entries[name] = (info, script)
check_info, _ = entries['autorun/autorun.check']
check = hashlib.sha256(entries['autorun/autorun.json'][1] + script + b'f7bed1c9-7e90-4731-9194-c996666b4331').hexdigest().encode()
entries['autorun/autorun.check'] = (check_info, check)
temporary = package.with_suffix('.zip.new')
with zipfile.ZipFile(temporary, 'w') as z:
    for info, contents in entries.values():
        z.writestr(info, contents)
    z.write(root / 'scripts/pads-off.sh', 'autorun/pads-off.sh')
temporary.replace(package)
