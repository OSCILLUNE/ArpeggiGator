# VP-0.0.7.0-CZESIEK-PLAN — Plan akceptacji, wytyczne QA/screenshotów, tabela provenance, handoff

**Autor:** Czesiek (`opencode/iceman-free`) — plan, kryteria akceptacji, tabela `src → paczka`, guidelines weryfikacji i screenshotów, handoff/prompt.
** Projekt:** ArpeggiGator v0.0.7.0. **Repo:** `/Users/fafi/Downloads/ArpeggiGator`.
**Snapshot odczytów repo:** `2026-11-18 17:34:34Z` (`date -u`); uzupełnienia odczytane w tym samym oknie 17:3xZ.
**HEAD:** `b912eed1028c9b9c54a166b36433cd349e782be0`.
**Zakres mojej pracy:** TYLKO ten plik. Nie zmieniłem `src/`, `build/`, nie dotknąłem maszyn wirtualnych, nie deployowałem, nie rebootowałem.

---

## 0. TRZY ZDANIA-KANON (powtarzam przy każdej decyzji)

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.**
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`).**
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.**

---

## 1. LEGENDA I ZASADA DOWODOWA

| Tag | Znaczenie |
|---|---|
| **PROVEN** | Zweryfikowane przeze mnie komendą w tym dokumencie (odczyt repo / plików). Pełna komenda podana przy fakcie. |
| **PROVEN (Heńek 17:32Z)** | Zweryfikowane przez poprzedniego agenta i zapisane w `docs/HANDOFF-0.0.7.0-PLAN.md` / `docs/logs/ag6/night-20261118/QA-FRESH-FROM-ZERO.md`. **Ja nie dotykałem maszyn**, więc nie potwierdzam tego niezależnie. |
| **INFERRED** | Wniosek z dokumentacji/nazwy pliku. Wymaga potwierdzenia komendą przed akceptacją. |
| **DO WERYFIKACJI** | Kryterium otwarte — właściciel (Flint/Krystyna/Jurek/fafi) musi dostarczyć dowód. |

**Zasada naczelna:** każde twierdzenie w raporcie końcowym ma parametr `date -u`, pełną komendę i pełny wynik. Nie wystarczy „kompiluje się i testy przechodzą". Nie ufamy testom opartym **wyłącznie** na heartbeat/OCR. Jeśli czegokolwiek nie da się udowodnić → piszemy wprost **„NIE NADAJE SIĘ DO DEPLOYU"**.

---

## 2. STAN WYJŚCIOWY (PROVEN — odczyt repo 17:34:34Z)

### 2.1 Repo i wersja

```
cd /Users/fafi/Downloads/ArpeggiGator
date -u
git rev-parse HEAD                  # b912eed1028c9b9c54a166b36433cd349e782be0
git tag -l | grep -E 'v0\.0\.6|v0\.0\.7'
git status --short
git diff --stat
```

| Fakt | Wynik | Status |
|---|---|---|
| HEAD | `b912eed1028c9b9c54a166b36433cd349e782be0` | PROVEN |
| Tagi | `v0.0.6.0`, `v0.0.6.1`, `v0.0.6.2`. **Brak `v0.0.6.3` i `v0.0.7.0`.** | PROVEN |
| `git diff --stat` | `CHANGELOG.md (+8)`, `Cargo.lock (2)`, `Cargo.toml (2)` — **3 pliki tracked zmodyfikowane** | PROVEN |
| `Cargo.toml` w HEAD | `version = "0.0.6-0"` | PROVEN (`git show HEAD:Cargo.toml`) |
| `Cargo.toml` w roboczym | `version = "0.0.7-0"` | PROVEN |
| `Cargo.lock` w roboczym | `version = "0.0.7-0"` | PROVEN |
| `CHANGELOG.md` | Ma już sekcję `## 0.0.7.0` z deklaracją: build hardware **bez** `MPC_MOD_FEATURES=emulator-audio`, autorun tylko w `/run`, brak trwałych zmian | PROVEN (treść), INFERRED (zgodność z kodem — do audytu) |
| Liczba nieśledzonych (`??`) | bardzo wiele (logi, screeny, skrypty `ag6_*.py`, pptx, itp.) | PROVEN |

> **UWAGA / ROZJEŻDŻENIE Z HANDOFFEM:** handoff (17:12Z) podaje „tracked files: czyste". O 17:34:34Z były już **3 zmodyfikowane pliki tracked** (wersja + changelog). To nie jest błąd, ale fakt do odnotowania w nowym handoffie: wersja `0.0.7-0` jest **w drzewie roboczym, nie w commicie**. Żadnych commitów ani pushów nie wykonano.

### 2.2 Buildy i paczka — pułapki (PROVEN, `shasum -a 256 build/*.so build/*.zip`)

| Plik | SHA-256 | Uwaga |
|---|---|---|
| `build/arpeggigator-060.so` | `34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b` | **JEDYNA dobra baza 0.0.6.0** (zgodna z DEV/QA i handoffem) |
| `build/arpeggigator-061.so` | `6e0eac84e2332ac3cdf03217269f4c45a12ceb0ae4f3d8066542a042c9896550` | odrzucona seria 0.0.6.1 |
| `build/arpeggigator-062.so` | `cf71383ce821af7e14c44de1922f775d6111ba2f845070acb692011406fc641f` | odrzucona seria 0.0.6.2 |
| `build/arpeggigator-063.so` | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` | odrzucona seria 0.0.6.3 |
| `build/arpeggigator-final.so` | `d76837d4614ef95b4501b2c88172397be4cefaf9cdb35c310fd884abe92697ea` | **stara pułapka** z handoffu |
| `build/arpeggigator-stage-d.so` | `0a8a60c0ff6cc60424d12218dc74751ab0c31b0428550656dcdbf01535732981` | staging, nie do użycia |
| `build/arpeggigator.so` (bieżący) | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` | **= odrzucony 0.0.6.3!** |
| `build/arpeggigator.zip` (bieżący) | `da392e0f5c9df5a668c56f7295a73d06acfa41d40c018ff952eae8dc566f298f` | **= odrzucony 0.0.6.3!** |
| `build/arpeggigator-061.zip` | `7b6dd70b90c197d1593ce1a80fcb54679125ea827664b164ac390a0a14725c0f` | archiwum |
| `build/arpeggigator-062.zip` | `6a894fd3644b14d15f7d231ed8b4a8baad023b4e62c7c60a10f90b853f7ae4b8` | archiwum |
| `build/arpeggigator-final.zip` | `386b349e8303a25e3b7c1340f17d75e32428795ea1ebeb84e0744811cf6a39c0` | pułapka |
| `build/arpeggigator-qa.zip` | `d9eb01ebe171129f1cb086b6d7ef59b364af89ac51cf715ca826985123ae1325` | stare QA |
| `build/arpeggigator-stage-d.zip` | `4479fe6acd03d6ba3ff3399e6b3ec78e0a5fdc1890dd6c319375889b4859bb16` | staging |

> **Krytyczne:** aktualne `build/arpeggigator.{so,zip}` to **odrzucony 0.0.6.3**. Nowa paczka v0.0.7.0 musi mieć **nowy, inny hash** i być zbudowana z czystego źródła. Nie wolno wskrzesić żadnej z powyższych poza `arpeggigator-060.so` jako punktu odniesienia do porównania.

### 2.3 Zawartość bieżącej paczki — wzorzec listy plików (PROVEN, `unzip -l build/arpeggigator.zip`)

```
autorun/                      (katalog)
autorun/autorun.check             64 B
autorun/arpeggigator.so       565760 B  (tu: zły 0.0.6.3)
autorun/autorun.sh               822 B
autorun/autorun.json             247 B
autorun/pads-off.sh             1055 B
```

`autorun/autorun.sh` (PROVEN, `unzip -p … autorun/autorun.sh`) wykonuje, po guardzie firmware:
`cp …/arpeggigator.so /run/arpeggigator.so` → `mkdir -p /run/systemd/system/ac50.service.d` → drop-in `arpeggigator.conf` z `Environment=LD_PRELOAD=/run/arpeggigator.so` → `cp …/pads-off.sh /run/arpeggigator-pads-off.sh` → drop-in `zz-arpeggigator-pads-off.conf` z `ExecStopPost=-/run/arpeggigator-pads-off.sh` → `systemctl daemon-reload` → `systemctl restart ac50`. **Wszystkie zapisy są w `/run`** (tom-substituted tmpfs — do potwierdzenia na fizycznym sprzęcie przez Krystynę). Guard: `FW="44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f"`, porównywany z `dd if="/usr/bin/MPC Sample" bs=4096 count=1 | sha256sum`.

### 2.4 Maszyny — stan z dokumentacji (PROVEN przez Heńka 17:32Z; JA NIE DOTYKAŁEM VM)

