# VP-0.0.7.0 — JUREK: INTEGRALNOŚĆ LICZB, HASHY I PACZKI

**Agent:** JUREK (liczby, hashe, integralność paczki) — zespół Jeremiasza
**Data pomiarów:** 2026-11-18, `date -u` → `Wed Nov 18 17:34:05 UTC 2026` (start przejęcia wątku: `Wed Nov 18 17:32:07 UTC 2026`)
**Repo:** `cd /Users/fafi/Downloads/ArpeggiGator`
**HEAD:** `b912eed1028c9b9c54a166b36433cd349e782be0`
**Branch:** `release/0.0.6.4`
**Tracked files:** czyste (`git status --porcelain=v1 -- src/ Cargo.toml Cargo.lock` → puste; pełny status pokazuje wyłącznie pliki `??`, w tym dokumenty i logi innych agentów)
**Handoff przeczytany w całości:** `docs/HANDOFF-0.0.7.0-PLAN.md` (152 linie)
**Zakres:** TYLKO pliki repo. Nie dotykano maszyn DEV/QA, nie deployowano, nie restartowano. Nie zmieniono `src/` ani `build/`. Jedyna zmiana to ten raport.

---

## TRZY ZDANIA-KANON (powtórzone)

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.**
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (MPC_MOD_FEATURES=emulator-audio).**
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.**

Wersja handoffowa kanonu nr 2 (linia 27) brzmi: „CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — NIE KOPIOWAĆ Z WIRTUALKI, NIE ZAKŁADAĆ ŻE „TO DZIAŁA". To ma być v0.0.7.0." Obie formy są w tym raporcie utrzymane jako nadrzędne.

**Werdykt skrócony: NIE NADAJE SIĘ DO DEPLOYU.** Szczegóły w pkt 9.

---

## 1. Hashe buildów z handoffu — weryfikacja komendą

**Komenda (PROVEN):**
```
cd /Users/fafi/Downloads/ArpeggiGator
shasum -a 256 build/arpeggigator-060.so build/arpeggigator-062.so build/arpeggigator-063.so build/arpeggigator-final.so build/arpeggigator.zip build/arpeggigator-qa.zip
```

**Wynik (PROVEN):**
```
34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b  build/arpeggigator-060.so
cf71383ce821af7e14c44de1922f775d6111ba2f845070acb692011406fc641f  build/arpeggigator-062.so
9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d  build/arpeggigator-063.so
d76837d4614ef95b4501b2c88172397be4cefaf9cdb35c310fd884abe92697ea  build/arpeggigator-final.so
da392e0f5c9df5a668c56f7295a73d06acfa41d40c018ff952eae8dc566f298f  build/arpeggigator.zip
d9eb01ebe171129f1cb086b6d7ef59b364af89ac51cf715ca826985123ae1325  build/arpeggigator-qa.zip
```

