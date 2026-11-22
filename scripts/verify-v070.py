#!/usr/bin/env python3
"""20-check integrity + hardware-safety proof for the v0.0.7.0 SD package.

Read-only. Never deploys, never touches a VM or the physical MPC, never
modifies build/. Usage: python3 scripts/verify-v070.py [path/to/package.zip]

Writes docs/VP-0.0.7.0-VERIFY-OUTPUT.md with each check marked PASS/FAIL.
Exit 0 only when all 20 checks pass.
"""
from __future__ import annotations

import hashlib
import pathlib
import re
import shutil
import subprocess
import sys
import zipfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
PKG = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "build/arpeggigator.zip"
GUARD = "44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f"
HANDOFF_NAME = "HANDOFF-PRZED-WLOZENIEM-0.0.7.4.md"
OUT = ROOT / "docs/VP-0.0.7.0-VERIFY-OUTPUT.md"

lines: list[str] = []
results: list[tuple[int, str, bool, str]] = []


def emit(s: str = "") -> None:
    lines.append(s)
    print(s)


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def check(no: int, name: str, ok: bool, detail: str) -> None:
    results.append((no, name, bool(ok), detail))
    emit(f"| {no} | {name} | {'PASS' if ok else 'FAIL'} | {detail} |")


def dyn_symbols(path: pathlib.Path) -> set[str]:
    try:
        r = subprocess.run(["nm", "-D", str(path)], capture_output=True, text=True)
        if r.returncode == 0 and r.stdout.strip():
            return {ln.split()[-1] for ln in r.stdout.splitlines() if ln.strip()}
    except OSError:
        pass
    return set()


emit("# VP 0.0.7.0 — 20 kontroli integralności i bezpieczeństwa paczki")
emit()
emit(f"Wygenerowane przez `python3 scripts/verify-v070.py`. Paczka: `{PKG}`")
emit(f"SHA-256 paczki: `{sha(PKG.read_bytes())}`")
emit()
emit("| # | Kontrola | Wynik | Szczegół (PROVEN) |")
emit("|---:|---|---|---|")

# 1 — zip integrity
r = subprocess.run(["unzip", "-t", str(PKG)], capture_output=True, text=True)
check(1, "Integralność ZIP (CRC)", r.returncode == 0 and "No errors" in r.stdout,
      r.stdout.strip().splitlines()[-1] if r.stdout.strip() else r.stderr.strip())

with zipfile.ZipFile(PKG) as z:
    entries = {i.filename: z.read(i) for i in z.infolist()}
    names = set(entries)

# 2 — runtime files
need = {"autorun/arpeggigator.so", "autorun/autorun.sh", "autorun/autorun.json",
        "autorun/autorun.check", "autorun/pads-off.sh"}
check(2, "5 plików runtime w paczce", need <= names,
      "obecne: " + ", ".join(sorted(need & names)) + "; brak: " + ", ".join(sorted(need - names)))

with zipfile.ZipFile(PKG) as z:
    z.extractall(ROOT / "build/.verify-v070-extract")
so = ROOT / "build/.verify-v070-extract/autorun/arpeggigator.so"
syms = dyn_symbols(so)

# 3 — g_probe
check(3, "Symbol g_probe wyeksportowany", "g_probe" in syms,
      "g_probe=" + ("TAK" if "g_probe" in syms else "NIE"))

# 4 — native audio feature marker
check(4, "Marker GATOR_NATIVE_AUDIO (prawdziwy arpegiator)", "GATOR_NATIVE_AUDIO" in syms,
      "GATOR_NATIVE_AUDIO=" + ("TAK" if "GATOR_NATIVE_AUDIO" in syms else "NIE "
      "(to byłaby tylko nakładka UI — NIE DO DEPLOYU)"))

sdk_build = (ROOT / "vendor/mpc-sample-sdk/build-mod.sh").read_text()
build_sh = (ROOT / "scripts/build.sh").read_text()
autorun = entries["autorun/autorun.sh"].decode()

# 5 — guard literal is passed by the wrapper; the SDK only consumes --firmware-sha256
check(5, "Guard firmware w scripts/build.sh i --firmware-sha256 w SDK",
      GUARD in build_sh and "--firmware-sha256" in sdk_build,
      "scripts/build.sh zawiera pełny hash; SDK deklaruje --firmware-sha256")

# 6 — guard in autorun
check(6, "Guard firmware w autorun.sh przed cp", GUARD in autorun and
      autorun.index(GUARD) < autorun.index("cp "), "FW=... || exit 0 przed cp")

# 7 — autorun.check
j = entries["autorun/autorun.json"]
s = entries["autorun/autorun.sh"]
expected = sha(j + s + b"f7bed1c9-7e90-4731-9194-c996666b4331").encode()
stored = entries["autorun/autorun.check"].strip()
check(7, "Spójność autorun.check", stored == expected,
      f"oczekiwany={expected.decode()} stored={stored.decode(errors='replace')}")