| | DEV | QA (świeżo od zera) |
|---|---|---|
| Panel | `http://127.0.0.1:8765` | `http://127.0.0.1:8766` |
| SSH | port `2022` | port `2032` |
| Klucz | `/Users/fafi/Downloads/virtual-mpc-sample/out/keys/id_ed25519` | `/Users/fafi/Downloads/virtual-mpc-sample-qa/out/keys/id_ed25519` |
| Katalog labu | `/Users/fafi/Downloads/virtual-mpc-sample` | `/Users/fafi/Downloads/virtual-mpc-sample-qa` |
| VNC (z `lab.py PORTS`) | `5910` (display `:10`) | `5920` (display `:20`) |
| Inne porty QA | — | `cs 5651, ext_midi 5652, console 5653, qmp 5654, qmp_panel 5655, audio 5656, pcm 5657, usb_ssh 2232` |
| Moduł | `/run/arpeggigator.so` = `34a9ec26…`, PID 1103 | `NO_SO`, `NO_DROPIN`, `MAPS=0`, `NRestarts=0`, brak `[AG6]`, brak karty, `connected:true` |
| Uwaga | „ dobry build" — nie dotykać niepotrzebnie | Przeznaczona do testów v0.0.7.0 za zgodą fafi |

Hashes obrazów bazowych QA (PROVEN przez Heńka 17:32Z, `docs/logs/ag6/night-20261118/QA-FRESH-FROM-ZERO.md`):

| Plik | SHA-256 |
|---|---|
| `out/emmc.base.qcow2` | `4235b1ba50b9b290b84a41e711b7e8c380d6f5c242639f47d8c105d5baef2519` |
| `out/lab-layer.ext4` | `9d53ad5989cbf2b751aa1b5206efab4ab0baea3bb535225660ae6f7a54453446` |
| `out/kernel/Image` | `fe17d8b3e0b1fee83803fc8657e975c7c011e13e039a978bf15f0f66cfcceac8` |
| `out/initramfs.cpio.gz` | `e3f3635adbbf13e97cad86fefd55446e58d64c44ed71af55610a4dfee6933f68` |

### 2.5 API panelu — fakty do komend (PROVEN, odczyt `panel/server.py` i `controls.json`)

- `GET /` → panel WWW; `GET /screen.png` → **LCD 320×240**; `GET /state` → JSON z polami `connected, leds, pads, values, pots, rx_bytes, log, sysex, midi{counts,log,bpm}, rear{levels,patch,...}, sdcard{inserted,error,folder}, pcm_port, power`.
- `POST /control` z `{"name":…,"action":"press"|"release"|"set"|"turn","value":…}` → `{"ok":true,"sent":"<hex>"}`.
- Nazwy kontrolek (PROVEN z `controls.json`): `SHIFT`, `SCREEN 1` (= B1), `SCREEN 2` (= B2), `SCREEN 3` (= B3), `16 LEVELS`, `DATA` (= HOLD w 16 Levels), `PAD 1`…`PAD 16`, `K1/K2/K3`, `FADER`, `MAIN VOLUME`.
- `POST /sdcard {"action":"insert"|"eject"}`; `POST /power` (przełącz zasilanie, `system_powerdown`/`system_reset`); `POST /autorun`, `POST /midi`, `POST /route`, `POST /send`.
- Mapa B1/B2/B3 → `SCREEN 1/2/3` jest PROVEN z `17_log.json` nocnych testów (`SHIFT press` → `90 31 7f`, `SCREEN 1 press` → `90 0c 7f`).
- **Semantyka SHIFT+B1:** README mówi „ARP on/off", ale harness nocny używał `SHIFT+SCREEN 1` do zmiany pola `mode` w heartbeat. rozstrzygnąć z kodu (Flint) — INFERRED. W wytycznych poniżej opisuję **obserwowalne skutki**, nie interpretację.

---

## 3. DEFINICJA „QA JAK ZE SKLEPU" — stany dziewicze i komendy

**Definicja (z handoffu pkt 4 + QA-FRESH-FROM-ZERO):** stan dziewiczy to `NO_SO`, `NO_DROPIN`, `MAPS=0`, `NRestarts=0`, brak `[AG6]`, brak karty w `/media/MPCSAMPLE`, panel `connected=true`. Dodaję: firmware zgodny z guardem, ekran fabrycznego edytora sampli, zero ABRT/SEGV w journalu.

### 3.1 Jedna komenda audytu dziewiczości (QA)

```bash
QA=/Users/fafi/Downloads/virtual-mpc-sample-qa
ssh -o BatchMode=yes -o ConnectTimeout=5 -i $QA/out/keys/id_ed25519 -p 2032 root@127.0.0.1 '
  printf "SO=";      ls /run/arpeggigator.so 2>/dev/null || echo NO_SO
  printf "DROPIN=";  ls /run/systemd/system/ac50.service.d/ 2>/dev/null | tr "\n" " "; echo
  printf "ETC_DROPIN="; find /etc/systemd/system -iname "*arpeggiator*" -print 2>/dev/null | wc -l
  printf "MEDIA=";   ls -A /media/MPCSAMPLE 2>/dev/null || echo NO_MEDIA
  printf "FW=";      dd if="/usr/bin/MPC Sample" bs=4096 count=1 2>/dev/null | sha256sum | cut -d" " -f1
  systemctl show ac50 -p MainPID -p NRestarts -p ActiveState
  p=$(systemctl show ac50 -p MainPID --value)
  printf "MAPS=";    grep -c arpeggigator /proc/$p/maps 2>/dev/null || echo 0
  printf "HB=";      tail -1 /tmp/arpeggigator-6-events.log 2>/dev/null || echo NO_HB
  printf "AG6_JOURNAL="; journalctl -u ac50 -b --no-pager 2>/dev/null | grep -ci AG6
  printf "ABRT=";    journalctl -u ac50 -b --no-pager 2>/dev/null | grep -ciE "ABRT|SEGV|double free|dumped|core"
'
curl -s --max-time 5 http://127.0.0.1:8766/state | jq '{connected, power, sdcard: .sdcard.inserted}'
```

**Oczekiwany wynik (dziewiczy):**

```
SO=NO_SO
DROPIN=            (puste)
ETC_DROPIN=0
MEDIA=NO_MEDIA
FW=44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f
MainPID=<liczba>
NRestarts=0
ActiveState=active
MAPS=0
HB=NO_HB            (albo brak pliku)
AG6_JOURNAL=0
ABRT=0
{ "connected": true, "power": "on", "sdcard": false }
```

### 3.2 Tabela kontrolna „jak ze sklepu"

| # | Warunek | Komenda weryfikująca | Oczekiwany wynik |
|---|---|---|---|
| V1 | `NO_SO` | `ssh … 'ls /run/arpeggigator.so 2>&1'` | `No such file or directory` |
| V2 | `NO_DROPIN` | `ssh … 'ls -A /run/systemd/system/ac50.service.d 2>&1; find /etc/systemd/system -iname "*arpeggiator*"'` | puste / brak wyników |
| V3 | `MAPS=0` | `ssh … 'p=$(systemctl show ac50 -p MainPID --value); grep -c arpeggigator /proc/$p/maps \|\| echo 0'` | `0` |
| V4 | `NRestarts=0` | `ssh … 'systemctl show ac50 -p NRestarts -p ActiveState -p MainPID'` | `NRestarts=0`, `ActiveState=active`, stały PID |
| V5 | brak `[AG6]` | `ssh … 'tail -1 /tmp/arpeggigator-6-events.log; journalctl -u ac50 -b --no-pager \| grep -ci AG6'` | brak heartbeat / `0` |
| V6 | brak karty | `curl -s localhost:8766/state \| jq .sdcard.inserted` + `ssh … 'ls -A /media/MPCSAMPLE'` | `false`, puste |
| V7 | panel `connected=true` | `curl -s localhost:8766/state \| jq .connected` | `true` |
| V8 | firmware | `ssh … 'dd if="/usr/bin/MPC Sample" bs=4096 count=1 2>/dev/null \| sha256sum'` | `44507c7a…5299d1f` |
| V9 | ekran fabryczny | `curl -s localhost:8766/screen.png -o /tmp/virgin.png` i obejrzeć | `A01 BoomBap-K11-BTB`, waveform, zakładki Trim/Tune/Filter, **brak** nakładki aligatora/ARP/FRZ, brak splashu |
| V10 | brak crashy | `ssh … 'journalctl -u ac50 -b --no-pager \| grep -ciE "ABRT\|SEGV\|core"'` | `0` |

**Wzorzec z 17:32:15Z (PROVEN przez Heńka):** `NO_SO`, drop-iny `NONE`, `MainPID/NRestarts/ActiveState = 325/0/active`, `MAPS=0`, `/media` puste, heartbeat `NO_HB`, ABRT/SEGV/core `0`, firmware zgodny, panel `connected:true`, ekran `A01 BoomBap-K11-BTB`, projekt `/data/TempProjects/Temp-Project.xpj` 205651 B mtime `17:31:37Z`. **Nie wpisuj własnego mtime do raportu — zawsze czytaj `stat`.**

### 3.3 Jak odtworzyć stan dziewiczy (jeśli fafi zażądają — NIE robić bez zgody)

