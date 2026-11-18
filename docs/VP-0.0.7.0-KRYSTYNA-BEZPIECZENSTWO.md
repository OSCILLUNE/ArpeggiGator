# VP-0.0.7.0 — KRYSTYNA: analiza bezpieczeństwa fizycznej MPC Sample

**Agent:** KRYSTYNA (mechanizmy, firmware, bezpieczeństwo `ac50`/SysEx), zespół Jeremiasza.
**Data/godzina startu:** `Wed Nov 18 17:32:07 UTC 2026`. **Data/godzina zakończenia pomiarów:** `Wed Nov 18 17:40:53 UTC 2026`.
**Repo:** `/Users/fafi/Downloads/ArpeggiGator`. **HEAD:** `b912eed1028c9b9c54a166b36433cd349e782be0` (PROVEN: `git rev-parse HEAD`).
**Zakres:** wyłącznie pliki w repo. **NIE** dotknięto maszyn DEV/QA, nie deployowano, nie rebootowano (zgodnie z poleceniem). Nie zmieniono żadnego pliku `src/` ani `build/` (PROVEN: `git status --short -- src Cargo.toml Cargo.lock CHANGELOG.md` pokazuje tylko ` M CHANGELOG.md`, ` M Cargo.lock`, ` M Cargo.toml` — zmiany wprowadzone przez innego agenta; `src/` i `build/` czyste).

---

## 0. Trzy zdania-kanon (powtarzam dosłownie)

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.**
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`).**
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.**

---

## 1. Metoda i konwencja dowodów

- **PROVEN** = twierdzenie poparte komendą podaną w tekście, pełną ścieżką, hashem lub dosłownym cytatem kodu z numerami linii, które sam odczytałem w tym raporcie.
- **INFERRED** = wniosek z kodu/dokumentacji/emulatora, którego **nie** mogę potwierdzić na fizycznej MPC (maszyn wirtualnych nie wolno było dotknąć).
- **NIEUDowodnione** = brak dowodu; traktuję jako blokadę deployu.
- Każde polecenie wykonane z katalogu `/Users/fafi/Downloads/ArpeggiGator`.

**Zastrzeżenie (PROVEN):** w repozytorium nie ma implementacji firmware'u MPC ani kodu mechanizmu autorunu konsoli. Mam tylko skrypty, kod modułu i logi z emulatora. Dlatego zachowanie fizycznej konsoli po wyjęciu karty **nie może** być udowodnione statyczną analizą kodu modułu.

---

## 2. Inwentarz zbadanego kodu (dowód, że przeczytano CAŁY kod)

### 2.1 `vendor/mpc-sample-sdk` — wszystkie pliki (PROVEN: `find vendor/mpc-sample-sdk -type f | sort | xargs shasum -a 256`)

| Plik | SHA-256 |
|---|---|
| `ARPEGGIGATOR-PATCH.md` | `9af9c9f6cef15f5c879be4aed47f628d7e7e686dbea7d040c3e8d8fc84cfc40d` |
| `Cargo.toml` | `63042900a4fcaf279601a43afd9629ba723e6820988e249bb6a24135e2752437` |
| `LICENSE` | `90edc9ba2cc4e8c47c8c40d7b621df28a9de7c35025570a8c061f290a23d84cd` |
| `README.md` | `8d69febdedec9018d5571dba222828374d982602ccd79eac804965aefdd7308e` |
| `build-mod.sh` | `ba2bd44ac811ac0b3ae2311a82c9f33bbee45747943bb438e0023f8232e34dcb` |
| `scripts/zig-cc-aarch64-linux-gnu` | `e8c74b442f1bbbf5fcd077d632f00be479ed57a31eafec18b1c8ae6305ca943d` |
| `src/audio.rs` | `6e114b979c7ad1960bd1a83820e0370f2506f957561c84f404166619363b9869` |
| `src/draw.rs` | `b2f32f575509f4c4fd72931e9422ac993adeeade93e0ba2ab590a62486a4c910` |
| `src/event.rs` | `ffc5f64dea982ceacaf629eeb09c98575a4f0e176db938022b46d2292f31269d` |
| `src/hook.rs` | `b4076d5c05e583deffeebfc38c0990e13d33f3b525f4fc2ee5ce7e4291b10f40` |
| `src/image.rs` | `e90d99cb5f943364a1a9063143226fe3101a9b2833addfe03fe1f68b6e721080` |
| `src/lib.rs` | `eb367faa0aa39db3edb980f7f72455f64b0fcd42ef94fc4de6d2b133835fcf0d` |
| `src/loader.rs` | `96021887d1d9f7ddd09bb7cc6121f1b81d0897164829cb1760535b6df7453e3d` |
| `src/midi.rs` | `d27557422fccb6a5486b9c9768ce6e1bcfd987dc64c4bb2915839a4eb897d357` |
| `src/platform/clock.rs` | `3e8ae6aa2d4631b02724ea19964c7a8c514430678d6730c7e5c70b29c13473c3` |
| `src/platform/control.rs` | `eabdc06a4a7ebee21a785dc8a81c116b16116272bebd101ab51c62ea957e90d9` |
| `src/platform/effect.rs` | `ef09fac14b75b6ac3ba6fbf0b7b03474ee9eb24d9958ea3dec7251682e259f99` |
| `src/platform/extract.rs` | `b7cd20a5a8bc0572459372a3ba70428e92add29f4028c32407258f2504cbf015` |
| `src/platform/lighting.rs` | `9c469b4f6e842f2a445014563dced393e82b6a500dc4183fe4ac51a82981ecfd` |
| `src/platform/mod.rs` | `5463fc62332117d36c13cb3aea3434aaf4e9cc5675a272c32b6971d66cac7b14` |
| `src/platform/pads.rs` | `b9c27c07bb20afa45cae33d5825a59a6d7fbd39d7b2caa8a69ac1079342ebc21` |
| `src/platform/page.rs` | `427386d71bb12d9e076657dae0ff7675623aeb3a6363cd4047f57d4109fd673e` |
| `src/platform/paths.rs` | `4c44e6c38a69b37407f371873129b19c3f04894a2c19a3dc5c0b88366ed677df` |
| `src/platform/sample.rs` | `f596d4b3fe2903e16ad0a6dcd5754c1c087c458e0bb72df4771c35780e83ebfe` |
| `src/platform/seed.rs` | `6aaef04f89a4e62a5259068eedab98cfc11094976a6800d9e175d6177936eee8` |
| `src/platform/selection.rs` | `9aa62be7a28e9773f77571cd58f67b2fa60adffde91a76732b111bb7a588d36e` |
| `src/platform/slice.rs` | `27313cd064b1d0dde1105e8bff787522957f83f41955b1f094cad1395d4b345a` |
| `src/platform/status.rs` | `ce776a4b4970928d979150360d6d0149ef2324265ebe6d3ebfd137e6e7e5bbdb` |
| `src/platform/trim.rs` | `93cad0fc81538c2f5a763989cc19b76993e29b3d2622248c107da61368ef20e0` |
| `src/platform/wav.rs` | `1110648ee901c6462d7844efe47abc0ab72988a82d17be8c8f1c854cb64c53b6` |
| `src/platform/writer.rs` | `be9aa76a46325bf180225061909fc314c29a4aebe23fb146e2b9836992ec41a4` |
| `src/render.rs` | `4780f51abf4fa406e86cfc6db7a511f25643c45401cb678e39667f7457a8b0a2` |
| `src/sites.rs` | `cb08f94e5b2cbc84b94b8fc6c6210c49802590c5b440e6d99b4f0daac7857dc3` |
| `src/state.rs` | `bdd4aaf0b6bb66975592af6ad586b5bf24d67645a24029411edfaadfe0ad47c1` |
| `src/task.rs` | `f5890dda6cff61f41142adf4312abbdf3271a9589ebe5f9ca5199e146770be6d` |

### 2.2 `src/` — wszystkie pliki (PROVEN: `find src -type f | sort | xargs shasum -a 256`)

| Plik | SHA-256 |
|---|---|
| `src/acordator.rs` | `c37ac9d70ff12387583b3510742a148eccf647bed6872a2198e7ca5fd0764971` |
| `src/arp.rs` | `2cc9249d9c55f8236d9ff620f13b8e4890230019dab108545c4d2bfac9e2c53f` |
| `src/diag.rs` | `b9ad3d5c7dc3d282b60c7096c89533e8bf6b3a339ff7a29c40c898b66500fdd1` |
| `src/lib.rs` | `b0f93c20b56e9b786ce8cf9742422edb0d371b74a4559510b87c11d91318fa91` |
| `src/melody.rs` | `d2c28b3ec5e34153c3e155e3ad06ec8735fccac5bba6169cdacb8ab4187b4227` |
| `src/mutator.rs` | `ed7cb29c2503b11fda3c0c953421c9693f728ce59b82108af980299a992c2670` |
| `src/native_audio.rs` | `280d77b0d922b60fab5671ac1e646d405e90b29f8447eba285e966363ff6b60f` |
| `src/orbitator.rs` | `1ffe148f8a946ff254ab1437e87b8db724601101cbcaa051dda999375d87ce08` |
| `src/playback.rs` | `ce3b4d02174a92a654b3dd9c501591c0ac05d1738354afd314eb9865481b5b64` |
| `src/probe.rs` | `bd48a492398e56160d6ca588bb5cade6e695ad717b95027b02f2bcd1282deac8` |
| `src/scheduler.rs` | `3e224da55511f337e70adc62de8548a1f66bff744989a8b68dcb0a5e6d9328c1` |
| `src/splash.rs` | `b4a2c06d7b1f4ca15bb7224abcaf137ef6b4e25c97bfb9beaa718772a27f1aa9` |
| `src/splash_art.rs` | `eebc77f040ec641166b5e0e58f6e5fdb682f98d6a3adc42637f52e231d263cca` |
| `src/tempo.rs` | `a408468779407335e418fd245a3d9e48d57573ae8cf6d806316d614bd60198e8` |
| `src/transport_clock.rs` | `cd1b397899354709a30b367b510eddeb330de117c32b5cf382e57d6dcd33350c` |
| `src/transport_playback.rs` | `a56baa4dda31ab4b62ed8bebcd167b4dd199751f222d474a8420f32730e685f4` |

**Uwaga (PROVEN):** powyższe hashe to stan roboczy w chwili raportu. Paczka z `12:54` została zbudowana **przed** zmianą wersji na `0.0.7-0` (patrz §9).

---

## 3. Aktualna paczka `build/arpeggigator.zip` — dane twarde

**PROVEN** (`shasum -a 256 build/arpeggigator.zip`, `unzip -l`, `unzip -lv`, `unzip -p … | shasum -a 256`):

- Pełna ścieżka: `/Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.zip`
- SHA-256 paczki: `da392e0f5c9df5a668c56f7295a73d06acfa41d40c018ff952eae8dc566f298f`
- `unzip -l`: 6 wpisów, suma rozpakowana `567948` B, skompresowana `297274` B (48%).

| Wpis | Długość | CRC-32 | Data | SHA-256 treści |
|---|---:|---|---|---|
| `autorun/` | 0 | `00000000` | 11-18-2026 12:54 | — |
| `autorun/autorun.check` | 64 | `20f1a398` | 11-18-2026 12:54 | `9decad364fd69b6fd909048631202994a7889f8c7c1783fbaa155a57a7145dad` |
| `autorun/arpeggigator.so` | 565760 | `0c31162f` | 11-18-2026 12:54 | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` |
| `autorun/autorun.sh` | 822 | `d8a64ccf` | 11-18-2026 12:54 | `a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517` |
| `autorun/autorun.json` | 247 | `06a1e02e` | 11-18-2026 12:54 | `a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b` |
| `autorun/pads-off.sh` | 1055 | `d0752930` | 11-17-2026 17:45 | `a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9` |