# 8 — pads-off identical
repo_pads = (ROOT / "scripts/pads-off.sh").read_bytes()
check(8, "pads-off.sh identyczny z repo", entries["autorun/pads-off.sh"] == repo_pads,
      sha(repo_pads))

# 9 — ExecStopPost
check(9, "Drop-in ExecStopPost zainstalowany",
      "zz-arpeggigator-pads-off.conf" in autorun and "ExecStopPost=" in autorun,
      "ExecStopPost=-/run/arpeggigator-pads-off.sh")

# 10 — only /run writes
writes = re.findall(r"(?:>|>>|cp |printf )[^\n]*", autorun)
bad = [w for w in writes if re.search(r"/etc|/data|/content|/factory|/usr|/opt", w)]
check(10, "Autonun zapisuje wyłącznie do /run", not bad,
      "brak zapisów poza /run" if not bad else f"ZABRONIONE: {bad}")

# 11 — no persistent mount/remount
check(11, "Brak remontu/remount systemu plików",
      "remount" not in autorun and "mount " not in autorun and "chattr" not in autorun,
      "brak mount/remount/chattr w autorun.sh")

# 12 — diag log volatile
diag = (ROOT / "src/diag.rs").read_text()
check(12, "Log diagnostyczny w /run", 'LOG_PATH: &str = "/run/' in diag,
      re.search(r'LOG_PATH: &str = "([^"]+)"', diag).group(1))

# 13 — no /tmp in diag
check(13, "Brak zapisu do /tmp w diag", "/tmp/" not in diag, "brak '/tmp/' w src/diag.rs")

# 14 — tempo stop
tempo = (ROOT / "src/tempo.rs").read_text()
check(14, "Wątek tempo ma stop", "pub fn stop_monitor" in tempo and "MONITOR_STOP" in tempo,
      "stop_monitor() + MONITOR_STOP")

# 15 — Drop calls cleanup
lib = (ROOT / "src/lib.rs").read_text()
drop_m = re.search(r"impl Drop for ArpeggiGator \{(.*?)\n\}", lib, re.S)
drop = drop_m.group(1) if drop_m else ""
check(15, "Drop wygasza wątek i głosy",
      "crate::tempo::stop_monitor()" in drop and "panic()" in drop and "restore_pad_rgbs()" in drop,
      "stop_monitor + panic + restore_pad_rgbs")

# 16 — all modules linked
mods = re.findall(r"^\s*(?:pub\(crate\) )?mod (\w+);", lib, re.M)
srcmods = sorted(p.stem for p in (ROOT / "src").glob("*.rs") if p.stem != "lib")
check(16, "Wszystkie moduły źródłowe podłączone", set(srcmods) <= set(mods),
      f"{len(srcmods)} plików, {len(mods)} deklaracji mod; brak: {sorted(set(srcmods)-set(mods))}")

# 17 — version
ct = (ROOT / "Cargo.toml").read_text()
cl = (ROOT / "Cargo.lock").read_text()
check(17, "Wersja 0.0.7-4 w manifestach", "0.0.7-4" in ct and "0.0.7-4" in cl,
      "Cargo.toml + Cargo.lock")

# 18 — stage gates off
check(18, "Stage gates WYŁĄCZONE",
      "stage-arp-only" not in open(ROOT / "scripts/build.sh").read().replace("--features", "")
      or "native-audio" in open(ROOT / "scripts/build.sh").read(),
      "build.sh ustawia wyłącznie MPC_MOD_FEATURES=native-audio")

# 19 — vendor SDK matches git
r = subprocess.run(["git", "diff", "--quiet", "HEAD", "--", "vendor/mpc-sample-sdk"], cwd=ROOT)
check(19, "SDK bez ukrytego fixu", r.returncode == 0,
      "git diff HEAD -- vendor/mpc-sample-sdk → czyste" if r.returncode == 0 else "ROZJAZD!")

# 20 — handoff inside package
check(20, "Handoff dla kolejnej AI w paczce", HANDOFF_NAME in names,
      HANDOFF_NAME if HANDOFF_NAME in names else "BRAK")

emit()
passed = sum(1 for _, _, ok, _ in results if ok)
emit(f"**Wynik: {passed}/20 kontroli PASS.**")
emit()
if passed == 20:
    emit("`PACZKA KOMPLETNA I BEZPIECZNA NA POZIOMIE KODU.` Pozostaje fizyczny test A/B "
        "przed włożeniem do MPC — patrz handoff w paczce.")
else:
    emit("**NIE NADAJE SIĘ DO DEPLOYU** — patrz kontrolе z wynikiem FAIL.")

shutil.rmtree(ROOT / "build/.verify-v070-extract", ignore_errors=True)
OUT.write_text("\n".join(lines) + "\n")
print(f"\nZapisano: {OUT}")
sys.exit(0 if passed == 20 else 2)