```bash
cd /Users/fafi/Downloads/virtual-mpc-sample-qa
python3 lab.py status
ps aux | grep -E 'lab\.py|reboot'      # upewnić się, że nie trwa cudzy reboot
date -u
python3 lab.py stop                    # zamyka porty 5651-5657 i 2032
python3 lab.py reset                   # usuwa overlay; NIE dotyka emmc.base.qcow2
python3 lab.py run                     # świeży emmc.qcow2 z emmc.base.qcow2
```
Potem audyt z pkt 3.1. Uwaga z handoffu: „nie robić tego w trakcie cudzego rebootu". W chwili przejęcia taki proces był opisany i został zatrzymany; sprawdzić `date -u`, `ps`, `lab.py status` **przed** jakąkolwiek akcją.

---

## 4. PLAN AKCEPTACJI v0.0.7.0 — kryteria MUST-HAVE / MUST-NOT

Każde kryterium: ID, typ, treść, komenda, oczekiwany wynik, powiązane ostrzeżenie (W1…W10 z handoffu), status dowodu.

### 4.1 Mapa 10 ostrzeżeń → kryteria

| Ostrzeżenie (skrót) | Kryteria |
|---|---|
| W1 „cały kod linia po linii, żeby nie rozwalić fizycznej MPC" | K01, K02, K03, K13–K20 |
| W2 „analiza linia po linii pod kątem bezpieczeństwa sprzętu" | K03, K13–K18, K20 |
| W3 „żadnego deployu na fizyczną MPC bez pełnej analizy i weryfikacji paczki" | K08, K19, K39 |
| W4 „każda linia dotykająca `LD_PRELOAD`, `ac50.service`, `/run`, autorunu, SysEx, ALSA, wątków — przeczytana" | K03, K06, K09, K13, K14, K18, K20 |
| W5 „cały kod przepisać na nowo, nie kopiować z wirtualki" | K01, K02, K10, K12 |
| W6 „sprawdzić cały arpegiator — wszystkie pliki, symbole, zasoby" | K07, K08, K11 |
| W7 „żaden fix tylko w VM/systemie/`ac50`" | K10, K14, K15, K19 |
| W8 „po wyjęciu karty MPC w stanie jak przed" | K14–K18, K21–K31 |
| W9 „używaliśmy emulatora audio — ustalić co właściwe dla hardware" | K05, K07, K11 |
| W10 „pracujemy całą noc: pisać i weryfikować" | K36, K38, K40 |

### 4.2 GRUPA A — Kod, wersja, provenance (W1, W2, W4, W5, W6, W7, W9)

| ID | Typ | Kryterium | Komenda weryfikująca | Oczekiwany wynik | Status |
|---|---|---|---|---|---|
| K01 | MUST-HAVE | Bazą jest czysty `v0.0.6.0` = `b912eed`; nie użyto 0.0.6.1/2/3 | `git rev-parse HEAD; git tag -l \| grep -E '0\.0\.6\.[123]\|0\.0\.7'` | HEAD = `b912eed…` (lub jego potomek z udokumentowanym przepisaniem); brak tagu `v0.0.6.3`; `v0.0.7.0` dopiero po akceptacji | PROVEN |
| K02 | MUST-HAVE | Cały `src/` przepisany na nowo, każda linia przejrzana | audyt `git diff b912eed -- src/` + raport linia-po-linii Flinta | raport `docs/VP-0.0.7.0-*.md` z PROVEN/INFERRED dla każdego pliku z 16 | DO WERYFIKACJI (Flint) |
| K03 | MUST-HAVE | Analiza bezpieczeństwa sprzętu obejmuje: `autorun.sh`, `LD_PRELOAD`, drop-iny, `ExecStopPost`, `pads-off.sh`, `/run`, `/media`, SysEx, ALSA, wątki, repaint, audio | lista sekcji w raporcie Krystyny + komendy przy każdej | wszystkie 11 obszarów obecne, każdy z dowodem | DO WERYFIKACJI (Krystyna) |
| K04 | MUST-HAVE | Wersja `0.0.7-0` w `Cargo.toml` i `Cargo.lock`, wpis w CHANGELOG | `grep -n '^version' Cargo.toml; grep -n -A1 'name = "arpeggigator"' Cargo.lock; head -12 CHANGELOG.md` | `0.0.7-0` w obu; sekcja `## 0.0.7.0` | PROVEN (drzewo robocze); wymaga commita |
| K05 | MUST-NOT | Paczka hardware **nie** zawiera `MPC_MOD_FEATURES=emulator-audio` | `grep -n 'MPC_MOD_FEATURES' scripts/build.sh scripts/build-emulator-audio.sh; <log buildu>` | standardowy `scripts/build.sh`; feature tylko w `build-emulator-audio.sh`; log buildu bez `emulator-audio` | INFERRED z CHANGELOG — DO WERYFIKACJI (build log) |
| K06 | MUST-HAVE | Guard firmware'u w buildzie i w autorunie | `grep -n '44507c7a…' scripts/build.sh; unzip -p build/arpeggigator.zip autorun/autorun.sh \| grep '^FW='` | oba zawierają `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f` | PROVEN |
| K07 | MUST-HAVE | Symbol `g_probe` obecny w finalnym `.so` | `unzip -p build/arpeggigator.zip autorun/arpeggigator.so > /tmp/ag.so && nm -D /tmp/ag.so \| grep g_probe` | `T g_probe` (lub ekwiwalent z `llvm-nm`) | DO WERYFIKACJI (Jurek) |
| K08 | MUST-HAVE | Paczka kompletna: dokładnie znana lista plików, żadnych brakujących | `unzip -l build/arpeggigator.zip` | 6 wpisów: `autorun/`, `autorun.check`, `arpeggigator.so`, `autorun.sh`, `autorun.json`, `pads-off.sh`; każdy zhasowany; brak plików emulator-only | PROVEN (wzorzec); DO WERYFIKACJI (nowa paczka) |
| K09 | MUST-HAVE | `package-pad-cleanup.py` poprawnie Applied | `unzip -p build/arpeggigator.zip autorun/autorun.sh \| grep -nE 'pads-off\|ExecStopPost\|daemon-reload'`; przeliczenie `autorun.check` wg wzoru z `sha256(autorun.json + autorun.sh + 'f7bed1c9-7e90-4731-9194-c996666b4331')` | dokładnie 1× `systemctl daemon-reload`; drop-in `ExecStopPost` tylko w `/run`; `autorun.check` zgodny | PROVEN (mechanizm); DO WERYFIKACJI (nowa paczka) |
| K10 | MUST-NOT | Żaden fix nie istnieje tylko w VM / systemie gościa / usłudze `ac50`; nie używamy `lab.py preload` jako dowodu | porównanie `git diff b912eed -- src/ scripts/` z listą „co zmieniono na VM" + `history \| grep preload` | całe zachowanie wynika z repo i paczki; brak zależności od ręcznych zmian w gościu | DO WERYFIKACJI (Flint/Krystyna) |
| K11 | MUST-HAVE | Tabela „plik źródłowy → plik w paczce → hash" wypełniona (16 `src/*.rs` + skrypty + Cargo + assety) | `shasum -a 256 src/*.rs scripts/* Cargo.toml Cargo.lock CHANGELOG.md assets/*`; sekcja 7 tego raportu | zero `<<DO WYPEŁNIENIA>>`; każdy wiersz ma pełny 64-znakowy hash | DO WERYFIKACJI (Jurek); szkielet w §7 |
| K12 | MUST-HAVE | Czesiek nie zmienił `src/` ani `build/` | `git status --short \| grep -E '^ M (src/\|build/)'` | puste (zmiany tylko w `docs/VP-0.0.7.0-CZESIEK-PLAN.md`) | PROVEN (przed zapisem raportu) |

### 4.3 GRUPA B — Bezpieczeństwo fizyczne i wyjęcie karty (W1, W2, W3, W4, W7, W8)