`autorun/arpeggigator.so` w zipie jest **identyczny bajt w bajt** z `/Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.so`, `build/arpeggigator-063.so` i `build/pkg/autorun/arpeggigator.so` (PROVEN: `cmp -s` → YES; `shasum -a 256` → ten sam hash `9beaf588…`).

`autorun/pads-off.sh` jest identyczny z `scripts/pads-off.sh` (PROVEN: hash `a27f155b…` w obu).

---

## 4. DOSŁOWNA treść `autorun/autorun.sh` po modyfikacji przez `scripts/package-pad-cleanup.py`

**PROVEN** (`unzip -p build/arpeggigator.zip autorun/autorun.sh | cat -n`; SHA-256 `a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517`, 822 B):

```sh
     1	#!/bin/sh
     2	# Generated by mpc-sample-sdk/build-mod.sh.
     3	DIR="$(cd "$(dirname "$0")" && pwd)"
     4	STOCK="/usr/bin/MPC Sample"
     5	FW="44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f"
     6	[ "$(dd if="$STOCK" bs=4096 count=1 2>/dev/null | sha256sum | cut -d" " -f1)" = "$FW" ] || exit 0
     7	cp "$DIR/arpeggigator.so" /run/arpeggigator.so || exit 0
     8	chmod 755 /run/arpeggigator.so
     9	mkdir -p /run/systemd/system/ac50.service.d
    10	printf '[Service]\nEnvironment=LD_PRELOAD=/run/arpeggigator.so\n' > /run/systemd/system/ac50.service.d/arpeggigator.conf
    11	cp "$DIR/pads-off.sh" /run/arpeggigator-pads-off.sh || exit 0
    12	chmod 755 /run/arpeggigator-pads-off.sh
    13	printf '[Service]\nExecStopPost=-/run/arpeggigator-pads-off.sh\n' > /run/systemd/system/ac50.service.d/zz-arpeggigator-pads-off.conf
    14	systemctl daemon-reload
    15	systemctl restart ac50
```