| Plik | Hash zmierzony | Hash w handoffie (linie 63–66) | Zgodność |
|---|---|---|---|
| `/Users/fafi/Downloads/ArpeggiGator/build/arpeggigator-060.so` | `34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b` | `34a9ec26…` | ✅ PROVEN — identyczny |
| `…/build/arpeggigator-062.so` | `cf71383ce821af7e14c44de1922f775d6111ba2f845070acb692011406fc641f` | `cf71383c…` | ✅ PROVEN — identyczny |
| `…/build/arpeggigator-063.so` | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` | `9beaf588…` | ✅ PROVEN — identyczny |
| `…/build/arpeggigator-final.so` | `d76837d4614ef95b4501b2c88172397be4cefaf9cdb35c310fd884abe92697ea` | `d76837d4…` | ✅ PROVEN — identyczny ( pułapka ) |
| `…/build/arpeggigator.zip` | `da392e0f5c9df5a668c56f7295a73d06acfa41d40c018ff952eae8dc566f298f` | **BRAK** | ⚠️ PROVEN pomiar, **brak wartości referencyjnej** |
| `…/build/arpeggigator-qa.zip` | `d9eb01ebe171129f1cb086b6d7ef59b364af89ac51cf715ca826985123ae1325` | **BRAK** | ⚠️ PROVEN pomiar, **brak wartości referencyjnej** |

**Luka w handoffie (PROVEN):** przeszukanie `docs/` pod kątem obu hashy zipów (`grep -rn 'da392e0f…\|d9eb01eb…' docs/`) zwróciło pusty wynik. Handoff podaje hashe tylko dla czterech plików `.so` (linie 63–66), mimo że zadanie zlecone Jurkowi mówi o „hashach z handoffu" także dla obu zipów. **Nie da się zweryfikować zipów względem handoffu — można tylko odnotować ich aktualny odczyt.** To samo dotyczy pliku `docs/PROMPT-0.0.7.0-NOWY-WATEK.md`.

**Dodatkowe artefakty znalezione w `build/` (PROVEN, `shasum -a 256 build/*.so`):**
```
6e0eac84e2332ac3cdf03217269f4c45a12ceb0ae4f3d8066542a042c9896550  build/arpeggigator-061.so
0a8a60c0ff6cc60424d12218dc74751ab0c31b0428550656dcdbf01535732981  build/arpeggigator-stage-d.so
9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d  build/arpeggigator.so
```
`build/arpeggigator.so` (bez sufiksu) jest **bitowo identyczny z odrzuconym `-063.so`** (PROVEN).

**Znaczniki czasu (PROVEN, `ls -la build/`):**
| Plik | mtime |
|---|---|
| `arpeggigator-final.so` | Nov 18 03:37 |
| `arpeggigator-qa.zip` | Nov 17 17:14 |
| `arpeggigator-062.so` | Nov 18 10:31 |
| `arpeggigator-063.so` | Nov 18 12:54 |
| `arpeggigator.so` / `.elf` / `.zip` | Nov 18 12:54 |
| **`arpeggigator-060.so`** | **Nov 18 15:07** ← najnowszy, potwierdzony testami |

---

## 2. Zawartość paczek — pełna lista z rozmiarami i hashami

### 2.1 `unzip -l build/arpeggigator.zip` (PROVEN)
```
Archive:  build/arpeggigator.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
        0  11-18-2026 12:54   autorun/
       64  11-18-2026 12:54   autorun/autorun.check
   565760  11-18-2026 12:54   autorun/arpeggigator.so
      822  11-18-2026 12:54   autorun/autorun.sh
      247  11-18-2026 12:54   autorun/autorun.json
     1055  11-17-2026 17:45   autorun/pads-off.sh
---------                     -------
   567948                     6 files
```

### 2.2 `unzip -l build/arpeggigator-qa.zip` (PROVEN)
```
Archive:  build/arpeggigator-qa.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
        0  11-14-2026 03:38   autorun/
       64  11-14-2026 03:38   autorun/autorun.check
   569640  11-17-2026 17:14   autorun/arpeggigator.so
      587  11-14-2026 03:38   autorun/autorun.sh
      247  11-14-2026 03:38   autorun/autorun.json
---------                     -------
   570538                     5 files
```

### 2.3 Hashe sha256 **zawartości** każdego pliku (`unzip -p … | shasum -a 256`) — PROVEN

**`build/arpeggigator.zip`:**
| Plik w paczce | Rozmiar | SHA-256 zawartości | Uwaga |
|---|---:|---|---|
| `autorun/autorun.check` | 64 | `9decad364fd69b6fd909048631202994a7889f8c7c1783fbaa155a57a7145dad` | to hash **pliku**, którego treścią jest inny 64-zn. hash — patrz pkt 5 |
| `autorun/arpeggigator.so` | 565760 | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` | **= odrzucony `-063.so`** |
| `autorun/autorun.sh` | 822 | `a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517` | po `package-pad-cleanup.py` |
| `autorun/autorun.json` | 247 | `a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b` | identyczny z QA i ze stagingiem |
| `autorun/pads-off.sh` | 1055 | `a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9` | **= `scripts/pads-off.sh` z repo** |

**`build/arpeggigator-qa.zip`:**
| Plik w paczce | Rozmiar | SHA-256 zawartości | Uwaga |
|---|---:|---|---|
| `autorun/autorun.check` | 64 | `54eaadb02aee3d7b7436472784dbe227d9d0cfeca0f625df294328c8b87f169d` | treść `03382278…` (stary SDK-check) |
| `autorun/arpeggigator.so` | 569640 | `f265170f8b416712066044101cfcc04e5961339efb9c8ccd7cdd092628ba0967` | **nie pasuje do ŻADNEgo pliku `build/*.so`** |
| `autorun/autorun.sh` | 587 | `fddbe1c59ec4687a614f75d57777d10e33723f91f69d104bd9e2db97cad0da9f` | surowy SDK, bez cleanupu |
| `autorun/autorun.json` | 247 | `a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b` | identyczny jak wyżej |

**Staging `build/pkg/autorun/` (PROVEN, `shasum -a 256 build/pkg/autorun/*`):**
```
9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d  build/pkg/autorun/arpeggigator.so
54eaadb02aee3d7b7436472784dbe227d9d0cfeca0f625df294328c8b87f169d  build/pkg/autorun/autorun.check
a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b  build/pkg/autorun/autorun.json
fddbe1c59ec4687a614f75d57777d10e33723f91f69d104bd9e2db97cad0da9f  build/pkg/autorun/autorun.sh
```
`build/pkg/` nie zawiera `pads-off.sh` — to staging **przed** cleanupem, zgodne z oczekiwaniem (PROVEN).

### 2.4 Integralność archiwów (PROVEN)
```
unzip -t build/arpeggigator.zip     → No errors detected in compressed data of build/arpeggigator.zip.
unzip -t build/arpeggigator-qa.zip  → No errors detected in compressed data of build/arpeggigator-qa.zip.
```
CRC i kompresja poprawne. `unzip -lv build/arpeggigator.zip` potwierdza 6 wpisów, 48% kompresji, `pads-off.sh` stored (0%).

### 2.5 Mapowanie środka zipu na pliki build/ (PROVEN, skrypt Python w pamięci)
```
build/arpeggigator.zip          9beaf588… → matches=['build/arpeggigator.so','build/arpeggigator-063.so']
build/arpeggigator-061.zip      6e0eac84… → matches=['build/arpeggigator-061.so']
build/arpeggigator-062.zip      cf71383c… → matches=['build/arpeggigator-062.so']
build/arpeggigator-final.zip    d76837d4… → matches=['build/arpeggigator-final.so']
build/arpeggigator-stage-d.zip  0a8a60c0… → matches=['build/arpeggigator-stage-d.so']
build/arpeggigator-qa.zip       f265170f… → matches=[]   ← orphan
```

---

## 3. Symbol `g_probe`

**Komenda (PROVEN):**
```
nm -D build/arpeggigator-060.so build/arpeggigator-062.so build/arpeggigator-063.so build/arpeggigator-final.so build/arpeggigator.so | grep -i g_probe
nm build/arpeggigator.elf | grep -i g_probe
objdump -T build/arpeggigator-060.so build/arpeggigator-063.so build/arpeggigator.so | grep -i g_probe
```

**Wynik (PROVEN):**
```
build/arpeggigator-060.so:  00000000000ba080 D g_probe
build/arpeggigator-062.so:  00000000000b87f8 D g_probe
build/arpeggigator-063.so:  00000000000b8170 D g_probe
build/arpeggigator-final.so:00000000000b8170 D g_probe
build/arpeggigator.so:      00000000000b8170 D g_probe
build/arpeggigator.elf:     00000000000b8170 D g_probe
--- objdump -T ---
00000000000ba080 g    DO .data  0000000000000070  g_probe   (-060.so)
00000000000b8170 g    DO .data  0000000000000070  g_probe   (-063.so)
00000000000b8170 g    DO .data  0000000000000070  g_probe   (arpeggigator.so)
```
`grep -a -c g_probe` daje `1` w każdym `.so` i w `build/pkg/autorun/arpeggigator.so`, `2` w `.elf`.

**Definicja w źródle (PROVEN):** `src/probe.rs:106` → `g_probe: ProbeDiagnostics = ProbeDiagnostics::new()` wewnątrz `static` z `#[linkage]`/eksportem (użycia w `src/probe.rs:77–86, 167–173`).

**Wniosek PROVEN:** symbol `g_probe` jest obecny i wyeksportowany we **wszystkich** buildach `.so` oraz w paczce (`autorun/arpeggigator.so` = `-063.so`). Jest to globalny symbol danych w sekcji `.data`, rozmiar `0x70` = 112 bajtów. Wymóg SDK `--require-symbol g_probe` (`scripts/build.sh:69`) jest spełniony.

**Uwaga (INFERRED):** obecność symbolu **nie** dowodzi poprawności jego zawartości ani bezpieczeństwa wątków — to przedmiot analizy linia-po-linii innych agentów.

---

## 4. Guard firmware'u `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f`

### 4.1 Gdzie jest (PROVEN)

**W `autorun.sh` we WSZYSTKICH paczkach** (`unzip -p … autorun/autorun.sh | grep -c …` → `1` dla każdego z 6 zipów):
```
build/arpeggigator.zip          1
build/arpeggigator-qa.zip       1
build/arpeggigator-061.zip      1
build/arpeggigator-062.zip      1
build/arpeggigator-final.zip    1
build/arpeggigator-stage-d.zip  1
```
Dokładna linia w `arpeggigator.zip` (PROVEN):
```
STOCK="/usr/bin/MPC Sample"
FW="44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f"
[ "$(dd if="$STOCK" bs=4096 count=1 2>/dev/null | sha256sum | cut -d" " -f1)" = "$FW" ] || exit 0
```

**W kodzie źródłowym / SDK (PROVEN):**
- `/Users/fafi/Downloads/ArpeggiGator/scripts/build.sh:70`: `--firmware-sha256 44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f \`
- `vendor/mpc-sample-sdk/build-mod.sh:21` — documented jako `first-4KiB app hash for the autorun guard; required`
- `vendor/mpc-sample-sdk/build-mod.sh:93–96` — walidacja: 64 znaki hex, inaczej `exit 2`
- `vendor/mpc-sample-sdk/build-mod.sh:226–227` — wstrzyknięcie `FW="…"` i warunku `dd … | sha256sum … = "$FW" || exit 0` do generowanego `autorun.sh`
- `docs/NATIVE-DISPATCH-RESEARCH.md:11` — hash pierwszych 4 KiB firmware'u MPC Sample 1.3.0.12, „matching the SDK build guard"
- `docs/logs/ag6/night-20261118/QA-FRESH-FROM-ZERO.md:40` — zgodny guard w świeżo postawionej QA

**W plikach binarnych (PROVEN, `grep -a -c <hash> build/*.so build/arpeggigator.elf` → `0` dla wszystkich):**
```
build/arpeggigator-060.so           count=0
build/arpeggigator-062.so           count=0
build/arpeggigator-063.so           count=0
build/arpeggigator-final.so         count=0
build/arpeggigator.so               count=0
build/arpeggigator.elf              count=0
```

**Wniosek PROVEN:** guard nie jest stringiem w `.so` — jest **warunkiem w launcherze `autorun.sh`**, co jest zgodne z konstrukcją SDK (skrypt sprawdza firmware przed `cp … /run/…`). Sam literal nie występuje jako stała w źródłach SDK, lecz jest przekazywany przez CLI z `scripts/build.sh`. **Brak guardu w `.so` nie jest brakiem elementu paczki.**

**Wniosek INFERRED:** mechanizm guardu jest „fail-safe" w sensie braku modyfikacji przy niezgodnym firmware — `exit 0` przed jakimkolwiek `cp`/`printf`. **Nie udowadnia to** stanu konsoli po wyjęciu karty SD (kanon 3) — to osobne zadanie analityczne.

---

## 5. `scripts/package-pad-cleanup.py` i `autorun.check` — spójność sumy kontrolnej

### 5.1 Algorytm (PROVEN — czytanie kodu)

`scripts/package-pad-cleanup.py` (hash pliku: `1d9a4b61f8f30b2126b92a6c339817660ea4f08cec9c108c2fd25a7bee032bc2`):
1. Otwiera `build/arpeggigator.zip`, czyta wszystkie wpisy do słownika (linie 8–10).
2. W `autorun/autorun.sh` szuka **dokładnie jednego** wystąpienia `b'systemctl daemon-reload\n'`; przy innej liczbie `raise SystemExit` (linie 13–15).
3. Wstawia przed nim 3 linie (linie 16–19):
   - `cp "$DIR/pads-off.sh" /run/arpeggigator-pads-off.sh || exit 0`
   - `chmod 755 /run/arpeggigator-pads-off.sh`
   - `printf '[Service]\nExecStopPost=-/run/arpeggigator-pads-off.sh\n' > /run/systemd/system/ac50.service.d/zz-arpeggigator-pads-off.conf`
4. **Suma kontrolna (linia 22):**
   ```
   check = sha256( autorun.json_bytes + zmodyfikowany_autorun.sh_bytes + b'f7bed1c9-7e90-4731-9194-c996666b4331' ).hexdigest().encode()
   ```
5. Zapisuje wynik jako 64 znaki ASCII do `autorun/autorun.check` (linia 23), przepakowuje zip i **dodaje `scripts/pads-off.sh` jako `autorun/pads-off.sh`** (linie 24–29).

`vendor/mpc-sample-sdk/build-mod.sh:259–262` robi to samo przed cleanupem:
```
blob = (root/"autorun.json").read_bytes() + (root/"autorun.sh").read_bytes()
blob += b"f7bed1c9-7e90-4731-9194-c996666b4331"
(root/"autorun.check").write_text(hashlib.sha256(blob).hexdigest())
```
→ **Sól `f7bed1c9-7e90-4731-9194-c996666b4331` jest identyczna w SDK i w cleanupie (PROVEN).** Spójność algorytmu zachowana; cleanup podmienia check, bo podmienia `autorun.sh`.

### 5.2 Weryfikacja niezależna (PROVEN)

⚠️ **Celowo NIE uruchomiono `python3 scripts/package-pad-cleanup.py`** — skrypt nadpisuje `build/arpeggigator.zip`, a zlecenie wyraźnie zabrania zmiany `build/`. Zamiast tego odtworzono jego logikę w pamięci (Python, tylko odczyt):

```
orig autorun.sh bytes: 587   sha256 fddbe1c59ec4687a614f75d57777d10e33723f91f69d104bd9e2db97cad0da9f
simulated autorun.sh bytes: 822   sha256 a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517
zip    autorun.sh bytes: 822   sha256 a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517
SIMULATED == ZIP autorun.sh: True
SIMULATED check: 26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4
ZIP stored check: 26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4
CHECK MATCH: True
JSON identical (pkg vs zip): True
```

Bezpośrednie przeliczenie z zawartości obu zipów (PROVEN):
```
=== build/arpeggigator.zip ===
stored check   : 26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4
recomputed     : 26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4
MATCH          : True
=== build/arpeggigator-qa.zip ===
stored check   : 033822780f40dbf093dce9ca879f69d17f40c3f126d7571968c2ddcd144c278c
recomputed     : 033822780f40dbf093dce9ca879f69d17f40c3f126d7571968c2ddcd144c278c
MATCH          : True
```

**Rozróżnienie (częsty błąd odczytu):** `9decad36…` z pkt 2.3 to **sha256 pliku** `autorun.check`, natomiast **treścią** tego pliku jest `26a97bfc…`. To treść jest sumą kontrolną i ona się zgadza. Analogicznie QA: hash pliku `54eaadbd…`, treść `03382278…`.

**Wniosek PROVEN:** suma kontrolna jest **spójna** w obu paczkach, a paczka produkcyjna jest **odtwarzalna bajt w bajt** ze stagingu `build/pkg/` + `scripts/pads-off.sh` + `package-pad-cleanup.py`. `pads-off.sh` w paczce (`a27f155b…`) jest identyczny z `scripts/pads-off.sh` (`a27f155b…`).

**Uwaga (INFERRED):** skrypt zakłada dokładnie jedno `systemctl daemon-reload`; gdyby SDK zmieniło generator, cleanup odmówi modyfikacji — to dobra cecha, ale wymaga ponownego sprawdzenia przy przepisywaniu kodu na v0.0.7.0 (kanon 2).

---

## 6. Tabela „plik źródłowy → plik w paczce → hash"

### 6.1 Pliki źródłowe (wszystkie 16 `.rs` w `src/`)

Wiązywanie (PROVEN, `grep -n '^\s*\(pub \)\?mod ' src/*.rs`): `src/lib.rs` deklaruje linie 7–21: `acordator`, `arp`, `diag`, `melody`, `mutator`, `native_audio`, `orbitator`, `playback`, `probe`, `scheduler`, `splash`, `pub(crate) mod splash_art` (linia 18), `tempo`, `transport_clock`, `transport_playback` — 15 modułów + sam `lib.rs` = 16 plików. Rust `crate-type = ["cdylib", "rlib"]` (`Cargo.toml:15–16`) kompiluje je do **jednego** `arpeggigator.so`.

| Plik źródłowy (pełna ścieżka) | SHA-256 źródła | Plik w paczce | SHA-256 w paczce | Status |
|---|---|---|---|---|
| `/Users/fafi/Downloads/ArpeggiGator/src/lib.rs` | `b0f93c20b56e9b786ce8cf9742422edb0d371b74a4559510b87c11d91318fa91` | `autorun/arpeggigator.so` | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` | PROVEN (mod root, `g_probe` eksportowany) |
| `…/src/acordator.rs` | `c37ac9d70ff12387583b3510742a148eccf647bed6872a2198e7ca5fd0764971` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:7; string „ACCORDATOR" w .elf) |
| `…/src/arp.rs` | `2cc9249d9c55f8236d9ff620f13b8e4890230019dab108545c4d2bfac9e2c53f` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:8; `src/arp.rs` w DWARF) |
| `…/src/diag.rs` | `b9ad3d5c7dc3d282b60c7096c89533e8bf6b3a339ff7a29c40c898b66500fdd1` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:9; namespace `diag` w symbolach .elf) |
| `…/src/melody.rs` | `d2c28b3ec5e34153c3e155e3ad06ec8735fccac5bba6169cdacb8ab4187b4227` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:10; symble `arpeggigator::melody::…` w .elf) |
| `…/src/mutator.rs` | `ed7cb29c2503b11fda3c0c953421c9693f728ce59b82108af980299a992c2670` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:11; string „MUTATOR" w .elf) |
| `…/src/native_audio.rs` | `280d77b0d922b60fab5671ac1e646d405e90b29f8447eba285e966363ff6b60f` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:12; `src/native_audio.rs` w DWARF, namespace w .elf) |
| `…/src/orbitator.rs` | `1ffe148f8a946ff254ab1437e87b8db724601101cbcaa051dda999375d87ce08` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:13; `src/orbitator.rs` w DWARF, string „ORBITATOR") |
| `…/src/playback.rs` | `ce3b4d02174a92a654b3dd9c501591c0ac05d1738354afd314eb9865481b5b64` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:14) |
| `…/src/probe.rs` | `bd48a492398e56160d6ca588bb5cade6e695ad717b95027b02f2bcd1282deac8` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:15; `g_probe` w `nm -D`) |
| `…/src/scheduler.rs` | `3e224da55511f337e70adc62de8548a1f66bff744989a8b68dcb0a5e6d9328c1` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:16; `src/scheduler.rs` w DWARF) |
| `…/src/splash.rs` | `b4a2c06d7b1f4ca15bb7224abcaf137ef6b4e25c97bfb9beaa718772a27f1aa9` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:17; namespace `splash` w .elf) |
| `…/src/splash_art.rs` | `eebc77f040ec641166b5e0e58f6e5fdb682f98d6a3adc42637f52e231d263cca` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:18; `crate::splash_art::GATOR_GROUPS` użyte w `src/splash.rs:1301`) |
| `…/src/tempo.rs` | `a408468779407335e418fd245a3d9e48d57573ae8cf6d806316d614bd60198e8` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:19; namespace `tempo` w .elf) |
| `…/src/transport_clock.rs` | `cd1b397899354709a30b367b510eddeb330de117c32b5cf382e57d6dcd33350c` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:20) |
| `…/src/transport_playback.rs` | `a56baa4dda31ab4b62ed8bebcd167b4dd199751f222d474a8420f32730e685f4` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN (mod w lib.rs:21; `src/transport_playback.rs` w DWARF) |

Wszystkie 16 plików `.rs` jest **zaangażowanych w budowę `.so`** — żaden nie jest osierocony (PROVEN przez deklaracje `mod`).

### 6.2 Konfiguracja, SDK, skrypty, zasoby

| Plik źródłowy | SHA-256 źródła | Plik w paczce | SHA-256 w paczce | Status |
|---|---|---|---|---|
| `/Users/fafi/Downloads/ArpeggiGator/Cargo.toml` | `1a27777eea8cb425d46622af728317c9a1fc0fdfd9ce3e2dda707e7d05770f6a` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN — hash **obecnego pliku roboczego `0.0.7-0`** (zmiana w trakcie pomiarów, patrz pkt 10); wersja z HEAD (`0.0.6-0`) ma hash `a203bc951462f01ce5a8dcfda53d5196622e0dceb5c8c34d3116ab6c488a3039` |
| `…/Cargo.lock` | `40722e170229aacaa4f013f76223ddc221ac5b1dc437abdff8e316cbbffcabb3` | `autorun/arpeggigator.so` | `9beaf588…` | PROVEN — hash **obecnego pliku roboczego `0.0.7-0`**; wersja z HEAD (`0.0.6-0`) ma hash `6785a8aa82149d78f00b36645c1520edf82a4e80ebea4a186417ef16ac26e060` |
| `…/vendor/mpc-sample-sdk/build-mod.sh` | `ba2bd44ac811ac0b3ae2311a82c9f33bbee45747943bb438e0023f8232e34dcb` | `autorun/autorun.sh`, `.json`, `.check` | `a387165d…`, `a085472e…`, treść `26a97bfc…` | PROVEN (generator) |
| `…/vendor/mpc-sample-sdk/ARPEGGIGATOR-PATCH.md` | `9af9c9f6cef15f5c879be4aed47f628d7e7e686dbea7d040c3e8d8fc84cfc40d` | — | — | dokumentacja, nie runtime |
| `…/scripts/build.sh` | `1be953a07ccc5557726c5bb66a8bddc3c632b1997d372e73149240853b716ebf` | (pośrednio) `autorun/*` | j.w. | PROVEN (przekazuje guard + `--require-symbol g_probe`) |
| `…/scripts/package-pad-cleanup.py` | `1d9a4b61f8f30b2126b92a6c339817660ea4f08cec9c108c2fd25a7bee032bc2` | `autorun/autorun.sh`, `autorun/pads-off.sh`, `autorun/autorun.check` | `a387165d…`, `a27f155b…`, `26a97bfc…` | PROVEN (symulacja bajt-w-bajt) |
| `…/scripts/pads-off.sh` | `a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9` | `autorun/pads-off.sh` | `a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9` | ✅ PROVEN identyczny |
| `…/scripts/sync-emulator-autorun.py` | `4eb764b1634ef3b4bb3f51d4428c8639068e168486b7ad8ac384c1eb3778d18c` | — | — | PROVEN: tylko EMULATOR (`build.sh:75–82`), nie paczka |
| `…/assets/arpeggigator-logo.svg` | `ad4c10b6329af937c7fa7ab8025f8b9db76e69bf8612539a1a3c0d1a1bc314da` | **BRAK** | — | PROVEN: brak `include_bytes!`/`include_str!` w `src/` |
| `…/assets/arpeggigator-splash.svg` | `b4834317e9a799a59f2911f644308f935126b7104d26cc894cf86836885502b1` | **BRAK** | — | PROVEN: j.w. (grafika generowana w kodzie: `splash_art.rs`) |

---

## 7. Kompletność elementów arpegiatora i lista plików „brakujących"

### 7.1 Wymagana lista vs rzeczywista — `build/arpeggigator.zip` (PROVEN)

| Element | Obecny? | Hash / uwaga |
|---|---|---|
| `.so` (silnik arpegiatora) | ✅ | `autorun/arpeggigator.so` = `9beaf588…` |
| `autorun.sh` | ✅ | `a387165d…`, 822 B, z guardem FW i `ExecStopPost` |
| `autorun.json` | ✅ | `a085472e…`, 247 B |
| `autorun.check` | ✅ | treść `26a97bfc…`, zgodna (pkt 5) |
| `pads-off.sh` | ✅ | `a27f155b…` = repo |
| `assets/` | ❌ (nie oczekiwane) | SVG nie są używane w runtime (brak `include_*!`) |
| katalog `autorun/` | ✅ | wpis 0 B |

**Wniosek PROVEN:** paczka produkcyjna zawiera **wszystkie pięć wymaganych plików runtime**. **Żaden element arpegiatora nie zaginął jako plik** — cała logika 16 modułów jest w `.so`.

### 7.2 `build/arpeggigator-qa.zip` (PROVEN)
- ❌ **BRAK `autorun/pads-off.sh`** — paczka nie przeszła przez `package-pad-cleanup.py`.
- ⚠️ `.so` = `f265170f8b416712066044101cfcc04e5961339efb9c8ccd7cdd092628ba0967` — nie odpowiada żadnemu plikowi `build/*.so` (por. pkt 2.5). Pochodzi z Nov 17 17:14 i jest starszą, niezidentyfikowaną kompilacją.
- ❌ Brak linii `ExecStopPost` w `autorun.sh` (587 B zamiast 822 B).

### 7.3 Pliki źródłowe, które nie trafiły do paczki jako osobne pliki (PROVEN, oczekiwane)

Wszystkie pliki `src/*.rs` (16), `Cargo.toml`, `Cargo.lock`, całe `vendor/mpc-sample-sdk/`, `scripts/build.sh`, `scripts/package-pad-cleanup.py`, `scripts/sync-emulator-autorun.py`, `assets/*.svg` — to **materiały budowlane / projektowe**, nie pliki runtime karty SD. Ich rola jest odwzorowana w tabeli 6.1–6.2. **Nie jest to brak arpegiatora.**

Wyjątek wymagający uwagi: `scripts/pads-off.sh` **został** włączony do paczki produkcyjnej jako `autorun/pads-off.sh` (PROVEN).

### 7.4 Inne obserwacje integralnościowe (PROVEN)
- `git status --porcelain=v1 -- src/ Cargo.toml Cargo.lock` → puste: zawartość źródeł odpowiada HEAD `b912eed`. Mtime `src/splash.rs` = Nov 18 17:09 (późniejszy niż build 12:54), ale **git zgłasza plik jako czysty** → zmiana czasu bez zmiany treści; nie dowodzi to rozjazdu kodu z buildem (INFERRED, wymagałoby rebuildu i porównania hashów, czego zlecenie zabrania).
- W chwili pierwszego pomiaru (`17:34:05 UTC`) `Cargo.toml:3` i `Cargo.lock:13` miały wersję **`0.0.6-0`** (PROVEN, `read` + `grep`). W trakcie moich pomiarów inny agent zmienił je na **`0.0.7-0`** i dodał wpis `## 0.0.7.0` do `CHANGELOG.md` (PROVEN, `date -u` → `17:38:10 UTC`, `git status` → ` M Cargo.lock`, ` M Cargo.toml`, ` M CHANGELOG.md`; mtime plików `18:35:27`/`18:35:31` czasu lokalnego). Szczegóły w pkt 10. **Niezależnie od tej zmiany paczka z `12:54` nie mogła być zbudowana z nowej wersji** (PROVEN).

---

## 8. Co jest PROVEN, a co pozostaje INFERRED / nieudowodnione

**PROVEN (pomiary w tym raporcie):** hashe 4 `.so` względem handoffu; hashy obu zipów (bez referencji); pełna lista i hashe zawartości paczek; `unzip -t`; obecność `g_probe` (`nm -D`, `objdump -T`, `nm`); obecność guardu w `autorun.sh` i w `build.sh`/SDK; brak guardu w binariach; algorytm sumy kontrolnej i jego spójność; odtwarzalność paczki z `build/pkg/`; identyczność `pads-off.sh`; mapa 16 modułów do `.so`; wersja `0.0.6-0` **w momencie pierwszego pomiaru** (później zmieniona przez innego agenta na `0.0.7-0` — pkt 10); tożsamość `.so` w zipie z `-063.so`.

**INFERRED (oznaczone przy twierdzeniach):** przypisanie `-063.so` do „odrzuconej wersji 0.0.6.3" na podstawie konwencji nazewnictwa i zapisu handoffu linia 68; „fail-safe" charakter guardu jako intencja projektowa; kompilacja wszystkich modułów do `.so` w oparciu o semantykę Rust (dla 6 modułów dodatkowo potwierdzono symbolami/DWARF).

**NIEUDOWODNIONE i poza zakresem JURKA (nie składam w tej sprawie wniosków):**
- analiza linia po linii kodu dotykającego `LD_PRELOAD`, `ac50.service`, `/run`, SysEx, ALSA i wątków (kanon 1 i 2, ostrzeżenia 3–5 handoffu);
- dowód, że po wyjęciu karty SD fizyczna MPC jest w stanie sprzed włożenia (kanon 3, pkt 3 handoffu);
- poprawność zachowań na fizycznym sprzęcie — tu używano audio emulatora `MPC_MOD_FEATURES=emulator-audio` (kanon 2, ostrzeżenie 9), a `Cargo.toml:10` wciąż deklaruje feature `emulator-audio`;
- weryfikacja zipów względem handoffu (handoff nie zawiera ich hashy).

---

## 9. WERDYKT

> **NIE NADAJE SIĘ DO DEPLOYU.**

**Powody (PROVEN, niezależne od analizy kodu):**

1. **Paczka produkcyjna zawiera odrzucony build.** `unzip -p build/arpeggigator.zip autorun/arpeggigator.so | shasum -a 256` → `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` = `build/arpeggigator-063.so` (i `build/arpeggigator.so`). Handoff (linie 66–68) wyraźnie mówi, że próby `0.0.6.1/0.0.6.2/0.0.6.3` zostały odrzucone przez fafi, a potwierdzony testami jest `0.0.6.0` = `34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b`. Co więcej, `-060.so` ma mtime `15:07`, czyli jest **nowszy** niż paczka (`12:54`) — paczka powstała przed finalizacją dobrego buildu. (INFERRED: tożsamość `-063.so` z odrzuconym wydaniem 0.0.6.3 na podstawie nazwy i handoffu — ale sam fakt użycia złego hashu jest PROVEN.)
2. **Paczka nie jest v0.0.7.0.** W chwili budowy paczki (`12:54`) repo miało wersję `0.0.6-0`; w trakcie moich pomiarów (`~17:35 UTC`) inny agent zmienił `Cargo.toml:3`/`Cargo.lock:13` na `0.0.7-0` i dopisał `CHANGELOG.md`, ale **nie zbudował jeszcze nowej paczki** — `build/arpeggigator.zip` ma wciąż mtime `12:54` i zawiera `-063.so`. Kanon 2 (cały kod na nowo) nie został jeszcze zrealizowany w zbadanej paczce. Patrz pkt 10.
3. **Brak referencyjnych hashy zipów w handoffie** — nie da się udowodnić, że `da392e0f…` / `d9eb01eb…` są „właściwe" (PROVEN luka dokumentacyjna).
4. **`build/arpeggigator-qa.zip` jest niekompletny i osierocony:** brak `pads-off.sh`, brak `ExecStopPost`, `.so` `f265170f…` bez odpowiednika w `build/`. Nie wolno go użyć jako wzorca.
5. **Nie udowodniono kanonu 3** (stan konsoli po wyjęciu karty) ani kanonu 1 (analiza linia po linii). Częściowa pozytywna racja: drop-iny w `autorun.sh` są pisane do `/run/systemd/system/ac50.service.d/` (PROVEN z treści skryptu), czyli lokalizacji nietrwałej — ale to **nie jest dowód** fizycznego zachowania (INFERRED i poza moim zakresem).

**Co jest w porządku (PROVEN, do zachowania po przepisaniu kodu):**
- hashe 4 `.so` z handoffu są bezbłędne;
- `g_probe` obecny w każdym buildzie i w paczce (`0x70` bajtów, `.data`);
- guard firmware'u poprawnie wstrzyknięty do `autorun.sh` (ale nie do `.so` — to zgodne z SDK);
- `autorun.check` policzony zgodnie z algorytmem SDK/cleanupu i **spójny** w obu paczkach;
- paczka produkcyjna jest odtwarzalna bajt-w-bajt z `build/pkg/` + `scripts/pads-off.sh`;
- `unzip -t` bez błędów; wszystkie 5 wymaganych plików runtime obecnych;
- wszystkie 16 plików `.rs` jest podłączonych do kompilacji (zero osieroconych modułów).

**Rekomendacja JURKA dla zespołu:** po przepisaniu kodu na v0.0.7.0 (`0.0.7-0`), zbudować paczkę od zera, **wpisać do handoffu hashe obu zipów**, upewnić się, że `.so` w `arpeggigator.zip` ma hash buildu zaakceptowanego testami (nie `9beaf588…`, nie `d76837d4…`), a `package-pad-cleanup.py` zastosować ponownie i zweryfikować `autorun.check` komendami z pkt 5.2. Dopiero wtedy przekazać paczkę do weryfikacji bezpieczeństwa fizycznego (kanon 1 i 3).

---

## 10. Aneks: zmiana wersji przez innego agenta w trakcie pomiarów (PROVEN)

Zgodnie z zasadą PROVEN vs INFERRED odnotowuję zmianę stanu repo, która nastąpiła **w trakcie** mojej pracy. Nie jest to moja zmiana — Jurek nie modyfikował żadnego pliku oprócz tego raportu.

| Moment (UTC) | Obserwacja | Komenda | Wynik |
|---|---|---|---|
| `17:34:05` | start pomiarów, HEAD `b912eed`, tracked clean | `date -u`; `git status --porcelain=v1` | brak pozycji ` M` dla `Cargo.*` |
| `17:34:05`–`17:35` | `Cargo.toml:3` = `0.0.6-0`, `Cargo.lock:13` = `0.0.6-0` | `read Cargo.toml`; `grep -n -A2 'name = "arpeggigator"' Cargo.lock` | `version = "0.0.6-0"` |
| `~17:35` (mtime `18:35:27`/`18:35:31` czasu lokalnego) | inny agent zaktualizował wersję i changelog | `git status --porcelain=v1 \| grep -v '^??'` | ` M CHANGELOG.md`, ` M Cargo.lock`, ` M Cargo.toml` |
| `17:38:10` | weryfikacja po fakcie | `date -u`; `grep -n '^version' Cargo.toml` | `3:version = "0.0.7-0"`; `13-version = "0.0.7-0"` |

**Hashe wersji (PROVEN):**
```
git show HEAD:Cargo.toml | shasum -a 256
  a203bc951462f01ce5a8dcfda53d5196622e0dceb5c8c34d3116ab6c488a3039  -     (0.0.6-0)
git show HEAD:Cargo.lock | shasum -a 256
  6785a8aa82149d78f00b36645c1520edf82a4e80ebea4a186417ef16ac26e060  -     (0.0.6-0)
shasum -a 256 Cargo.toml Cargo.lock   [obecny plik roboczy]
  1a27777eea8cb425d46622af728317c9a1fc0fdfd9ce3e2dda707e7d05770f6a  Cargo.toml   (0.0.7-0)
  40722e170229aacaa4f013f76223ddc221ac5b1dc437abdff8e316cbbffcabb3  Cargo.lock   (0.0.7-0)
```

`git diff -- CHANGELOG.md` pokazuje nowy wpis `## 0.0.7.0`, który m.in. stwierdza: „Hardware package is built with the standard `scripts/build.sh` path, i.e. WITHOUT `MPC_MOD_FEATURES=emulator-audio`" oraz „Full provenance table … and line-by-line hardware-safety analysis are recorded in `docs/VP-0.0.7.0-*.md`". To potwierdza, że koordynator oczekuje tego raportu jako części kompletu (PROVEN).

**Wpływ na werdykt: żaden.** Paczka `build/arpeggigator.zip` (mtime `12:54`, hash `da392e0f…`, `.so` = `9beaf588…`) powstała **przed** zmianą wersji i przed finalizacją `-060.so` (`15:07`). Zmiana metadanych w źródle nie aktualizuje już zbudowanego archiwum. Konieczny jest pełny rebuild i ponowny pomiar wszystkich hashy z pkt 1, 2 i 5.2.

**Weryfikacja, że Jurek nie naruszył zakazu (PROVEN):**
```
git status --porcelain=v1 -- src/     → (puste)
ls -la build/                          → bez zmian względem pierwszego odczytu
```
Jedynym plikiem zapisanym przeze mnie jest `/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-JUREK-INTEGRALNOSC.md`
(hash `a32b2b27e036af2b3ef7ee7c03715704dfb37b3f5e35c791c2860d4eb07dbb73` przed tym aneksem — po aneksie hash się zmienił).

---

*Jurek — liczby, hashe, integralność paczki. Żadnego pliku `src/`, `build/` ani `Cargo.*` nie zmieniono. 🪖☕*