| ID | Typ | Kryterium | Komenda weryfikująca | Oczekiwany wynik | Status |
|---|---|---|---|---|---|
| K13 | MUST-HAVE | Autorun zapisuje **wyłącznie** do `/run` | `unzip -p build/arpeggigator.zip autorun/autorun.sh \| grep -nE 'cp \|mkdir \|printf \|tee\|>'` | ścieżki: `/run/arpeggigator.so`, `/run/systemd/system/ac50.service.d`, `/run/arpeggigator-pads-off.sh`. Zero zapisów do `/etc`, `/usr`, `/data`, `/boot` | PROVEN (stary pakiet); DO WERYFIKACJI (nowy) |
| K14 | MUST-HAVE | Drop-iny są na tmpfs, nie w `/etc` | `lab.py ssh 'mount \| grep " /run "; ls -la /run/systemd/system/ac50.service.d; find /etc/systemd/system -iname "*arpeggiator*" -print'` | `/run` = `tmpfs`; drop-iny widoczne tylko z kartą; `/etc` puste | DO WERYFIKACJI (VM + fizyczna — Krystyna) |
| K15 | MUST-HAVE | Po wyjęciu karty + restarcie: `NO_SO`, `NO_DROPIN`, `MAPS=0`, `NRestarts=0`, brak `[AG6]` | audyt z §3.1 po `POST /sdcard {"action":"eject"}` i cyklu zasilania | wszystkie wartości jak w §3.1; `ac50` identyczny jak przed włożeniem karty | DO WERYFIKACJI (fafi + Krystyna) |
| K16 | MUST-HAVE | Firmware konsoli niezmieniony | `dd if="/usr/bin/MPC Sample" bs=4096 count=1 2>/dev/null \| sha256sum` | `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f` przed i po | PROVEN (guard); DO WERYFIKACJI (fizyczna) |
| K17 | MUST-HAVE | Projekt i sample niezmienione przez mod | audyt kodu: `grep -R '/data\|/sample\|open(' src/ -n` + `stat`/`sha256sum` plików projektu przed/po | mod nie zapisuje do projektu/sampledanych; bez różnic w hashach (mtime może być regenerowany przez firmware — nie wpisywać zmyślonych wartości) | DO WERYFIKACJI (Flint) |
| K18 | MUST-HAVE | Cleanup padów przy zatrzymaniu usługi działa | `lab.py ssh 'ls -l /run/arpeggigator-pads-off.sh; systemctl cat ac50 \| grep ExecStopPost'` + odczyt `.pads` z panelu | `ExecStopPost=-/run/arpeggigator-pads-off.sh`; po restarcie/stopie pady wyłączone | INFERRED (ALSA `Surface` na fizycznej hardware niezweryfikowane) |
| K19 | MUST-NOT | Deploy na fizyczną MPC przed zamknięciem pkt 1–4 i pisemną zgodą fafi | dziennik decyzji w nowym handoffie + brak komendy `lab.py autorun` na fizycznym | brak deployu; w przeciwnym razie natychmiast „NIE NADAJE SIĘ DO DEPLOYU" | PROCEDURALNE |
| K20 | MUST-HAVE | Nieznany firmware → fail-closed (brak zmian) | test w VM: zmienić 1 bajt w porównywanym hashu (tylko w warstwie testowej) i uruchomić autorun; alternatywnie dowód z kodu | `autorun.sh` kończy `exit 0` przed `cp`; brak SO i drop-inu | DO WERYFIKACJI (Krystyna) |

### 4.4 GRUPA C — QA „jak ze sklepu" (W8, W10)

Kryteria V1–V10 z §3.2 wchodzą w skład akceptacji jako **K21–K30** (identyczne komendy i wyniki). Dodatkowo:

| ID | Typ | Kryterium | Komenda | Oczekiwany wynik | Status |
|---|---|---|---|---|---|
| K31 | MUST-HAVE | QA postawiona z obrazów bazowych, nie ze snapshotu | `shasum -a 256 /Users/fafi/Downloads/virtual-mpc-sample-qa/out/emmc.base.qcow2 …` + `QA-FRESH-FROM-ZERO.md` | hashy z §2.4; brak użycia `.pristine` / `.dirty-*`; stare `out/run` zarchiwizowane | PROVEN (Heńek 17:32Z) |
| K32 | MUST-HAVE | Po teście paczki da się wrócić do stanu dziewiczego | `lab.py stop` → `lab.py reset` → `lab.py run` → audyt §3.1 | pełne V1–V10 | DO WERYFIKACJI (fafi) |

### 4.5 GRUPA D — Dowody, screenshoty, regresja (W3, W6, W10)

| ID | Typ | Kryterium | Komenda | Oczekiwany wynik | Status |
|---|---|---|---|---|---|
| K33 | MUST-HAVE | Screen stanu dziewiczego (1200×950 + LCD) z hashem i stanem usługi | `python3 scripts/ag6_probe.py qa VIRGIN_00 '{}'` | katalog z `03_screen1200_before.png` (1200×950), `09_device*.png`, `_state.json`, `_service.txt`, `13_hashes.json`, `14_dimensions.txt` | PROCEDURA w §5 |
| K34 | MUST-HAVE | Screen **w momencie naciśnięcia** SHIFT+B1 / HOLD / pad — przed puszczeniem | `python3 scripts/ag6_probe.py qa <LABEL> '{"enter16":true,"mode":3,"hold":["SHIFT","SCREEN 1"]}'` (i warianty §5.4) | `06_screen1200_PRESSED.png` 1200×950, różny od `before`, `no_meaningful_duplicate=true`, `PASS=true` | PROCEDURA w §5 |
| K35 | MUST-HAVE | Screen po wyjęciu karty | `POST /sdcard eject` → cykl `/power` → audyt + snap | zgodny z V1–V10, ekran fabryczny | PROCEDURA w §5.6 |
| K36 | MUST-HAVE | Każdy screen ma komplet dowodów: kontrola, stan, usługa, hash, `date -u`, wymiary | `cat …/13_hashes.json …/14_dimensions.txt …/17_log.json …/12_service_RELEASED.txt` | `restarts=0`, `abrt=0`, `pressed_1200x950=true`, hash PNG zgodny z plikiem | PROVEN (harness) |
| K37 | MUST-NOT | Nie ufamy wyłącznie heartbeat/OCR; zakaz zmyślania mtime i patrzenia w prawy panel WWW zamiast LCD | krzyżowe porównanie `state.json` + `service.txt` + PNG; `sips -g pixelWidth -g pixelHeight` | zgodność trzech źródeł; wymiary 1200×950 / 320×240; brak raportu z odrzuconego agenta | PROCEDURALNE |
| K38 | MUST-HAVE | Regresja funkcjonalna na nowej paczce: layout 18/18 (DEV 9 + QA 9) i funkcjonalność 4/4 | `ag6_probe.py` dla planów z `docs/logs/ag6/night-20261118/{dev,qa}/*/00_plan.json` | `PASS=true`, mod = nowy hash, `NRestarts=0`, `ABRT=0`, zrzuty 1200×950 | INFERRED — wzorzec z 0.0.6.0 (18/18, 4/4) |
| K39 | MUST-HAVE | Soak 30 min nowej paczki | `docs/tools/snap.sh watch` (30 min, telemetria co 2 s) + raport | `NRestarts=0`, `ABRT=0`, PID stały, panel connected, hash moduła bez zmian | INFERRED — wzorzec: DEV 843 / QA 778 próbek |
| K40 | MUST-NOT | Brak splashu, sticky HOLD, `set_root`/`silence`, phase-preserving, zmian repaintu bez A/B | `grep -RniE 'draw_boot_splash\|set_root\|silence\|phase' src/` + lista zmian repaintu z A/B | żadnej nowej aktywacji; każda zmiana repaintu ma A/B | DO WERYFIKACJI (Flint) |

### 4.6 BRAMKA DECYZYJNA (GO / NO-GO)

```
JEŚLI (wszystkie MUST-HAVE = PASS) ORAZ (zero MUST-NOT = naruszenie)
    ORAZ (K02, K03, K15, K20 mają dowody PROVEN)
  → „NADAJE SIĘ DO DEPLOYU NA CZYSTĄ KARTĘ" (następnie dopiero fizyczna MPC za zgodą fafi)
INACZEJ
  → „NIE NADAJE SIĘ DO DEPLOYU" + raport z ryzykiem
```

---

## 5. WYTYCZNE WERYFIKACJI I SCREENÓW DLA FAFI (QA „jak ze sklepu")

### 5.0 Mapa dostępu — skróty

| Profil | Panel | SSH | Klucz | Lab | VNC |
|---|---|---|---|---|---|
| DEV | `http://127.0.0.1:8765` | `2022` | `/Users/fafi/Downloads/virtual-mpc-sample/out/keys/id_ed25519` | `/Users/fafi/Downloads/virtual-mpc-sample` | `vnc://127.0.0.1:5910` |
| QA | `http://127.0.0.1:8766` | `2032` | `/Users/fafi/Downloads/virtual-mpc-sample-qa/out/keys/id_ed25519` | `/Users/fafi/Downloads/virtual-mpc-sample-qa` | `vnc://127.0.0.1:5920` |

Komendy labu (PROVEN z `lab.py --help`):

```
cd /Users/fafi/Downloads/virtual-mpc-sample-qa
python3 lab.py status                      # stan maszyny (tylko odczyt)
python3 lab.py ssh 'uname -a; systemctl show ac50 -p NRestarts'   # command w VM
python3 lab.py panel                       # uruchom serwer panelu, jeśli nie działa
python3 lab.py run [--headless]            # VM + panel
python3 lab.py stop | reset                # stop / usuń overlay do czystego eMMC
python3 lab.py autorun PAKA.zip --no-reboot --yes   # TYLKO po autoryzacji!
python3 lab.py preload LIB.so | --off      # NIE UŻYWAĆ jako dowód bezpieczeństwa
```

Szybka migawka (tylko odczyt, bez POST/control, bez restartów):

```
SNAP_PROFILE=qa /Users/fafi/Downloads/ArpeggiGator/docs/tools/snap.sh "VIRGIN_00"
# → docs/logs/session-YYYYMMDD-qa/HHMMSS_VIRGIN_00/{screen.png,state.json,summary.json,service.txt,time_utc.txt,screen.sha256}
SNAP_PROFILE=qa docs/tools/snap.sh watch     # telemetria co 2 s + migawka przy restarcie (Ctrl+C)
```

### 5.1 Zasady dowodowe — czytać przed zrobieniem pierwszego screenu