**Staging przed cleanupem** — `/Users/fafi/Downloads/ArpeggiGator/build/pkg/autorun/autorun.sh` (PROVEN, `cat -n`, 12 linii, SHA-256 `fddbe1c59ec4687a614f75d57777d10e33723f91f69d104bd9e2db97cad0da9f`):

```sh
     1	#!/bin/sh
     2	# Generated by mpc-sample-sdk/build-mod.sh.
     3	DIR="$(cd "$(dirname "$0")" && pwd)"
     4	STOCK="/usr/bin/MPC Sample"
     5	FW="44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f"
     6	[ "$(dd if="$STOCK" bs=4096 count=1 2>/dev/null | sha256sum | cut -d" " -f1)" = "$FW" ] || exit 0
     7	cp "$DIR/arpeggigator.so" /run/arpeggigator.so || exit 0
     8	chmod 755 /run/arpeggigator.so
     9	mkdir -p /run/systemd/system/ac50.service.d
    10	printf '[Service]\nEnvironment=LD_PRELOAD=/run/arpeggigator.so\n' > /run/systemd/system/ac50.service.d/arpeggigator.conf
    11	systemctl daemon-reload
    12	systemctl restart ac50
```

**Różnica to dokładnie 3 wstawione linie 11–13.** PROVEN z `scripts/package-pad-cleanup.py`:

```
    13	needle = b'systemctl daemon-reload\n'
    14	if script.count(needle) != 1:
    15	    raise SystemExit('unexpected SDK autorun script; refusing to modify package')
    16	addition = (b'cp "$DIR/pads-off.sh" /run/arpeggigator-pads-off.sh || exit 0\n'
    17	            b'chmod 755 /run/arpeggigator-pads-off.sh\n'
    18	            b"printf '[Service]\\nExecStopPost=-/run/arpeggigator-pads-off.sh\\n' > /run/systemd/system/ac50.service.d/zz-arpeggigator-pads-off.conf\n")
    19	script = script.replace(needle, addition + needle)
    21	check_info, _ = entries['autorun/autorun.check']
    22	check = hashlib.sha256(entries['autorun/autorun.json'][1] + script + b'f7bed1c9-7e90-4731-9194-c996666b4331').hexdigest().encode()
```

**Spójność sumy kontrolnej (PROVEN, odrębny skrypt Python w terminalu):**

- `autorun.check` w zipie zawiera dokładnie `26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4`.
- `sha256(autorun.json || zmodyfikowany autorun.sh || "f7bed1c9-7e90-4731-9194-c996666b4331")` daje `26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4` → **MATCH = True**.
- Dla stagingu: `033822780f40dbf093dce9ca879f69d17f40c3f126d7571968c2ddcd144c278c` → **MATCH = True**.
- Algorytm pochodzi z `vendor/mpc-sample-sdk/build-mod.sh:260-262`:

```
   260	blob = (root / "autorun.json").read_bytes() + (root / "autorun.sh").read_bytes()
   261	blob += b"f7bed1c9-7e90-4731-9194-c996666b4331"
   262	(root / "autorun.check").write_text(hashlib.sha256(blob).hexdigest())
```

**INFERRED:** że fizyczna MPC 1.3.0.12 akceptuje tak policzony `autorun.check` — znamy algorytm tylko z kodu SDK i zachowania emulatora, nie z firmware'u konsoli.

---

## 5. Analiza linia po linii: instalacja modułu, `LD_PRELOAD`, drop-iny

### 5.1 Skrypt generujący (PROVEN — `vendor/mpc-sample-sdk/build-mod.sh:221-246`)

```
   221	lines = [
   222	    "#!/bin/sh",
   223	    "# Generated by mpc-sample-sdk/build-mod.sh.",
   224	    'DIR="$(cd "$(dirname "$0")" && pwd)"',
   225	    'STOCK="/usr/bin/MPC Sample"',
   226	    f'FW="{firmware}"',
   227	    '[ "$(dd if="$STOCK" bs=4096 count=1 2>/dev/null | sha256sum | cut -d" " -f1)" = "$FW" ] || exit 0',
   228	    f'cp "$DIR/{name}.so" /run/{name}.so || exit 0',
   229	    f'chmod 755 /run/{name}.so',
   230	    'mkdir -p /run/systemd/system/ac50.service.d',
   231	    f'printf \'[Service]\\nEnvironment=LD_PRELOAD=/run/{name}.so\\n\' > /run/systemd/system/ac50.service.d/{name}.conf',
   242	    ...
   243	    'systemctl daemon-reload',
   244	    'systemctl restart ac50',
   245	])
   246	(root / "autorun.sh").write_text("\n".join(lines) + "\n")
```

`--firmware-sha256` jest wymagany przy `--pack` i musi mieć 64 znaki hex (PROVEN: `build-mod.sh:92-97`). Wartość wstrzyknięta przez `scripts/build.sh:70`:

```
    70	    --firmware-sha256 44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f \
```

### 5.2 `autorun.sh` — linia po linii (PROVEN, numery jak w §4)

| Linia | Co robi | Gdzie zapisuje | Ryzyko / uwaga |
|---|---|---|---|
| 1 | interpreter `/bin/sh` | — | PROVEN |
| 2 | komentarz generatora | — | PROVEN |
| 3 | `DIR` = katalog skryptu na karcie SD | — | PROVEN. Ścieżka karty to zwykle `/media/MPCSAMPLE/autorun` (PROVEN dla emulatora: `scripts/sync-emulator-autorun.py:35` `D=/media/MPCSAMPLE/autorun`; INFERRED dla fizycznej karty) |
| 4 | `STOCK="/usr/bin/MPC Sample"` | odczyt | PROVEN |
| 5 | stała `FW` z 64 hex | — | PROVEN |
| 6 | **guard**: `dd … bs=4096 count=1 | sha256sum` ≠ `FW` → `exit 0` | tylko odczyt | PROVEN. Fail-safe: przy braku `sha256sum`/pliku albo innym firmware skrypt kończy bez zmian (INFERRED co do zachowania `exit 0` w autorunie) |
| 7 | `cp … /run/arpeggigator.so \|\| exit 0` | **/run** | PROVEN. Jedyny krok kopiujący binarkę; brak zapisu na eMMC |
| 8 | `chmod 755 /run/arpeggigator.so` | **/run** | PROVEN. Błąd `chmod` jest ignorowany (brak `set -e`) |
| 9 | `mkdir -p /run/systemd/system/ac50.service.d` | **/run** | PROVEN. To jest katalog runtime systemd, nie `/etc/systemd/system` |
| 10 | `printf … > …/arpeggigator.conf` → `[Service] Environment=LD_PRELOAD=/run/arpeggigator.so` | **/run** | PROVEN. `LD_PRELOAD` dotyczy **usługi `ac50`** (i jej procesów potomnych), nie całego systemu |
| 11 | `cp "$DIR/pads-off.sh" /run/arpeggigator-pads-off.sh \|\| exit 0` | **/run** | PROVEN. **UWAGA:** kopiowanie `pads-off.sh` jest PO zapisie `arpeggigator.conf` (linia 10), a PRZED `daemon-reload` (14). Jeśli plik zniknie z karty, skrypt wychodzi z kodem 0, zostawiając **sam drop-in z `LD_PRELOAD`** bez restartu — stan częściowy (PROVEN z kolejności; skutek INFERRED) |
| 12 | `chmod 755 /run/arpeggigator-pads-off.sh` | **/run** | PROVEN, błąd ignorowany |
| 13 | `printf … > …/zz-arpeggigator-pads-off.conf` → `[Service] ExecStopPost=-/run/arpeggigator-pads-off.sh` | **/run** | PROVEN. Prefiks `zz-`gwarantuje późniejszą kolejność ładowania drop-inów (INFERRED — standard systemd, potwierdzony w logach emulatora `DROPIN=arpeggigator.conf zz-arpeggigator-pads-off.conf`, np. `docs/logs/ag6/night-20261118/dev/A2_dry2/01_service_before.txt:8`) |
| 14 | `systemctl daemon-reload` | runtime systemd | PROVEN |
| 15 | `systemctl restart ac50` | proces `ac50` | PROVEN. Restartuje usługę z nowym `Environment=`; **nic nie zapisuje na dysk** |

**Brak `set -eu` w `autorun.sh` (PROVEN)** — tylko dwie linie mają `|| exit 0`. Błędy `mkdir`, `printf`, `chmod`, `daemon-reload`, `restart` nie przerywają skryptu. To zwiększa prawdopodobieństwo stanu częściowego.

**`autorun.json` (PROVEN, SHA-256 `a085472e…`):**

```
     5	        "path": "autorun.sh",
     6	        "askQuestion": false,
     7	        "message": "Start ArpeggiGator?",
```

`askQuestion: false` → skrypt jest przeznaczony do uruchomienia bez pytania użytkownika (INFERRED co do dokładnego zachowania firmware'u). Na fizycznej konsoli oznaczałoby to automatyczne wykonanie skryptu z karty SD po jej włożeniu.

### 5.3 Co robi drop-in `zz-arpeggigator-pads-off.conf` (PROVEN + INFERRED)

- Zawartość (PROVEN z linii 13): `[Service]\nExecStopPost=-/run/arpeggigator-pads-off.sh\n`.
- `ExecStopPost` jest wywoływany przy **każdym** zatrzymaniu `ac50`, nie tylko przy wyjęciu karty (INFERRED — semantyka systemd; PROVEN w kodzie, że nie ma warunku sprawdzającego kartę).
- Wiodący `-` w `ExecStopPost=-…` oznacza „ignoruj błąd" — jeśli skrypt nie znajdzie urządzenia, systemd nie zgłosi problemu (INFERRED — semantyka systemd).
- Drop-in nie modyfikuje `ac50.service` w `/etc`; `autorun.sh` nie zawiera ani jednej ścieżki `/etc` (PROVEN: pełna treść w §4).

---

## 6. Co robi `pads-off.sh` — SysEx i sprzęt

**PROVEN** (`scripts/pads-off.sh` = `autorun/pads-off.sh`, SHA-256 `a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9`):

```
     1	#!/bin/sh
     2	# Run after ac50 exits, while the raw MIDI control-surface device is available.
     3	card=$(sed -n 's/^ *\([0-9][0-9]*\) \[Surface *\].*/\1/p' /proc/asound/cards | head -n 1)
     4	[ -n "$card" ] || exit 1
     5	device="/dev/snd/midiC${card}D0"
     6	[ -c "$device" ] || exit 1
     7	printf '\360\107\177\134\145\000\004\000\000\000\000\000\367…\367' > "$device"
```

Analiza:

- **L. 3:** szuka w `/proc/asound/cards` pierwszej karty, której opis zaczyna się od `[Surface`. To jest **heurystyka z emulatora**, nie udokumentowany interfejs MPC (INFERRED dla fizycznego sprzętu — nie mam kodu firmware'u ani `alsa` karty fizycznej).
- **L. 4–6:** brak karty/urządzenia → `exit 1`. Ponieważ w `ExecStopPost` jest `-`, błąd jest ignorowany (INFERRED).
- **L. 7:** zapis **binarnych ramek SysEx bezpośrednio do urządzenia znakowego `/dev/snd/midiC<n>D0`**. To jest jedyny zapis do sprzętu poza procesem `ac50` w całym łańcuchu.
- **Dekodowanie ramek (PROVEN z oktalnych escape'ów + dokumentacja `docs/HANDOFF-LED-PADS-CLEANUP.md:23-25`):**

  `F0 47 7F 5C 65 00 04 <pad> 00 00 00 00 F7` — 16 ramek dla `pad = 0..15`.
  `docs/HANDOFF-LED-PADS-CLEANUP.md:23-25` podaje: „Panel emulatora (`panel/server.py`) emuluje sprzętowy mikrokontroler połączony przez UART. Odbiera ramki SysEx formatu: `F0 47 7F 5C 65 00 04 <pad_idx> <r> <g> <b> F7`", oraz „MCU nie posiada własnego timeoutu – pamięta ostatni ustawiony kolor RGB dla każdego z 16 padów, dopóki nie otrzyma jawnej ramki z zerowym kolorem".

- **Wniosek (INFERRED):** skrypt gasi diody 16 padów, wysyłając `r=g=b=0`. **Nie udowodniono**, że fizyczna MPC ma ten sam format ramek, że `Surface` jest tam nazwą karty, ani że MCU fizycznej konsoli zachowuje się identycznie jak panel emulatora. To jest bezpośredni zapis do kontrolera LED — potencjalnie najbardziej ryzykowny element całego łańcucha.
- **`docs/HANDOFF-LED-PADS-CLEANUP.md:11-16` (PROVEN)** dokumentuje obserwację w emulatorze: „diody LED padów potrafią pozostać zapalone w panelu emulatora, nawet gdy urządzenie przejdzie w stan całkowitego wyłączenia". Czyli **bez** tego skryptu stan „nie jak przed" był realny już w emulatorze. Skrypt jest próbą naprawy, nie dowodem naprawy na fizycznym sprzęcie.
- **Brak `atexit`/handlerów sygnałów (PROVEN: `grep -rnE "atexit|signal\(|SIGTERM"` w `src/` i SDK daje 0 trafień poza komentarzami).** `impl Drop for ArpeggiGator` w `src/lib.rs:750-753` wywołuje `extinguish_pads()`, ale `Drop` nie jest wywoływany przy `SIGTERM`/`SIGKILL` (zgodnie z `docs/HANDOFF-LED-PADS-CLEANUP.md:26-29` — PROVEN dla emulatora, INFERRED dla fizycznej MPC). Dlatego jedyną „siatką bezpieczeństwa" jest `ExecStopPost`.

---

## 7. Co mod robi w pamięci procesu — dlaczego to nie jest zapis na dysk

### 7.1 Ładowanie (PROVEN — `vendor/mpc-sample-sdk/src/loader.rs`)

```
   196	pub unsafe fn install(module: &'static mut dyn Mod) -> Result<(), InstallError> {
   197	    let Some(firmware) = Firmware::find() else {
   198	        return Err(InstallError::NotFound);
   199	    };
   200	    // SAFETY: verified image, loader runs once before application threads.
   201	    unsafe { image::set_base(firmware.base()) };
   202	    if !firmware_ok(firmware.base()) {
   203	        return Err(InstallError::WrongFirmware);
   204	    }
   206	    // Keep the ALSA taps linked; they are resolved by name by the app.
   207	    crate::audio::keep_exports();
   209	    if !unsafe { sites::install(firmware.base()) } {
   210	        return Err(InstallError::NoHooks);
   211	    }
   212	    if let Some(interceptor) = module.note_interceptor() {
   213	        let _ = NOTE_INTERCEPTOR.set(interceptor);
   214	    }
   215	    if let Some(observer) = module.surface_observer() {
   216	        let _ = SURFACE_OBSERVER.set(observer);
   217	    }
   218	    let hooks = module.audio();
   219	    let _ = AUDIO.set(hooks);
   220	    module.on_start();
   221	    let _ = MODULE.set(std::sync::Mutex::new(module));
   222	    Ok(())
```

Punkt wejścia to `.init_array` (PROVEN — `loader.rs:228-247`, `export_mod!`), czyli kod działa **wewnątrz procesu `ac50`**, przed startem wątków aplikacji.

### 7.2 Patchowanie kodu (PROVEN — `vendor/mpc-sample-sdk/src/hook.rs`)

```
     1	//! In-memory branch hooks.
     2	//!
     3	//! The SDK never touches the firmware on disk: it patches one instruction in
     4	//! the loaded image and keeps everything else in pages it maps next to the app.
```

`patch_site()` (`hook.rs:398-421`) robi `mprotect(PROT_READ|PROT_WRITE|PROT_EXEC)`, zapisuje 4 bajty przez `image::write`, czyta z powrotem i przywraca `PROT_READ|PROT_EXEC`. Mapowanie to `MAP_PRIVATE|MAP_ANONYMOUS|MAP_FIXED_NOREPLACE` (`hook.rs:423-447`). `image::write` używa `/proc/self/mem` (`image.rs:301-307`, `315-324`) — zapis do **przestrzeni adresowej własnego procesu**, nie do pliku. `Firmware::find()` identyfikuje obraz przez `dl_iterate_phdr` (`image.rs:242-291`), nie przez otwarcie pliku.

**PROVEN:** w kodzie nie ma `open()`/`create()` na `/usr/bin/MPC Sample`; jedynym „zapisem" jest `/proc/self/mem` i `mprotect`. **INFERRED:** zgodnie z semantyką Linuxa, zapis do `MAP_PRIVATE` jest copy-on-write i znika wraz z procesem; brak dowodu z fizycznej konsoli.

### 7.3 ALSA — przechwytywanie symboli (PROVEN — `vendor/mpc-sample-sdk/src/audio.rs`)

SDK eksportuje (przez `LD_PRELOAD`) cztery symbole ALSA niezależnie od featury `emulator-audio`:

```
   371	#[unsafe(no_mangle)]
   372	pub unsafe extern "C" fn snd_pcm_writei(pcm: usize, buffer: *const i32, size: u64) -> i64 {
   374	    let real = *REAL_WRITEI.get_or_init(|| look_up(c"snd_pcm_writei"));
   ...
   426	    let written = unsafe { write(pcm, buffer.cast::<u8>(), size) };
```

```
   609	#[unsafe(no_mangle)]
   610	pub unsafe extern "C" fn snd_pcm_readi(pcm: usize, buffer: *mut i32, size: u64) -> i64 {
```

```
   759	#[unsafe(no_mangle)]
   760	pub unsafe extern "C" fn snd_pcm_readn(pcm: usize, buffers: *mut *mut i32, size: u64) -> i64 {
```

```
   822	#[unsafe(no_mangle)]
   823	pub unsafe extern "C" fn snd_pcm_hw_params_set_channels(pcm: usize, params: usize, channels: u32) -> i32 {
```

`keep_exports()` (`audio.rs:839-851`) wymusza ich obecność w `.so`. **PROVEN z `nm -D build/arpeggigator.so`:** `snd_pcm_hw_params_set_channels`, `snd_pcm_readi`, `snd_pcm_readn`, `snd_pcm_writei` są eksportowane jako `T`. **Wniosek:** przechwytywanie ALSA jest w paczce **zawsze**, bez względu na `MPC_MOD_FEATURES`; hooki wywołują oryginał przez `RTLD_NEXT` (`audio.rs:132-141`) i nie zmieniają parametrów streamu (`set_channels_with` → `forward_set_channels`, `audio.rs:724-751`). **INFERRED:** brak zmiany parametrów nie został zmierzony na fizycznym sprzęcie.

### 7.4 Wątki (PROVEN)

- `tempo::start_monitor()` (`src/tempo.rs:28-72`) — jeden wątek `arpeggigator-tempo`, czyta `/data/TempProjects/Temp-Project.xpj` co 500 ms i wywołuje `diag::tick()`. **Tylko odczyt** (PROVEN: `tempo.rs:85-88` `File::open`; zapis w `tempo.rs:138-140` jest wewnątrz `#[cfg(test)]` od linii 125).
- `Task::spawn` (`vendor/mpc-sample-sdk/src/task.rs:57-82`) — wątek `mod-task`, ale w `src/` nie ma wywołań `Task::spawn` (PROVEN: `grep "Task::spawn" src/` → 0 wyników).
- `draw.rs:213`, `writer.rs:308/383`, `extract.rs:615`, `loader.rs:288`, `sites.rs:891` — wywołania `thread::spawn` wyłącznie w blokach `#[cfg(test)]` (PROVEN: granice `#[cfg(test)]` w `draw.rs:311`, `writer.rs:226`, `extract.rs:576`, `loader.rs:249`, `sites.rs:848`).
- `src/diag.rs` — atomiki z wątku audio (PROVEN: `note_on()` l. 46-53), zapis pliku z `tick()` (l. 56-107) wywoływanego z `on_paint` i z wątku tempo (PROVEN: `tempo.rs:63`).

---

## 8. ZMIANY TRWAŁE — tabela dowodowa

| Cel | Co się dzieje | Trwałość | Status |
|---|---|---|---|
| `/run/arpeggigator.so` | `cp`, `chmod 755` (autorun l. 7-8) | `/run` — według dokumentacji VM tmpfs | PROVEN zapis; **INFERRED** nietrwałość na fizycznej MPC |
| `/run/systemd/system/ac50.service.d/arpeggigator.conf` | `printf > …` (l. 10) | j.w. | PROVEN zapis; INFERRED nietrwałość |
| `/run/systemd/system/ac50.service.d/zz-arpeggigator-pads-off.conf` | `printf > …` (l. 13) | j.w. | PROVEN zapis; INFERRED nietrwałość |
| `/run/arpeggigator-pads-off.sh` | `cp`, `chmod` (l. 11-12) | j.w. | PROVEN zapis; INFERRED nietrwałość |
| `/etc/systemd/system/…` | **brak jakiegokolwiek zapisu** | — | **PROVEN** — pełna treść `autorun.sh` w §4 nie zawiera `/etc`; `grep -rn "/etc" scripts/ vendor/…/build-mod.sh src/` daje 0 trafień w kodzie produkcyjnym |
| `/usr/bin/MPC Sample` | tylko `dd … count=1` (odczyt); hooki patchują pamięć procesu przez `/proc/self/mem` | brak zapisu na plik | PROVEN (brak `open()` zapisu w kodzie); INFERRED (semantyka `MAP_PRIVATE`) |
| `/media/MPCSAMPLE/autorun/` | tylko `scripts/sync-emulator-autorun.py:34-45` (narzędzie deweloperskie do emulatora, nie część autorunu) | karta SD | PROVEN; na fizycznej karcie pliki znikają wraz z kartą |
| `/data/TempProjects/Temp-Project.xpj` | wyłącznie `File::open` (`src/tempo.rs:86`) | — | PROVEN — mod nie zapisuje projektu |
| `/tmp/arpeggigator-6-events.log` (+ `.1`, `.2`, `.3`) | **zapis 1 Hz, rotacja 4×128 KiB** (`src/diag.rs:14-15`, `89-107`) | `/tmp` — **nie udowodniono**, że tmpfs na fizycznej MPC | **PROVEN zapis; INFERRED trwałość.** To jedyny realny zapis poza `/run` |
| stderr → journal systemd | `eprintln!` w `diag.rs:85` | zależy od konfiguracji journald | PROVEN wywołanie; INFERRED trwałość |
| diody padów (kontroler MCU) | SysEx z `pads-off.sh` przy zatrzymaniu `ac50` | MCU „pamięta ostatni kolor" (`HANDOFF-LED-PADS-CLEANUP.md:25`) | PROVEN skrypt; **NIEUDowodnione** na fizycznym sprzęcie |

**Kluczowy wniosek (PROVEN):** twierdzenie z `CHANGELOG.md:8` — „leaves no persistent change on the console" — jest **sprzeczne z kodem**: `src/diag.rs:89-107` zapisuje pliki w `/tmp`. Nawet jeśli `/tmp` jest tmpfs, pliki istnieją w pamięci aż do rebootu (INFERRED).

---

## 9. Guard firmware'u `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f`

### 9.1 Gdzie jest używany (PROVEN — `grep -rn` w całym repo)

1. `scripts/build.sh:70` — `--firmware-sha256 …` dla `build-mod.sh`.
2. `vendor/mpc-sample-sdk/build-mod.sh:21-22, 92-97, 226-227` — wymagany przy `--pack`, walidacja 64 hex, wstrzyknięcie do `autorun.sh` jako `FW`.
3. `autorun.sh:5-6` (w paczce) — porównanie pierwszych 4 KiB `/usr/bin/MPC Sample`.
4. `build/pkg/autorun/autorun.sh:5`, `led-0049/source/build/pkg/autorun/autorun.sh:5`, `docs/qa_screens/.../guest-autorun.txt:5` — kopie.
5. `docs/NATIVE-DISPATCH-RESEARCH.md:9-12` — potwierdzenie, że extracted `/usr/bin/MPC Sample` (119 243 440 B) ma ten sam hash pierwszych 4 KiB.
6. **Nie** występuje w `.so` (PROVEN: `grep` w binariach nie zwraca trafień; zgodne z `docs/VP-0.0.7.0-JUREK-INTEGRALNOSC.md:36`).

### 9.2 Co gwarantuje (PROVEN)

- **Poziom skryptu:** jeśli pierwsze 4096 B pliku `/usr/bin/MPC Sample` ma inny SHA-256, `autorun.sh` wychodzi (`exit 0`) **przed** jakimkolwiek `cp`/`mkdir`/`printf`/`systemctl` (linia 6). To jedyna bramka przed instalacją.
- **Poziom SDK:** `FIRMWARE_SITES` w `loader.rs:16-55` zawiera **38 par (offset, oczekiwane słowo 32-bit)**; `firmware_ok()` (`loader.rs:166-176`) odrzuca obraz, jeśli choć jedno słowo się nie zgadza; `install()` zwraca `WrongFirmware` i nie wywołuje `sites::install()` (`loader.rs:202-204`).
- **Poziom hooków:** `sites::install()` (`sites.rs:64-113`) przygotowuje 18 site'ów, odrzuca prologi PC-relative, wycofuje zmiany przy niepowodzeniu i nie publikuje branchy bez `seal()`; `hook::commit()` (`hook.rs:329-362`) przywraca wszystkie zapatchowane słowa przy błędzie.
- **Poziom audio (tylko z featą):** `native_audio::install()` (`src/native_audio.rs:267-316`) sprawdza 3 dodatkowe sygnatury (`RENDER_OFFSET 0x18dbcc0`, `DISPATCH_OFFSET+4`, `PITCH_OFFSET+4`) i wychodzi, jeśli się nie zgadzają.

### 9.3 Czego NIE gwarantuje (INFERRED / NIEUDowodnione)

- Sprawdza tylko **pierwsze 4 KiB** pliku, nie cały 119 MB obraz (PROVEN z `dd … bs=4096 count=1`).
- To nie jest podpis kryptograficzny; kolizja lub świadomie zmieniony firmware z identycznymi 4 KiB i 38 słowami przeszłaby (INFERRED).
- Nie udowodniono, że na fizycznej MPC ścieżka `/usr/bin/MPC Sample` i narzędzie `sha256sum` są dostępne w momencie autorunu (INFERRED). Jeśli nie są → `exit 0` (fail-safe, ale mod nie działa).

---

## 10. `MPC_MOD_FEATURES=emulator-audio` — wpływ na kod dotykający sprzętu

### 10.1 Co jest za bramką featury (PROVEN — `grep -rn "cfg(feature" src/`)

- `src/lib.rs:335-336`: `#[cfg(feature = "emulator-audio")] native_audio::install();` — **jedyny** moment, w którym instalowany jest dodatkowy hook `render()` (patch kodu w `src/native_audio.rs:267-316`).
- `src/lib.rs:583-586`: `#[cfg(feature = "emulator-audio")] fn note_interceptor(&self) -> Option<…> { Some(native_audio::capture) }`.
- `src/lib.rs:128`, `255-259`, `478-483`, `496-543`, `576`, itd.: propagacja stanu do `native_audio::*`.
- `src/native_audio.rs:25-35` (`kill_all_voices`), `:320-337` (`render`), `:599-640` (`send`/note-on) — wywołania funkcji firmware'u przez `image::base() + offset`.

**PROVEN — brak featury ⇒ brak `native_audio::install()` ⇒ brak `render()` ⇒ brak `diag::note_on` ⇒ brak generowania nut.** Potwierdzenie z logów: w katalogach z hashusem `9beaf588…` (aktywna usługa) heartbeaty pokazują wyłącznie `steps=0`, np.:

- `docs/logs/ag6/20261118_125833_E1_baseline/*` → `steps=0`
- `docs/logs/ag6/20261118_131444_M2_mode_verify/_service.txt` → `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d  /run/arpeggigator.so`

Natomiast w katalogu z hashem `34a9ec26…` (`docs/logs/ag6/night-20261118/dev/A2_dry2/01_service_before.txt`) pojawia się `steps=666` — czyli build z aktywną ścieżką audio.

**Wniosek (PROVEN z logów + kodu, INFERRED co do przyczyny):** obecna paczka (`9beaf588…`) **nie** ma aktywnego `emulator-audio`. To jest zgodne z `CHANGELOG.md:7`, ale **sprzeczne z kanonem 2** („tu używaliśmy audio emulatora") rozumianym jako „obecna paczka to build emulatora". Należy to rozstrzygnąć przed wydaniem.

### 10.2 Co jest Niezależne od featury (PROVEN)

- ALSA `LD_PRELOAD` hooki w `vendor/mpc-sample-sdk/src/audio.rs` (§7.3) — zawsze w `.so`.
- `surface_observer` (`src/lib.rs:129-131`, `observe_surface` l. 96-171) — zawsze zainstalowany; tylko czyta status MIDI i aktualizuje atomiki.
- `lighting` — `extinguish_pads()` (`src/lib.rs:315-318`), hook `sites::lighting` (`sites.rs:201-220`) — zawsze.
- `playback.rs:1-4`: „This layer owns only scalar pad state and timing. Its events are commands for a future verified output adapter; **it does not call firmware or MIDI**." → **PROVEN: czysta logika, zero sprzętu.**
- `transport_playback.rs:1-4`: „Synchronizes note generation strictly to the MPC transport pulse grid. Queues pad changes to grid boundaries" → **PROVEN: czysta logika; brak `unsafe`/`image::`/`libc::` w pliku (PROVEN z grep).**
- `transport_clock.rs:22-44`: wyłącznie `image::read_*` z pamięci procesu (PROVEN).
- `splash.rs`/`splash_art.rs`: rysowanie po `Graphics` w pamięci procesu, bez zapisu na dysk (PROVEN z `unsafe { graphics.* }`).

### 10.3 Co trzeba ustalić dla buildu fizycznego (INFERRED — rekomendacja)

1. **Czy `emulator-audio` ma być w ogóle w buildzie fizycznym?** Kod pokazuje, że bez niego mod **nie generuje nut** (§10.1). Z featą mod patchuje dodatkowy site `render()` w firmware'ie i wywołuje `kill_all_voices()` oraz dispatcher nut z wątku audio — to jest najbardziej inwazyjna ścieżka i **nie została zweryfikowana na fizycznej MPC**. Kanon 2 mówi wprost: nie zakładać, że build emulatora jest buildem docelowym. Decyzja musi być jawna i oparta na teście fizycznym, nie na domyśle.
2. **Nawet build bez featury zawiera 4 ALSA interposery** (§7.3) — też wymagają weryfikacji na fizycznym sprzęcie.
3. Trzeba przebudować paczkę z czystego źródła wg kanonu 2 i ponownie uruchomić `scripts/package-pad-cleanup.py` (`scripts/build.sh:73`), a następnie zweryfikować hashe — bieżąca paczka nie spełnia tego wymogu (§11).

---

## 11. Dodatkowe blokady integralności (PROVEN — niezależne od mojej analizy kodu)

Z raportu `docs/VP-0.0.7.0-JUREK-INTEGRALNOSC.md` (pełna ścieżka: `/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-JUREK-INTEGRALNOSC.md`):

- **Linia 386:** paczka produkcyjna zawiera `9beaf588…` = `build/arpeggigator-063.so`; handoff (`docs/HANDOFF-0.0.7.0-PLAN.md:66-68`) mówi, że próby `0.0.6.1/0.0.6.2/0.0.6.3` zostały **odrzucone przez fafi**, a potwierdzony jest `0.0.6.0` = `34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b`. **PROVEN** (hash) / **INFERRED** (interpretacja „odrzucony").
- **Linia 387:** paczka z `12:54` powstała, gdy repo miało wersję `0.0.6-0`; zmiana na `0.0.7-0` nastąpiła ok. `17:35 UTC`, **po** zbudowaniu paczki. **PROVEN.** Moje pomiary potwierdzają: `Cargo.toml:3` = `version = "0.0.7-0"`, a mtime `build/arpeggigator.zip` = `Nov 18 12:54`.
- **Linia 389:** `build/arpeggigator-qa.zip` jest niekompletny (brak `pads-off.sh`, brak `ExecStopPost`, `.so` `f265170f…`) — nie wolno go użyć jako wzorca. **PROVEN.**
- **Linia 368:** w chwili budowy paczki nie istniały referencyjne hashe zipów w handoffie. **PROVEN.**

---

## 12. Odpowiedź na kanon 3: czy po wyjęciu karty MPC jest jak przed?

### Scenariusz A — wyjęcie karty **bez** restartu konsoli (PROVEN z mechanizmu)

- `ac50` nadal działa z `LD_PRELOAD=/run/arpeggigator.so` (linia 10 + restart 15).
- Drop-iny i skrypt nadal istnieją w `/run` (linie 9-13).
- `diag.rs` nadal dopisuje do `/tmp/arpeggigator-6-events.log` co 1 s (PROVEN).
- Diody padów mogą pozostać zapalone: `pads-off.sh` uruchamia się tylko przy zatrzymaniu `ac50` (PROVEN z `ExecStopPost`), a wyjęcie karty nie zatrzymuje usługi (INFERRED).
- **Wniosek: konsola NIE jest w stanie sprzed włożenia karty. PROVEN z kodu.**

### Scenariusz B — wyjęcie karty + restart (lub shutdown)

| Element | Co się dzieje | Dowód |
|---|---|---|
| Pliki w `/run` | Znikają **tylko jeśli `/run` jest tmpfs** | **INFERRED** — dla VM PROVEN w `docs/QA-DEPLOY-2026-11-17.md:8` („`/run` = tmpfs (zapisywalny)") i `:32` („Drop-in i `.so` są w `/run` → znikają po reboocie QA"). Dla fizycznej MPC **brak dowodu** |
| `/etc/systemd/system` | Nietknięty przez cały łańcuch | **PROVEN** (§8) |
| `ac50.service` | Po restarcie systemd czyta jednostkę z `/etc` (niezmienioną) | **INFERRED** (semantyka systemd + brak zapisu do `/etc`) |
| `LD_PRELOAD` | Znika wraz z drop-inem z `/run` | **INFERRED** jak wyżej |
| Log `/tmp/arpeggigator-6-events.log*` | **Pozostaje, jeśli `/tmp` jest na trwałym storage** | **PROVEN** zapis; **NIEUDowodniona** nietrwałość `/tmp` |
| Diody padów | `ExecStopPost` próbuje wysłać SysEx; `-` ignoruje błąd; MCU pamięta ostatni kolor | **PROVEN** skrypt + `HANDOFF-LED-PADS-CLEANUP.md:25`; **NIEUDowodnione** na fizycznej MPC |
| Sam mechanizm autorunu firmware'u | Brak kodu źródłowego w repo; nie wiem, czy firmware zapisuje np. znacznik „autorun executed", licznik, log | **NIEUDowodnione** |

### Werdykt analityczny

**NIE DA SIĘ UDOWODNIĆ**, że po wyjęciu karty SD fizyczna MPC jest w stanie jak przed włożeniem. Powody w kolejności wagi:

1. **Brak dowodu na fizyczną nietrwałość `/run` i `/tmp`** — cały „powrót do fabryki" opiera się na założeniu z **emulatora** (`docs/QA-DEPLOY-2026-11-17.md:8,32`), co jest INFERRED dla żelaza. Nie wolno było dotknąć maszyn, więc nie mogę tego zweryfikować.
2. **Istnieje realny zapis poza `/run`: `src/diag.rs:14,89-107` → `/tmp/arpeggigator-6-events.log{,.1,.2,.3}`.** To bezpośrednio przeczy twierdzeniu `CHANGELOG.md:8` o „no persistent change".
3. **Nie udowodniano poprawności SysEx na fizycznym kontrolerze LED** (`pads-off.sh:3-7`; format z `HANDOFF-LED-PADS-CLEANUP.md:23-25` pochodzi z emulatora). `ExecStopPost` ma `-`, więc błąd jest niewidoczny.
4. **Nie ma kodu firmware'u autorunu** — nie da się wykluczyć trwałych efektów samego mechanizmu uruchomienia skryptu z karty.
5. **Stan częściowy przy awarii** — `autorun.sh` nie ma `set -e`, a `cp pads-off.sh` (l. 11) następuje po zapisie `arpeggiator.conf` (l. 10): brak pliku na karcie zostawia drop-in `LD_PRELOAD` bez restartu (PROVEN z kolejności).
6. **Paczka jest niedopuszczalna także z powodów integralności (§11):** zawiera build `9beaf588…` oznaczony w handoffie jako odrzucony, zbudowany przed zmianą wersji na `0.0.7-0`, bez referencyjnych hashy.
7. **Nierozstrzygnięty kanon 2:** featura `emulator-audio` jest w kodzie (`Cargo.toml:10`, `src/lib.rs:335-336`); dane z logów sugerują, że obecna paczka jest bez niej, ale decyzja dla buildu fizycznego nie zapadła — a z featą wchodzi najbardziej inwazyjny hook `render()` + `kill_all_voices()`.

---

## 13. WERDYKT

> **NIE NADAJE SIĘ DO DEPLOYU.**

**Dlaczego (wprost):** z powodu punktów 1–7 w §12 — przede wszystkim dlatego, że **nie potrafię udowodnić** powrotu fizycznej konsoli do stanu sprzed włożenia karty (założenia o `/run` i `/tmp` pochodzą z emulatora), a w kodzie istnieje udowodniony zapis do `/tmp` oraz niezweryfikowany zapis SysEx do kontrolera padów. Dodatkowo badana paczka nie jest paczką v0.0.7.0 i zawiera build oznaczony jako odrzucony.

**Co musi się wydarzyć, aby zmienić werdykt (INFERRED — lista wymagań):**

1. Przepisać cały kod na nowo z czystego `b912eed` (kanon 2), z jawną decyzją o `emulator-audio` dla żelaza.
2. Przeprowadzić pełną analizę linia po linii także po przepisaniu (kanon 1) — niniejszy raport obejmuje kod stanu na `17:32–17:40 UTC`, nie przyszły rebuild.
3. Zbudować nową paczkę, ponownie uruchomić `scripts/package-pad-cleanup.py`, wygenerować referencyjne hashe i tabelę źródło→wpis→hash.
4. Udowodnić **na fizycznej MPC**: `/run` jest tmpfs, `/tmp` jest tmpfs (albo usunąć zapis `diag.rs` z produktu), diody wracają do stanu sprzed włożenia karty, `ac50` po restarcie ma `NRestarts=0`, brak drop-inów i brak `LD_PRELOAD`.
5. Udowodnić, że format ramek SysEx z `pads-off.sh` jest poprawny dla fizycznego kontrolera i że awaria skryptu jest widoczna (rozważyć usunięcie wiodącego `-` z `ExecStopPost`).

---

## A. Pełne ścieżki przywołane w raporcie

- `/Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.zip`
- `/Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.so`
- `/Users/fafi/Downloads/ArpeggiGator/build/arpeggigator-063.so`
- `/Users/fafi/Downloads/ArpeggiGator/build/pkg/autorun/autorun.sh`
- `/Users/fafi/Downloads/ArpeggiGator/scripts/build.sh`
- `/Users/fafi/Downloads/ArpeggiGator/scripts/build-emulator-audio.sh`
- `/Users/fafi/Downloads/ArpeggiGator/scripts/package-pad-cleanup.py`
- `/Users/fafi/Downloads/ArpeggiGator/scripts/pads-off.sh`
- `/Users/fafi/Downloads/ArpeggiGator/scripts/sync-emulator-autorun.py`
- `/Users/fafi/Downloads/ArpeggiGator/vendor/mpc-sample-sdk/build-mod.sh`
- `/Users/fafi/Downloads/ArpeggiGator/vendor/mpc-sample-sdk/src/{loader,hook,image,audio,sites,event,midi}.rs`
- `/Users/fafi/Downloads/ArpeggiGator/src/{lib,native_audio,playback,transport_playback,tempo,diag,transport_clock}.rs`
- `/Users/fafi/Downloads/ArpeggiGator/docs/QA-DEPLOY-2026-11-17.md`
- `/Users/fafi/Downloads/ArpeggiGator/docs/HANDOFF-LED-PADS-CLEANUP.md`
- `/Users/fafi/Downloads/ArpeggiGator/docs/HANDOFF-0.0.7.0-PLAN.md`
- `/Users/fafi/Downloads/ArpeggiGator/docs/NATIVE-DISPATCH-RESEARCH.md`
- `/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-JUREK-INTEGRALNOSC.md`
- `/Users/fafi/Downloads/ArpeggiGator/docs/qa_screens/2026-11-17-1619-qa-crash-private/guest-autorun.txt`