1. **Pełny panel z Chrome headless:** `--window-size=1200,950 --screenshot=…` → plik **1200×950**. To jest panel WWW (przyciski + LCD). Bez tego screen nie jest ważny.
2. **Osobny zrzut samego LCD:** `GET /screen.png` → **320×240**. To jest „widok z konsoli" — na nim oceniamy ekran MPC, nie na prawym panelu WWW.
3. **Trójka zdjęć:** `before` → `PRESSED` (w trakcie trzymania!) → `RELEASED`. Screen „w momencie naciśnięcia" robimy **zanim** wyślemy `release`.
4. **Do każdego screenu:** `_state.json`, `_service.txt` (hash moduła, PID, NRestarts, HB, ABRT), `date -u`, hash PNG (`13_hashes.json`), wymiary (`sips`), odpowiedzi `/control` (`17_log.json`).
5. **Podwójne ekrany:** duplikat bajtowy jest błędem **tylko** gdy coś jest wciśnięte i `before == PRESSED`. Gdy `hold=[]`, identyczność jest oczekiwana (harness oznacza `no_meaningful_duplicate=true`).
6. **Tryb/heartbeat:**
   - `16 LEVELS` wchodzi pętlą z odczytem `l16=1`;
   - zmiana `mode` — do 12 prób `SHIFT+SCREEN 1`, z odczytem po 1,1 s (heartbeat 1 Hz);
   - jeśli nie osiągnięto targetu → `PASS=false`, krok odrzucić i powtórzyć.
7. **Nie ufamy wyłącznie heartbeat/OCR.** Wynik = zgodność `state.json` + `service.txt` + obrazu + hashu.
8. **Nie wpisujemy zmyślonych mtime.** `date -u`, `stat`, `sha256sum` — zawsze z komendy.
9. **Zero deployu na QA bez zgody fafi.** Harness `ag6_probe.py` wykonuje tylko POST `/control`, GET `/state`, GET `/screen.png`, Chrome screenshot i **read-only SSH** — nie restartuje `ac50`, nie zmienia kodu.

### 5.2 KROK 0 — screen stanu dziewiczego (NAJPIERW)

```bash
cd /Users/fafi/Downloads/ArpeggiGator
date -u
cd /Users/fafi/Downloads/virtual-mpc-sample-qa && python3 lab.py status && cd -
curl -s --max-time 5 http://127.0.0.1:8766/state | jq '{connected, power, sdcard: .sdcard.inserted}'
python3 scripts/ag6_probe.py qa VIRGIN_00 '{}'
sips -g pixelWidth -g pixelHeight docs/logs/ag6/night-20261118/qa/VIRGIN_00/03_screen1200_before.png
```

**Co ma być widoczne w `03_screen1200_before.png` i `09_device_PRESSED.png`:**
- ekran `A01 BoomBap-K11-BTB`, waveform, zakładki **Trim / Tune / Filter** (czysty edytor sampli);
- **brak** nakładki aligatora, brak napisów `ARP`, `FRZ`, brak custom captions pod B1/B2/B3;
- **brak splashu** (żadnego boot/start ekranu);
- fabryczne podpisy softkeyów, panel w stanie `connected:true`, karta nie włożona.
- Dodatkowo w `_service.txt`: `NO_SO`, brak drop-inów, `NRestarts=0`, `MAPS=0`, `NO_HB`, `ABRT=0`.

Pliki w katalogu dowodów: `00_plan.json`, `01_service_before.txt`, `02_state_before.json`, `03_screen1200_before.png`, `06_screen1200_PRESSED.png` (= before, bo `hold=[]` — to OK), `07_state_PRESSED.json`, `10_screen1200_RELEASED.png`, `13_hashes.json`, `14_dimensions.txt`, `16_verdict.json` (`PASS=true`), `17_log.json`.

### 5.3 KROK 1 — SHIFT+B1 (ekran w momencie naciśnięcia)

Gotowy, zaakceptowany wzorzec z nocy (`qa/QA_B1_mut_shift_b1`):

```bash
python3 scripts/ag6_probe.py qa QA_SHIFT_B1 '{"enter16":true,"mode":3,"hold":["SHIFT","SCREEN 1"]}'
```

`SCREEN 1` = **B1**. Harness wyśle `SHIFT press`, potem `SCREEN 1 press`, odczeka 0,3 s i zrobi `06_screen1200_PRESSED.png` **zanim** wyśle `release` (kolejność `10…14…` w kodzie). Wynik: `PASS=true`, `pressed_1200x950=true`, `no_meaningful_duplicate=true`, `restarts=0`, `abrt=0`.

**Co ma być widoczne:** wciąż ekran 16 Levels; **tymczasowe** podpisy pod trzema softkeyami widoczne tylko gdy SHIFT jest trzymany (`ARP` / aktualny pattern / `+0..+3` — zgodnie z README; w 0.0.6.0 `mode` zmienia się przez SHIFT+B1 — rozstrzygnąć z kodu, ale screen musi się różnić od `before`); ewentualna nakładka aligatora, gdy ARP włączony; po puszczeniu SHIFT podpisy muszą zniknąć (`10_screen1200_RELEASED.png` różny od `PRESSED`, bez „zastygłego" SHIFT).

Wariant „na żywo" (jeśli fafi chce sam przycisnąć i zrobić screen w tym momencie):

```bash
PANEL=http://127.0.0.1:8766
# wejdź w 16 LEVELS (jeśli jeszcze nie jesteś):
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"16 LEVELS","action":"press"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"16 LEVELS","action":"release"}'
# TRZYMAJ:
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"SHIFT","action":"press"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"SCREEN 1","action":"press"}'
# SCREEN W MOMENCIE NACIŚNIĘCIA (jeszcze przed release!):
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless --disable-gpu --hide-scrollbars \
  --window-size=1200,950 --screenshot=docs/qa_screens/2026-11-18_QA_SHIFT_B1_PRESSED.png $PANEL
curl -s $PANEL/screen.png -o docs/qa_screens/2026-11-18_QA_SHIFT_B1_PRESSED_LCD.png
curl -s $PANEL/state > docs/qa_screens/2026-11-18_QA_SHIFT_B1_PRESSED_state.json
# dopiero teraz puść:
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"SCREEN 1","action":"release"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"SHIFT","action":"release"}'
```

Alternatywnie dla B2/B3: zamień `"SCREEN 1"` na `"SCREEN 2"` / `"SCREEN 3"` (plany `{"enter16":true,"mode":3,"hold":["SHIFT","SCREEN 2"]}` itd. — PROVEN z `00_plan.json`).

### 5.4 KROK 2 — HOLD i pad (screen w momencie naciśnięcia)

`DATA` = HOLD w 16 Levels. PROVEN z `faza_2v2_krok_2.4_hold_control.txt`: `POST /control {"name":"DATA","action":"press","value":127}` → `{"ok": true, "sent": "90 6f 7f"}`. Pad: `{"name":"PAD 1","action":"press","value":127}` → `99 25 7f`.

**HOLD — gotowy plan harnessu:**

```bash
python3 scripts/ag6_probe.py qa QA_HOLD_PAD1 '{"enter16":true,"mode":3,"music":true,"hold":[{"name":"PAD 1","value":100}],"animation":true}'
```

`music:true` włącza HOLD przez `DATA press` i odpala PAD 1/2/3; `hold` trzyma `PAD 1` w momencie zrzutu; `animation:true` robi dodatkowy zrzut po 1,5 s (`09b_screen1200_PRESSED_LATER.png`) — dwa różne hashe dowodzą, że ekran nie zamarł.

**Co ma być widoczne:** w heartbeat `l16=1`, `hold=1`, rosnące `midi.counts`; na LCD podświetlony pad, animacja aligatora (zmiana hashu między `PRESSED` a `PRESSED_LATER`), brak zamrożenia przez pasek odtwarzania sampla; po puszczeniu — brak stale trzymanego HOLD (jeśli HOLD był włączony, `DATA press` ponownie go wyłącza — sprawdzić `hold=0` w `07_state_PRESSED.json` / końcowym stanie).

**Surowe komendy (jeśli fafi robi to ręcznie):**

```bash
PANEL=http://127.0.0.1:8766
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"16 LEVELS","action":"press"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"16 LEVELS","action":"release"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"DATA","action":"press","value":127}'   # HOLD ON
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"PAD 1","action":"press","value":127}'  # TRZYMAJ PAD
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless --disable-gpu --hide-scrollbars \
  --window-size=1200,950 --screenshot=docs/qa_screens/2026-11-18_QA_HOLD_PAD1_PRESSED.png $PANEL
curl -s $PANEL/screen.png -o docs/qa_screens/2026-11-18_QA_HOLD_PAD1_PRESSED_LCD.png
curl -s $PANEL/state > docs/qa_screens/2026-11-18_QA_HOLD_PAD1_PRESSED_state.json
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"PAD 1","action":"release"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"DATA","action":"press","value":127}'   # HOLD OFF
```

Dla samego pada (bez HOLD): plan `{"enter16":true,"mode":2,"music":false,"hold":[{"name":"PAD 5","value":100}]}` (wzorzec `QA_C1_sample_swap`).

### 5.5 KROK 3 — wdrożenie paczki i test na czystej karcie (tylko za zgodą fafi)

```bash
python3 scripts/ag6_probe.py qa VIRGIN_00 '{}'                 # punkt odniesienia
cd /Users/fafi/Downloads/virtual-mpc-sample-qa
python3 lab.py autorun /Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.zip --no-reboot --yes
python3 lab.py ssh 'sha256sum /run/arpeggigator.so; systemctl show ac50 -p NRestarts -p MainPID; systemctl cat ac50 | grep -E "LD_PRELOAD|ExecStopPost"'
curl -s http://127.0.0.1:8766/state | jq '{connected, sdcard: .sdcard.inserted}'
```
Oczekiwany hash `/run/arpeggigator.so` = hash nowej paczki v0.0.7.0 (nie `34a9ec26…`, nie `9beaf588…`). Potem powtórzyć K33–K34 i regresję K38–K39.

### 5.6 KROK 4 — screen PO WYJĘCIU KARTY (najważniejszy dowód W8)

**W emulatorze (bezpieczna symulacja):**

```bash
PANEL=http://127.0.0.1:8766
curl -s -X POST $PANEL/sdcard -H 'Content-Type: application/json' -d '{"action":"eject"}' | jq .
curl -s $PANEL/state | jq '.sdcard'                       # inserted:false
# cykl zasilania BEZ karty (nie używaj `lab.py run` — on włoży kartę z powrotem!):
curl -s -X POST $PANEL/power -H 'Content-Type: application/json' -d '{}'   # powerdown (odczekać ~10-20 s)
curl -s -X POST $PANEL/power -H 'Content-Type: application/json' -d '{}'   # power on
curl -s $PANEL/state | jq '{connected, power, sdcard: .sdcard.inserted}'
cd /Users/fafi/Downloads/ArpeggiGator
python3 scripts/ag6_probe.py qa AFTER_EJECT_00 '{}'
SNAP_PROFILE=qa docs/tools/snap.sh "AFTER_EJECT_00"
# i pełny audyt z §3.1 — musi wyjść identycznie jak VIRGIN_00
```

**Co ma być widoczne po wyjęciu:** identycznie jak w kroku 0 — ekran `A01 BoomBap-K11-BTB`, brak nakładki/ARP/FRZ/splashu; `NO_SO`, `NO_DROPIN`, `MAPS=0`, `NRestarts=0`, `NO_HB`/`AG6_JOURNAL=0`, `ABRT=0`, `connected:true`, `sdcard.inserted=false`, firmware `44507c7a…`. Hash `03_screen1200_before.png` z `AFTER_EJECT_00` nie musi być bajtowo identyczny z `VIRGIN_00` (animacja/faza ekranu), ale **wszystkie pola audytu muszą się zgadzać**.

**Na fizycznej MPC Sample (procedura dla fafi — NIE wykonawana przeze mnie):**
1. Wyłączyć konsolę, wyjąć kartę SD, włączyć ponownie.
2. `dd if="/usr/bin/MPC Sample" bs=4096 count=1 2>/dev/null | sha256sum` → `44507c7a…` (jeśli dostępny shell).
3. `find /etc/systemd/system -iname '*arpeggiator*'`, `ls /run/arpeggigator.so`, `ls /run/systemd/system/ac50.service.d` → puste.
4. `systemctl show ac50 -p MainPID -p NRestarts -p ActiveState; grep -c arpeggigator /proc/$(systemctl show ac50 -p MainPID --value)/maps` → `0`.
5. `journalctl -u ac50 -b --no-pager | grep -ciE 'AG6|ABRT|SEGV|core'` → `0`.
6. Zachowanie: brak monitu autorun, stockowe pady i softkey'e, brak animacji moda, projekt/sample nienaruszone.
7. Jeśli choć jedno nie zgadza się z stanem sprzed włożenia karty → **NIE NADAJE SIĘ DO DEPLOYU**.

### 5.7 KROK 5 — VNC do obserwacji na żywo

```
open vnc://127.0.0.1:5920    # QA (display :20)
open vnc://127.0.0.1:5910    # DEV (display :10)
```
Porty PROVEN z `lab.py PORTS` i parametru `-vnc HOST:(PORT-5900)`. VNC służy tylko do obserwacji przez człowieka; **dowodem pozostaje screenshot z Chrome + state + service**.

### 5.8 Nazewnictwo dowodów

`docs/logs/ag6/night-20261118/<profil>/<ETYKIETA>/` (harness) albo `docs/qa_screens/2026-11-18_<ETYKIETA>_<co>.png` (ręczne). ETYKIETA: `VIRGIN_00`, `QA_SHIFT_B1`, `QA_SHIFT_B2`, `QA_SHIFT_B3`, `QA_HOLD_PAD1`, `AFTER_EJECT_00`, `SOAK_30M`. Nigdy nie nadpisuj starych dowodów — nowa etykieta za każdym razem.

---

## 6. BŁĘDY POPRZEDNICH RUNDOW — NIE POWTARZAĆ

- Stare dane z pamięci zamiast komend („28% duplikatów" gdy było 16%).
- „ebe2e2e zabezpieczyło handoff" — prawdziwy hash to `0565071`.
- Mylenie QA z DEV (różne porty, klucze, katalogi).
- Zmyślone mtime w raporcie wizualnym; patrzenie w prawy panel WWW zamiast ekranu LCD.
- Raport wizualny agenta odrzucony — nie używać.
- Uszkodzona klatka w `dev_track1` (ffmpeg) — przyjęto `dev_track2` i `qa_track3`.
- Kolumna hasha w telemetrii soaku miała błąd parsera (`Wed` zamiast hasha) — hash zawsze weryfikować osobnym SSH.
- Reboot QA w trakcie cudzego procesu — najpierw `date -u`, `ps aux | grep lab.py`, `lab.py status`.
- Wskrzeszanie 0.0.6.1/0.0.6.2/0.0.6.3 — baza tylko `b912eed`.

---

## 7. SZKIELET TABELI „PLIK ŹRÓDŁOWY → PLIK W PACZCE → HASH"

**Zasada**: 16 plików `src/*.rs` kompiluje się do **jednego** `autorun/arpeggigator.so`. Dowód nie jest „1 plik → 1 hash", lecz: `hash źródła` + `pełna lista 16 źródeł` + `build z czystego źródła` + `symbol g_probe` + `hash wpisu w paczce` + `guard firmware'u`. Hashe **nie zostały przeze mnie wypełnione** — to zadanie Jurka (`opencode/big-pickle`). Komendy do wypełnienia pod tabelą.

### 7.1 Pliki `src/*.rs` (16 — PROVEN z `ls -la src/`)

| # | Plik źródłowy | Rola (INFERRED z nazwy/docs) | Entry w paczce | SHA-256 źródła | SHA-256 entry | Uwagi |
|---|---|---|---|---|---|---|
| 1 | `src/acordator.rs` | logika akordów (INFERRED) | `autorun/arpeggigator.so` | `<<DO WYPEŁNIENIA — JUREK>>` | `<<DO WYPEŁNIENIA — JUREK>>` | many-to-one |
| 2 | `src/arp.rs` | silnik wzorców (README/IMPLEMENTATION: 8 wzorców) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |
| 3 | `src/diag.rs` | heartbeat / diagnostyka (IMPLEMENTATION) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |
| 4 | `src/lib.rs` | punkt wejścia modułu, hooki, `g_probe` (INFERRED) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | sprawdzić `g_probe` |
| 5 | `src/melody.rs` | silnik melodyczny (INFERRED) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |
| 6 | `src/mutator.rs` | tryb MUTATOR (INFERRED z logów) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |
| 7 | `src/native_audio.rs` | natywny retrigger audio — **feature `emulator-audio`** | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | **W9: sprawdzić, czy nie aktywny w hardware** |
| 8 | `src/orbitator.rs` | tryb ORBITATOR (INFERRED z logów) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |
| 9 | `src/playback.rs` | cykl życia nut (INFERRED) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | każdy NoteOn → NoteOff |
| 10 | `src/probe.rs` | probe / guard (INFERRED) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | powiązany z `g_probe` |
| 11 | `src/scheduler.rs` | czysty silnik timingu (PROVEN z IMPLEMENTATION) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |
| 12 | `src/splash.rs` | nakładka/aligator (INFERRED — nie boot splash!) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | K40: zero boot splashu |
| 13 | `src/splash_art.rs` | grafika pikselowa (INFERRED) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |
| 14 | `src/tempo.rs` | odczyt BPM z projektu (PROVEN z IMPLEMENTATION) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | tylko read-only |
| 15 | `src/transport_clock.rs` | faza zegara transportu (INFERRED) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | K40: bez phase-preserving |
| 16 | `src/transport_playback.rs` | integracja z transportem (INFERRED) | `autorun/arpeggigator.so` | `<<…>>` | `<<…>>` | many-to-one |

### 7.2 Skrypty, manifesty, dokumentacja, assety

| Plik źródłowy | Entry w paczce / rola | SHA-256 źródła | SHA-256 entry | Uwagi |
|---|---|---|---|---|
| `scripts/build.sh` | build-time → generuje paczkę przez SDK | `<<DO WYPEŁNIENIA>>` | n/a | wywołuje `build-mod.sh`, `--require-symbol g_probe`, guard FW |
| `scripts/build-emulator-audio.sh` | **NIE MA być w paczce hardware** | `<<…>>` | n/a | W9: tylko emulator |
| `scripts/package-pad-cleanup.py` | `autorun/autorun.sh`, `autorun/autorun.check`, `autorun/pads-off.sh` | `<<…>>` | `<<…>>` | wzór hasha z kodu: `sha256(autorun.json + autorun.sh + UUID f7bed1c9-…)` |
| `scripts/pads-off.sh` | `autorun/pads-off.sh` | `<<…>>` | `<<…>>` | identyczny bajtowo z wpisem (`z.write`) |
| `scripts/sync-emulator-autorun.py` | build-time, **nie w paczce** | `<<…>>` | n/a | aktualizuje kartę emulatora |
| `scripts/README.md` | dokumentacja | `<<…>>` | n/a | — |
| `scripts/ag6_probe.py` | dowody QA (nie w paczce) | `<<…>>` | n/a | usek w §5 |
| `scripts/ag6_step.py` | dowody QA (nie w paczce) | `<<…>>` | n/a | — |
| `scripts/ag6_record.py` | wideo QA (nie w paczce) | `<<…>>` | n/a | — |
| `scripts/ag6_compose.py` | muzyka do wideo (nie w paczce) | `<<…>>` | n/a | — |
| `scripts/ag6_soak_night.py`, `scripts/ag6_soak.sh` | soak (nie w paczce) | `<<…>>` | n/a | — |
| `scripts/ag6_pointf_pair.py` | pare klatek (nie w paczce) | `<<…>>` | n/a | — |
| `scripts/emulator_repaint_observer.{py,sh}` | obserwator (nie w paczce) | `<<…>>` | n/a | — |
| `Cargo.toml` | build input; wersja | `<<…>>` | n/a | PROVEN: HEAD `0.0.6-0`, roboczy `0.0.7-0` |
| `Cargo.lock` | build input; wersja | `<<…>>` | n/a | PROVEN: roboczy `0.0.7-0` |
| `CHANGELOG.md` | dokumentacja wydania | `<<…>>` | n/a | ma sekcję `0.0.7.0` |
| `README.md` | dokumentacja | `<<…>>` | n/a | opis sterowania |
| `assets/arpeggigator-logo.svg` | dokumentacja/README | `<<…>>` | n/a | — |
| `assets/arpeggigator-splash.svg` | dokumentacja (nie boot splash!) | `<<…>>` | n/a | K40 |
| `vendor/mpc-sample-sdk` (pin `87d3ec20…`) | generuje `autorun.sh` + guard | `<<…>>` | `<<…>>` | wg `IMPLEMENTATION.md` |
| `docs/tools/snap.sh` | narzędzie QA (nie w paczce) | `<<…>>` | n/a | — |
| `docs/PROMPT-0.0.7.0-NOWY-WATEK.md` | handoff (nie w paczce) | `<<…>>` | n/a | — |
| `docs/HANDOFF-0.0.7.0-PLAN.md` | handoff (nie w paczce) | `<<…>>` | n/a | — |

### 7.3 Komendy do wypełnienia hashy (Jurek)

```bash
cd /Users/fafi/Downloads/ArpeggiGator
# źródła
shasum -a 256 src/*.rs scripts/* Cargo.toml Cargo.lock CHANGELOG.md README.md assets/* docs/tools/snap.sh
# wpisy paczki
unzip -l build/arpeggigator.zip
for f in autorun/arpeggigator.so autorun/autorun.sh autorun/autorun.json autorun/autorun.check autorun/pads-off.sh; do
  printf '%s  ' "$f"; unzip -p build/arpeggigator.zip "$f" | shasum -a 256
done
# symbol i guard
unzip -p build/arpeggigator.zip autorun/arpeggigator.so > /tmp/ag070.so && nm -D /tmp/ag070.so | grep g_probe
unzip -p build/arpeggigator.zip autorun/autorun.sh | grep -E '^FW=|/run/|/etc/'
# pewność, że paczka to NIE stary build
shasum -a 256 build/arpeggigator.so build/arpeggigator.zip   # NIE MOŻE być 9beaf588… / da392e0f…
```

### 7.4 Kontrola kompletności — lista przeciwstawna (MUST-NOT)

W paczce **nie może być**: `build-emulator-audio.sh`, `ag6_*.py`, plików z `target/`, `build/*.so` jako osobnych wpisów, splashy PNG, skryptów zapisujących do `/etc/`, `/usr/`, `/data/`, `/boot/`, oraz żadnego z hashy z §2.2 oznaczonych „odrzucony/pułapka".

---

## 8. PROJEKT HANDOFFU DLA NASTĘPNEJ AI

### 8.1 HANDOFF — przejęcie wątku (szkielet do uzupełnienia przez Ling Ping)

```
# HANDOFF — przejęcie wątku: v0.0.7.0 (data -u: <wpisać z date -u>)
## 0. KTO PRZEJMUJE
Nowa AI przejmuje wątek od <...>. Źródło prawdy: docs/HANDOFF-0.0.7.0-PLAN.md,
docs/PROMPT-0.0.7.0-NOWY-WATEK.md, docs/VP-0.0.7.0-CZESIEK-PLAN.md (ten raport).

## ⚠️ OSTRZEŻENIE NR 1/10 — NAJCZYSTSZE I NAJWAŻNIEJSZE
TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ
FIZYCZNEJ MPC SAMPLE.
## ⚠️ OSTRZEŻENIE NR 2/10
PRZEANALIZOWAĆ KOD LINIA PO LINII POD KĄTEM BEZPIECZEŃSTWA SPRZĘTU. Nie wystarczy
„kompiluje się i testy przechodzą".
## ⚠️ OSTRZEŻENIE NR 3/10
ŻADNEGO DEPLOYU NA FIZYCZNĄ MPC BEZ PEŁNEJ, LINIOWEJ ANALIZY I WERYFIKACJI PACZKI.
Błąd może uszkodzić konsolę.
## ⚠️ OSTRZEŻENIE NR 4/10
KAŻDA LINIA KODU, KTÓRA DOTYKA LD_PRELOAD, ac50.service, /run, autorunu karty SD,
SysEx, ALSA i wątków, MUSI BYĆ PRZECZYTANA I ZROZUMIANA.
## ⚠️ OSTRZEŻENIE NR 5/10
CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — NIE KOPIOWAĆ Z WIRTUALKI, NIE ZAKŁADAĆ ŻE
„TO DZIAŁA". To ma być v0.0.7.0.
## ⚠️ OSTRZEŻENIE NR 6/10
SPRAWDZIĆ, CZY W PACZCE JEST CAŁY ARPEGIATOR — WSZYSTKIE PLIKI, WSZYSTKIE SYMBOLE,
WSZYSTKIE ZASOBY. Nic nie może zaginąć.
## ⚠️ OSTRZEŻENIE NR 7/10
SPRAWDZIĆ, CZY ŻADEN FIX NIE ZOSTAŁ WPROWADZONY TYLKO W WIRTUALNEJ MASZYNIE,
W SYSTEMIE OPERACYJNYM ALBO W SERWISIE ac50. Kod musi być samowystarczalny.
## ⚠️ OSTRZEŻENIE NR 8/10
MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC SAMPLE KONSOLA BĘDZIE
W STANIE JAK PRZED WŁOŻENIEM KARTY. Zero trwałych zmian w systemie.
## ⚠️ OSTRZEŻENIE NR 9/10
TU UŻYWALIŚMY AUDIO EMULATORA (MPC_MOD_FEATURES=emulator-audio). TRZEBA PRZEPISAĆ
KOD I USTALIĆ, CO JEST WŁAŚCIWE DLA FIZYCZNEGO SPRZĘTU. Nie zakładać, że build
emulatora jest buildem docelowym.
## ⚠️ OSTRZEŻENIE NR 10/10
PRACUJESIĘ CAŁĄ NOC: PISAĆ I WERYFIKOWAĆ. Celem jest co najmniej „nie uszkodzić MPC",
a najlepiej „działa na czystej karcie i po jej wyjęciu jest jak przed".

## 1. STAN REPO (PROVEN — Czesiek 17:34:34Z)
HEAD b912eed1028c9b9c54a166b36433cd349e782be0; tagi v0.0.6.0/1/2, brak v0.0.7.0.
Tracked zmodyfikowane: Cargo.toml, Cargo.lock, CHANGELOG.md (wersja 0.0.7-0 w drzewie
roboczym, nie w commicie). 16 plików src/*.rs. build/arpeggigator.{so,zip} = ODRZUCONY
0.0.6.3 (9beaf588… / da392e0f…) — NIE UŻYWAĆ. Dobra baza: build/arpeggigator-060.so
= 34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b.

## 2. STAN MASZYN (PROVEN przez Heńka 17:32Z; nowa AI ma zweryfikować komendami)
DEV: panel 8765, SSH 2022, klucz virtual-mpc-sample/out/keys/id_ed25519, mod 34a9ec26…,
     PID 1103, VNC 5910. NIE BYŁ dotykany przy odtwarzaniu QA.
QA:  panel 8766, SSH 2032, klucz virtual-mpc-sample-qa/out/keys/id_ed25519, VNC 5920.
     Postawiona od zera z emmc.base.qcow2 (4235b1ba…), STAN DZIEWICZY: NO_SO,
     NO_DROPIN, MAPS=0, NRestarts=0, brak [AG6], brak karty, connected:true.
     Przed jakąkolwiek akcją: date -u; ps aux | grep lab.py; lab.py status.

## 3. ZADANIA
1) Przeczytać handoff + prompt + VP-0.0.7.0-CZESIEK-PLAN.md i zweryfikować komendami.
2) Flint: rewersja i przepisanie kodu linia po linii (16 plików), raport PROVEN/INFERRED.
3) Krystyna: mechanizmy/firmware/ac50/SysEx/ALSA/wątki, dowód „po wyjęciu karty jak przed".
4) Jurek: hashe, kompletność paczki, tabela src → paczka (szkielet §7), g_probe, guard.
5) Ernie: cross-check PROVEN vs INFERRED. Ling Ping: ten handoff i ślad zmian.
6) Licznik/Wojtek: szybka weryfikacja komendami (bez wniosków bez komendy).
7) Wzrok: wizja — każdy raport weryfikować; poprzedni został odrzucony.
8) Chemik: forensyka crashy (limit 20/dobę). Kronikarz/Świeżak: kompletność/dyscyplina
   (bywali na limicie FREE).
9) fafi: QA „jak ze sklepu" według §5 — screen dziewiczy, w momencie naciśnięcia,
   po wyjęciu karty; komendy i porty w §5.0.
10) Dopiero po 1–9: paczka v0.0.7.0 do zatwierdzenia. Jeśli czegoś nie umiesz udowodnić
    → „NIE NADAJE SIĘ DO DEPLOYU".

## 4. ZAKAZY
Nie pushować bez autoryzacji. Nie resetować OpenCode. Nie dotykać fizycznej MPC.
Nie wdrażać na QA w trakcie cudzego rebootu. Nie ufać testom opartym wyłącznie na
heartbeat/OCR. Nie powtarzać 0.0.6.1/0.0.6.2/0.0.6.3. Nie robić splashu, sticky HOLD,
set_root/silence, phase-preserving, zmian repaintu bez A/B.

## 5. CO MA ZWRÓCIĆ NOWA AI
1) Pełną listę plików źródłowych i paczki z hashami (tabela §7 wypełniona).
2) Raport linia-po-linii bezpieczeństwa sprzętu (PROVEN/INFERRED).
3) Dowód, że po wyjęciu karty MPC jest w stanie sprzed włożenia (K15/K16/K18 + §5.6).
4) Świeżą QA z komendami startu i stanem dziewiczym (§3).
5) Paczkę v0.0.7.0 — dopiero po pkt 1–4.
6) Jeśli czegokolwiek nie da się udowodnić — „NIE NADAJE SIĘ DO DEPLOYU".
```

### 8.2 PROMPT STARTOWY DO SKOPIOWANIA (analog `PROMPT-0.0.7.0-NOWY-WATEK.md`)

```
Przejmujesz wątek od Heńka (Strażnik Pamięci i Ciągłości) i Czeska (plan akceptacji).
Jesteś odpowiedzialna za v0.0.7.0 — paczkę na CZYSTĄ kartę pamięci, potem do komputera
i fizycznej MPC Sample.

NATYCHMIAST PRZECZYTAJ CAŁE PLIKI:
/Users/fafi/Downloads/ArpeggiGator/docs/HANDOFF-0.0.7.0-PLAN.md
/Users/fafi/Downloads/ArpeggiGator/docs/PROMPT-0.0.7.0-NOWY-WATEK.md
/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-CZESIEK-PLAN.md
To nie jest opcjonalne. Tam jest stan maszyn, hashe, zakazy, agenci, 10 ostrzeżeń,
40 kryteriów akceptacji i procedura screenshotów.

TRZY ZDANIA, KTÓRE POWTARZASZ PRZY KAŻDEJ DECYZJI:
1. TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ
   FIZYCZNEJ MPC SAMPLE.
2. CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora
   (MPC_MOD_FEATURES=emulator-audio). Nie zakładać, że build emulatora jest buildem
   dla fizycznego sprzętu.
3. MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE
   JAK PRZED WŁOŻENIEM KARTY.

CZEGO NIE WOLNO CI ZROBIĆ:
- Nie wdrażaj na fizyczną MPC. Nie pushuj. Nie resetuj OpenCode.
- Nie dotykaj QA, jeśli trwa reboot z innego procesu. Najpierw date -u,
  ps aux | grep lab.py, python3 lab.py status.
- Nie używaj 0.0.6.1/0.0.6.2/0.0.6.3 ani bieżących build/arpeggigator.{so,zip}
  (to odrzucony 0.0.6.3: 9beaf588… / da392e0f…). Baza to b912eed.
- Nie ufaj testom opartym wyłącznie na heartbeat/OCR. Nie zmyślaj mtime.
- Nie powtarzaj błędów: „28% duplikatów", „ebe2e2e zabezpieczyło handoff" (prawda:
  0565071), mylenie QA z DEV, fałszywe hashe, raport wizualny z patrzeniem w prawy
  panel zamiast LCD.
- Nie rób splashu, sticky HOLD, set_root/silence, phase-preserving, zmian repaintu
  bez A/B.

CO MASZ ZROBIĆ:
1) Zweryfikować handoff komendami: git rev-parse HEAD; git status; shasum -a 256;
   git tag -l; date -u.
2) Rozdzielić agentów z handoffu (Flint, Krystyna, Jurek, Czesiek, Ernie, Ling Ping,
   Licznik, Wojtek, Wzrok, Chemik, Kronikarz, Świeżak). Każde twierdzenie poparte
   komendą. PROVEN vs INFERRED.
3) Przepisać cały kod z v0.0.6.0 (16 plików src/*.rs), wersja 0.0.7-0, CHANGELOG.
   Build hardware BEZ emulator-audio — udowodnić logiem buildu.
4) Zbudować paczkę i wypełnić tabelę §7: 16 źródeł + skrypty + Cargo + assety +
   wpisy paczki, pełne hashe, g_probe, guard 44507c7a…5299d1f, package-pad-cleanup.py.
5) Linia po linii przeanalizować bezpieczeństwo fizyczne: autorun, LD_PRELOAD,
   drop-iny, ac50, SysEx, ALSA, wątki, /run, /media. Udowodnić K15/K16/K18 i §5.6.
6) Przeprowadzić z fafi QA „jak ze sklepu" (§5): VIRGIN_00, SHIFT+B1/B2/B3,
   HOLD+pad, AFTER_EJECT_00. Porty: DEV 8765/2022/5910, QA 8766/2032/5920;
   klucze w out/keys/id_ed25519 obu labów.
7) Sprawdzić 40 kryteriów z §4. Dopiero gdy wszystkie MUST-HAVE = PASS i zero
   MUST-NOT → paczka do zatwierdzenia. Inaczej napisz wprost:
   „NIE NADAJE SIĘ DO DEPLOYU".

STYL: Nie pocieszaj. Diagnozuj. Przyznawaj błędy. Podawaj pełne hashe, pełne ścieżki,
date -u. Maksymalna jakość i rzetelność. Jedna zmiana do przodu — ale cała paczka
musi być kompletna i bezpieczna.

Przejmujesz wątek. 🪖☕
```

---

## 9. PODSUMOWANIE DLA KOORDYNATORA — 5 NAJWAŻNIEJSZYCH SEKCJI

1. **§4 — Plan akceptacji:** 40 kryteriów MUST-HAVE/MUST-NOT, każde z komendą i oczekiwanym wynikiem, mapa 10 ostrzeżeń W1–W10 i bramka GO/NO-GO.
2. **§5 — Wytyczne QA/screenshotów dla fafi:** kompletny przepis na screen dziewiczy, screen w momencie naciśnięcia (SHIFT+B1 = `SCREEN 1`, HOLD = `DATA`, pad = `PAD n`) i screen po wyjęciu karty, z dosłownymi komendami `lab.py`, `curl /control`, `ag6_probe.py`, portami i kluczami.
3. **§3 — Definicja „QA jak ze sklepu"** i jednokomendowy audyt dziewiczości (V1–V10).
4. **§7 — Szkielet tabeli `src → paczka → hash`:** 16 plików `src/*.rs`, skrypty, Cargo, CHANGELOG, assety, wpisy `autorun/*` oraz komendy do wypełnienia hashy przez Jurka.
5. **§8 — Handoff i prompt startowy** dla następnej AI: 10 ostrzeżeń w dosłownym brzmieniu, zakazy, lista agentów, stan maszyn i repo.

**Najważniejsze ostrzeżenie operacyjne z mojego odczytu:** `build/arpeggigator.{so,zip}` to obecnie **odrzucony 0.0.6.3** (`9beaf588…` / `da392e0f…`), a trzy pliki tracked (`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`) mają wersję `0.0.7-0` tylko w drzewie roboczym. Nowa AI musi to zweryfikować jako **pierwszą komendę** i nie może użyć bieżących artefaktów.

**Trzy zdania-kanon (jeszcze raz):**
1. TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.
2. CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`).
3. MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.

Czesiek 🪖☕
