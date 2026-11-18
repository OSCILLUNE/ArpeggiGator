# VP-0.0.7.0 — ERNIE: CROSS-CHECK NIEZALEŻNY (PROVEN vs INFERRED)

**Agent:** ERNIE (cross-check, PROVEN vs INFERRED) — zespół Jeremiasza
**Start wątku:** `Wed Nov 18 17:32:07 UTC 2026`
**Pomiary własne:** `Wed Nov 18 17:40:19 UTC 2026` → `Wed Nov 18 17:43:43 UTC 2026`
**Repo:** `/Users/fafi/Downloads/ArpeggiGator`, HEAD `b912eed1028c9b9c54a166b36433cd349e782be0`, branch `release/0.0.6.4`
**Zasada:** zero zaufania bez komendy. Każde poniższe twierdzenie ma podaną przeze mnie komendę i jej wynik. Tam, gdzie nie da się czegoś udowodnić komendą, jest wyraźnie `INFERRED`.
**Zakres:** TYLKO pliki repo. Nie dotykano maszyn DEV/QA, nie deployowano, nie restartowano, nie zmieniano `src/`, `build/`, `Cargo.*`. Jedyny zapisany plik: ten raport.

---

## TRZY ZDANIA-KANON (powtarzam przy każdej decyzji)

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.**
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (MPC_MOD_FEATURES=emulator-audio).**
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.**

---

## 0. Stan wyjściowy (PROVEN — własne komendy)

```
date -u                 → Wed Nov 18 17:40:19 UTC 2026
git rev-parse HEAD      → b912eed1028c9b9c54a166b36433cd349e782be0
git branch --show-current → release/0.0.6.4
git tag -l              → … v0.0.6.0, v0.0.6.1, v0.0.6.2 (BRAK v0.0.6.3, BRAK v0.0.7.0)
git status --short      →  M CHANGELOG.md /  M Cargo.lock /  M Cargo.toml + bardzo wiele '??'
```

Pełna treść zmian tracked (`git diff` — PROVEN): wyłącznie wersja `0.0.6-0` → `0.0.7-0` w `Cargo.toml:3` i `Cargo.lock:13` oraz nowy wpis `## 0.0.7.0` w `CHANGELOG.md`. `git diff --name-status HEAD` → dokładnie 3 pozycje. To potwierdza opis z aneksu Jurka (pkt 10 jego raportu).

---

## 1. Weryfikacja KLUCZOWYCH twierdzeń raportu Jurka — własne komendy

### 1(a) `autorun/arpeggigator.so` w `build/arpeggigator.zip` = `9beaf588…` — ✅ PROVEN

```
unzip -p build/arpeggigator.zip autorun/arpeggigator.so | shasum -a 256
→ 9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d  -
```

Tożsamość z plikami na dysku (PROVEN, `shasum -a 256 build/*.so`):

| Plik | SHA-256 (mój pomiar) |
|---|---|
| `build/arpeggigator-063.so` | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` |
| `build/arpeggigator.so` (bez sufiksu) | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` |
| `autorun/arpeggigator.so` w zipie | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` |

**Rozróżnienie (ważne dla pkt 4IT):** tożsamość hashu z `-063.so` jest **PROVEN**. Natomiast przypisanie `-063.so` do „odrzuconej wersji 0.0.6.3" jest **INFERRED** (konwencja nazwy + handoff linia 68: „poprzednie próby 0.0.6.1/0.0.6.2/0.0.6.3 zostały odrzucone przez fafi"). Żadna komenda w repo nie dowodzi, że plik nazwany `-063.so` faktycznie zawiera kod wydania 0.0.6.3.

`unzip -l build/arpeggigator.zip` (PROVEN): 6 wpisów, `autorun/arpeggigator.so` 565760 B, data `11-18-2026 12:54`. `unzip -t` → `No errors detected in compressed data of build/arpeggigator.zip.`

### 1(b) `build/arpeggigator-060.so` = `34a9ec26…` — ✅ PROVEN

```
shasum -a 256 build/arpeggigator-060.so
→ 34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b
```

Pełna tabela moich pomiarów (wszystkie zgodne z JURKIEM co do bitu):

| Plik | SHA-256 |
|---|---|
| `build/arpeggigator-060.so` | `34a9ec26949b98c7a3ec83562aea94af9ba7d9f35771687db0d3e929841a5b7b` |
| `build/arpeggigator-061.so` | `6e0eac84e2332ac3cdf03217269f4c45a12ceb0ae4f3d8066542a042c9896550` |
| `build/arpeggigator-062.so` | `cf71383ce821af7e14c44de1922f775d6111ba2f845070acb692011406fc641f` |
| `build/arpeggigator-063.so` | `9beaf5888f3f1d7a3c203bccc19e19e04b8949e481becd94e2686b3870a7309d` |
| `build/arpeggigator-final.so` | `d76837d4614ef95b4501b2c88172397be4cefaf9cdb35c310fd884abe92697ea` |
| `build/arpeggigator-stage-d.so` | `0a8a60c0ff6cc60424d12218dc74751ab0c31b0428550656dcdbf01535732981` |
| `build/arpeggigator.zip` | `da392e0f5c9df5a668c56f7295a73d06acfa41d40c018ff952eae8dc566f298f` |
| `build/arpeggigator-qa.zip` | `d9eb01ebe171129f1cb086b6d7ef59b364af89ac51cf715ca826985123ae1325` |

`ls -la build/` (PROVEN): `arpeggigator-060.so` mtime **Nov 18 15:07**, `arpeggigator.zip` mtime **Nov 18 12:54** → dobry build jest **nowszy** niż paczka. To jest twardy dowód, że paczka nie zawiera `-060.so`.

### 1(c) `g_probe` jest w buildach — ✅ PROVEN

```
nm -D build/arpeggigator-060.so build/arpeggigator-062.so build/arpeggigator-063.so \
      build/arpeggigator-final.so build/arpeggigator.so | grep -i g_probe
→ 00000000000ba080 D g_probe   (-060)
→ 00000000000b87f8 D g_probe   (-062)
→ 00000000000b8170 D g_probe   (-063)
→ 00000000000b8170 D g_probe   (-final)
→ 00000000000b8170 D g_probe   (arpeggigator.so)
nm build/arpeggigator.elf | grep -i g_probe → 00000000000b8170 D g_probe
objdump -T build/arpeggigator-060.so build/arpeggigator-063.so | grep -i g_probe
→ 00000000000ba080 g DO .data 0000000000000070  g_probe
→ 00000000000b8170 g DO .data 0000000000000070  g_probe
```

Symbol globalny w `.data`, rozmiar `0x70` = 112 B. Definicja: `src/probe.rs` (`g_probe: ProbeDiagnostics = …`). Wymóg `--require-symbol g_probe` istnieje w `scripts/build.sh:69` (PROVEN, `grep -n`).

**INFERRED (zgodnie z Jurkiem):** obecność symbolu nie dowodzi poprawności jego treści ani bezpieczeństwa wątków.

### 1(d) `autorun.check` się zgadza — ✅ PROVEN (niezależne przeliczenie)

```
unzip -p build/arpeggigator.zip autorun/autorun.check | shasum -a 256
→ 9decad364fd69b6fd909048631202994a7889f8c7c1783fbaa155a57a7145dad  -   (hash PLIKU)
unzip -p build/arpeggigator.zip autorun/autorun.check
→ 26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4   (TREŚĆ = suma kontrolna)
```

Niezależne przeliczenie (Python, tylko odczyt zipu; sól `f7bed1c9-7e90-4731-9194-c996666b4331`):

```
STORED   : 26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4
RECOMPUT : 26a97bfc955c678c7f5c5f03911d4734683d03da67f9c1ae24e10030323351f4
MATCH    : True
autorun.sh len: 822 | has ExecStopPost: True | daemon-reload occurrences: 1
```

Dla `build/arpeggigator-qa.zip` (PROVEN): treść checku `033822780f40dbf093dce9ca879f69d17f40c3f126d7571968c2ddcd144c278c`, przeliczona identycznie, `MATCH: True`; `.so` = `f265170f8b416712066044101cfcc04e5961339efb9c8ccd7cdd092628ba0967` (brak odpowiednika w `build/*.so`), `pads-off.sh` brak, `ExecStopPost` brak.

**Podsumowanie pkt 1:** wszystkie cztery kluczowe twierdzenia Jurka wytrzymały mój cross-check bez ani jednej rozbieżności liczbowej. Dodatkowo zweryfikowałem jego hashe 16 plików `src/*.rs`, `Cargo.toml`/`Cargo.lock` (robocze i z HEAD), `vendor/mpc-sample-sdk/build-mod.sh`, `ARPEGGIGATOR-PATCH.md`, `scripts/build.sh`, `scripts/package-pad-cleanup.py`, `scripts/pads-off.sh`, `scripts/sync-emulator-autorun.py` oraz obu SVG — **wszystkie identyczne** z podanymi w raporcie.

---

## 2. SPRZECIW: czy standard build naprawdę NIE kompiluje kodu audio?

### 2.1 Analiza `cfg(feature="emulator-audio")` (PROVEN)

```
grep -rn 'emulator-audio\|cfg(feature' src/
```

`emulator-audio` występuje **wyłącznie w `src/lib.rs`**, w 16 miejscach: linie **128, 255, 335, 478, 496, 501, 506, 511, 515, 524, 529, 534, 539, 543, 576, 583**. (Linia 183 to inny feature: `stage-arp-only`.) W `src/native_audio.rs` **nie ma ani jednego `cfg(feature=…)`** poza `#[cfg(test)]` w linii 696.

`Cargo.toml:10`: `emulator-audio = []`. `scripts/build.sh` **nie ustawia** `MPC_MOD_FEATURES`; robi to wyłącznie `scripts/build-emulator-audio.sh`, a `vendor/mpc-sample-sdk/build-mod.sh:123-124` dodaje `--features` tylko gdy zmienna jest ustawiona. To jest PROVEN.

**Co jest bramkowane (PROVEN, cytaty z `src/lib.rs`):**
- `336: #[cfg(feature = "emulator-audio")] native_audio::install();`
- `583: #[cfg(feature = "emulator-audio")] fn note_interceptor(&self) -> Option<…> { Some(native_audio::capture) }`
- `129: begin_press`; `257-258: set_mode / panic`; `480-481: panic / set_mode`; `497-544: setters`; `577: set_rate`.

**Co NIE jest bramkowane (PROVEN):**
- `12: mod native_audio;` — moduł jest zawsze częścią crate'u.
- `125, 268, 603: native_audio::set_active_sample_pad(…)` — zapisuje tylko dwa atomiki (`src/native_audio.rs:51-53`).
- `248: native_audio::is_transport_playing()` — tylko odczyt atomiku (`native_audio.rs:71-73`).

### 2.2 Symboli nie da się użyć jako prostego wykrywacza feature'u (PROVEN — SPRZECIW z oczekiwanym)

```
nm -D <build> | grep -i g_arp
```

| Build | `g_arp_audio_on` | `g_arp_audio_off` | `g_arp_mode` / `g_arp_transport_*` | `g_probe` |
|---|---|---|---|---|
| `build/arpeggigator-060.so` | `0xbba70` | `0xbba6c` | `0xbba74` / `0xbba78`, `0xbba80`, `0xbba88` | `0xba080` |
| `build/arpeggigator-063.so` | `0xb9aa0` | `0xb9a9c` | `0xb9aa4` / `0xb9aa8`, `0xb9ab0`, `0xb9ab8` | `0xb8170` |
| `build/audio-lab/arpeggigator-audio-lab.so` | `0xaa6d8` | `0xaa6d4` | **BRAK** | `0xa9508` |
| `build/transport-lab/arpeggigator-transport-lab.so` | `0xabb50` | `0xabb4c` | `0xac360` … | `0xaa720` |

To jest `#[unsafe(no_mangle)]` w `src/native_audio.rs:75-88`. **Wniosek PROVEN:** obecność `g_arp_audio_on/off` w ogóle nie odróżnia buildu z `emulator-audio` od zwykłego — są one w każdym `.so`, bo moduł jest kompilowany zawsze. Każda próba „udowodnienia" feature'u po obecności tych symboli byłaby błędem.

Co więcej, pliki `.map` dowodzą, że zwykły build **zawiera skompilowany kod audio**:

```
grep -E '12native_audio[0-9]+(render|capture)' build/arpeggigator.map
→ 937: ..._12arpeggigator12native_audio6render     (0x3ce84)
→ 939: ..._12arpeggigator12native_audio7capture   (0x3fbc0)
grep -E '12native_audio[0-9]+(render|capture)' build/audio-lab/arpeggigator-audio-lab.map
→ 921: ..._12arpeggigator12native_audio6render     (0x37a78)
→ 923: ..._12arpeggigator12native_audio7capture   (0x38830)
```

`grep 'note_interceptor' build/arpeggigator.map` → `0x46fb0`, ale to **domyślna metoda traitu SDK** (`vendor/mpc-sample-sdk/src/event.rs:300`: `fn note_interceptor(&self) -> Option<NoteInterceptor> { None }`), nie nadpisanie z `lib.rs:583`. Nie jest to dowód feature'u.

`audio-lab` pochodzi z **14.11 mtime** i nie ma statyków `g_arp_mode`/`g_arp_transport_*`, które są w źródle HEAD — czyli zbudowano go ze **starszej rewizji** `native_audio.rs`. Porównanie audio-lab vs zwykły build **nie jest więc miarodajnym testem obecności feature'u** dla aktualnego kodu. Zaznaczam to jako ograniczenie metody.

### 2.3 Co dokładnie włącza `emulator-audio` (PROVEN — czytanie kodu)

`src/native_audio.rs:267` `pub fn install()`:
- czyta `image::base()` i sprawdza sztywne sygnatury pod adresami `RENDER_OFFSET = 0x18dbcc0`, `DISPATCH_OFFSET + 4 = 0x211b094`, `PITCH_OFFSET + 4 = 0x27e4404`;
- używa `hook::HookEngine::new(unsafe { image::Firmware::from_base(base) })`, `hooks.prepare(RENDER_OFFSET, render …)`, `hooks.seal()`, `hooks.commit()` — czyli **patchuje kod firmware'u w pamięci procesu**;
- `render()` (`native_audio.rs:320+`) robi `core::mem::transmute::<usize, unsafe extern "C" fn…>` z surowych adresów i wywołuje funkcje firmware'u z ręcznie złożonym `PadPacket` (`PITCH`, `DISPATCH`, stałe `ENGINE_OFFSET 0x5c0`, `ENGINE_VTABLE 0x6e7f2d0`, `RENDER_VTABLE 0x6e3b220`, `KILL_ALL_VOICES 0x2118a20`, `PAD_SOURCE 0x7fff_fffa`);
- nagłówek pliku mówi wprost: `//! Experimental native audio path for the virtual MPC running 1.3.0.12.`;
- `scripts/build-emulator-audio.sh`: `# Build the opt-in audio experiment without packaging or installing it.` i `--require-symbol g_arp_audio_on --require-symbol g_arp_audio_off`.

`install()` nie występuje w `build/arpeggigator.map` (PROVEN, `grep` bez trafienia), co jest zgodne z tym, że jedyne wywołanie jest pod cfg i zostało usunięte w zwykłym buildzie. **INFERRED (silnie wsparte, ale nie 100% PROVEN):** `-063.so` w paczce został zbudowany bez `emulator-audio`; wnioskuję z braku `native_audio::install` w mapie o tym samym mtime (12:54) oraz z faktu, że standardowy `build.sh` nie ustawia feature'u. Nie potrafię tego udowodnić bezpośrednio z metadanych samego `.so`.

### 2.4 ODPOWIEDŹ WPROST

> **Czy build fizyczny ma mieć `emulator-audio`? — NIE. (PROVEN jako decyzja.)**

Uzasadnienie **PROVEN** (same fakty z repo):
1. feature aktywuje `native_audio::install()`, który patchuje kod firmware'u w locie, opierając się na sztywnych offsetach obrazu `1.3.0.12` (`src/native_audio.rs:267-316`);
2. autor pliku nazywa to ścieżką „**for the virtual MPC**", a skrypt nazywa to „**opt-in audio experiment**" buildowanym **osobno**, poza paczką;
3. kanoniczny `scripts/build.sh` (ten, który produkuje paczkę) feature'u nie ustawia; robi to wyłącznie `scripts/build-emulator-audio.sh` z osobnym katalogiem wyjściowym `build/audio-lab`;
4. kanon 2 handoffu wprost mówi: „Nie zakładać, że build emulatora jest buildem docelowym".

Uzasadnienie **INFERRED**: „uruchomienie tych patchy na fizycznej MPC uszkodzi konsolę" — nie mam komendy ani testu fizycznego, więc nie nazywam tego PROVEN. Decyzja „NIE" jest jednak udowodniona niezależnie od tego, czy skutek byłby natychmiastowy, czy opóźniony.

**Korekta do powszechnego uproszczenia (PROVEN):** „standard build nie kompiluje kodu audio" jest **FAŁSZEM w dosłownym brzmieniu** — moduł `native_audio` jest zawsze w crate'ie, liczniki `g_arp_audio_*` są eksportowane w każdym `.so`, a `render`/`capture` są w `.map` zwykłego buildu. Prawdziwe jest tylko: **standard build nie AKTYWUJE audio hooków** (`install()`, `note_interceptor()`, settery) — wywołania są wycięte przez `cfg`. Przy przepisywaniu kodu na v0.0.7.0 trzeba to rozróżniać, inaczej „usunięcie feature'u" zostanie mylnie uznane za „usunięcie kodu audio".

---

## 3. Ukryte fixy poza kodem źródłowym — poszukiwania

### 3.1 Co jest w repo i zgodne z git (PROVEN)

```
git status --porcelain=v1 -- vendor/ scripts/ src/ assets/
→ wyłącznie '??' dla nowych, nieśledzonych skryptów: scripts/ag6_*.py,
  scripts/build_final_deck.py, scripts/verify-v070.py, scripts/__pycache__/
  (ANI JEDNEGO ' M' w vendor/, src/, assets/, ani w śledzonych scripts/)

git diff --stat  → CHANGELOG.md | Cargo.lock | Cargo.toml  (tylko wersja 0.0.7-0)

git ls-files vendor/ | while read f; do porównaj shasum pliku z git show HEAD:$f; done
→ (puste) — WSZYSTKIE 35 plików vendor/ jest bajt w bajt identycznych z HEAD
```

Wniosek **PROVEN**: nie ma żadnej niezatwierdzonej modyfikacji SDK, źródeł ani skryptów względem `b912eed`. Nie ma ukrytego patcha w `vendor/`.

### 3.2 `vendor/mpc-sample-sdk/ARPEGGIGATOR-PATCH.md` (PROVEN)

Hash pliku: `9af9c9f6cef15f5c879be4aed47f628d7e7e686dbea7d040c3e8d8fc84cfc40d` (zgodny z JURKIEM; identyczny też w nieśledzonym backupie `backups/pre-test-20261117-135020/…`). Treść (cytat): „This directory vendors mpc-sample-sdk revision `87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf` (MIT). The local change adds the full MIDI status to surface events and an optional non-blocking surface observer … The optional real-time note interceptor runs on the native callback thread before the module mutex …".

To jest **udokumentowana, zatwierdzona w git zmiana lokalna SDK** — nie jest „ukrytym fixem". Jest częścią HEAD, więc przepisując kod na nowo trzeba jej świadomie użyć albo świadomie odrzucić (kanon 2), ale nie można jej nazwać niejawną.

Uwaga pobreżna (PROVEN): `diff -rq backups/pre-test-20261117-135020/ArpeggiGator/vendor/mpc-sample-sdk vendor/mpc-sample-sdk` → różni się tylko `src/draw.rs`. Ponieważ **bieżący** `vendor/` jest identyczny z HEAD, różnica dotyczy starej kopii zapasowej, nie aktualnego kodu.

### 3.3 Czego z repo NIE da się zweryfikować (INFERRED / poza zasięgiem)

```
find . -path ./.git -prune -o -name 'lab.py' -print   → (pusto)
```

`lab.py` **nie istnieje w tym repo** — leży w `virtual-mpc-sample[-qa]/`, czyli poza dozwolonym zakresem. Podobnie drop-iny w `/run/systemd/system/ac50.service.d/`, `/etc/`, stan `ac50` i zawartość karty SD są poza repo.

**Wniosek PROVEN o granicy wiedzy:** z samych plików repo **nie da się udowodnić**, że nie ma fixa wprowadzonego tylko w maszynie wirtualnej / systemie gościa / serwisie `ac50` (ostrzeżenie 7 handoffu). Mogę udowodnić tylko, że repo jest czyste. Twierdzenie „nie ma ukrytego fixu w VM" byłoby **INFERRED bez komendy** — więc go nie składam.

Nie uruchomiłem `scripts/verify-v070.py` (PROVEN z lektury: `grep -nE 'write|extract'` → zapisuje `docs/VP-0.0.7.0-VERIFY-OUTPUT.md` i rozpakowuje do `build/.verify-v070-extract`), bo zlecenie zabrania zmiany `build/`.

---

## 4. Ocena raportu Jurka — rozbieżności i „INFERRED jako PROVEN"

### 4.1 Rozbieżności stwierdzone

| # | Miejsce w raporcie Jurka | Ocena ERNIEGO |
|---|---|---|
| 1 | Linia 61: „`build/arpeggigator.so` jest bitowo identyczny z **odrzuconym** `-063.so` (PROVEN)"; linia 112: „**= odrzucony `-063.so`**"; tytuł pkt 9.1: „Paczka produkcyjna zawiera **odrzucony** build" | **Rozbieżność w oznaczaniu.** Hash identity jest PROVEN. Określenie „odrzucony" (= wersja 0.0.6.3) zostało przez samego Jurka uznane za INFERRED w pkt 8 (linia 370), ale w powyższych miejscach występuje jako fakt bez znacznika. Proszę o konsekwentne: „PROVEN: hash = `-063.so`; INFERRED: przypisanie do odrzuconego 0.0.6.3". |
| 2 | Tabela 6.1 (linie 295-313): każdy z 16 wierszy ma status „PROVEN" | **Rozbieżność w oznaczaniu.** W pkt 8 (linia 370) Jurek przyznaje, że kompilacja 6 modułów do `.so` opiera się na semantyce Rust (symbolami/DWARF potwierdzono tylko 6). Status „PROVEN" w tabeli jest więc zbyt mocny dla 10 modułów. Same hashe źródeł są PROVEN (zweryfikowałem je wszystkie). |
| 3 | Cały raport, zwłaszcza pkt 8 (linia 375) | Jurek ** jawnie wyłącza** z zakresu kwestię `emulator-audio` („poprawność zachowań na fizycznym sprzęcie — tu używano audio emulatora …"). To nie jest błąd merytoryczny, ale oznacza, że raport Jurka **nie może być użyty jako dowód decyzji „build fizyczny bez emulator-audio"**, mimo że `CHANGELOG.md` powołuje się na zestaw `docs/VP-0.0.7.0-*.md`. Decyzję udowadniam dopiero w pkt 2 tego raportu. |
| 4 | Werdykt pkt 9powód 5 (linia 390) | Jurek poprawnie mówi, że kanon 3 nieudowodniony. Moje badanie treści `autorun.sh` **wzmacnia** ten punkt: w skrypcie nie ma żadnego cleanupu przy wyjęciu karty (szczegóły w pkt 5). To nie jest sprzeczność, lecz rozszerzenie. |

### 4.2 Czego JUREK NIE pomylił (PROVEN — potwierdzam)

- Wszystkie hashe (4 `.so` z handoffu, 2 zipy, 16 źródeł, skrypty, SDK, assety) — **zero rozbieżności**.
- `unzip -l`, `unzip -t`, lista 6/5 plików — potwierdzone.
- `g_probe` w każdym buildzie, rozmiar `0x70` — potwierdzone.
- Rozróżnienie „hash pliku `autorun.check`" vs „treść `autorun.check`" — potwierdzone i jest poprawne.
- Spójność sumy kontrolnej w obu paczkach — potwierdzona niezależnym przeliczeniem.
- Luka w handoffie (brak referencyjnych hashy zipów) — potwierdzona.
- Aneks o zmianie wersji w trakcie pomiarów — potwierdzony moim `git diff` i `grep`.

**Werdykt o raporcie Jurka:** wytrzymuje cross-check w swoim zakresie (liczby, hashe, integralność). Nie wytrzymuje w dwóch miejscach oznaczania INFERRED jako PROVEN (pkt 4.1 #1 i #2) i nie rozstrzyga kanonu 2.

---

## 5. Co pozostaje nieudowodnione + werdykt końcowy

### 5.1 Stan konsoli po wyjęciu karty — kanon 3: **NIE UDOWODNIONY, są dowody luki**

Własna komenda — pełna treść `autorun.sh` z paczki:

```
unzip -p build/arpeggigator.zip autorun/autorun.sh
```

```
cp "$DIR/arpeggigator.so" /run/arpeggigator.so || exit 0
chmod 755 /run/arpeggigator.so
mkdir -p /run/systemd/system/ac50.service.d
printf '[Service]\nEnvironment=LD_PRELOAD=/run/arpeggigator.so\n' > …/arpeggigator.conf
cp "$DIR/pads-off.sh" /run/arpeggigator-pads-off.sh || exit 0
printf '[Service]\nExecStopPost=-/run/arpeggigator-pads-off.sh\n' > …/zz-…pads-off.conf
systemctl daemon-reload
systemctl restart ac50
```

```
grep -nE 'rm |trap|unload|stop|revert|cleanup' (tegoż autorun.sh) → BRAK trafień
```

**PROVEN:** skrypt instaluje `LD_PRELOAD` i drop-in w `/run` (lokalizacja nietrwała — to jedyna dobra wiadomość), ale **nie zawiera żadnej akcji wycofującej przy wyjęciu karty**. `ExecStopPost` dotyczy zatrzymania usługi `ac50`, nie wyjęcia nośnika. Zgadza się to z analizą Flinta (`docs/VP-0.0.7.0-FLINT-KOD-LINIA-PO-LINII.md:505`): mod pozostaje aktywny, a łaty w pamięci firmware'u znikają dopiero przy wyjściu procesu/reboocie. **Kanon 3 pozostaje NIEUDowodniony** — a konkretnie: nie ma drogi przez kod dostanu „jak przed". To jest **PROVEN jako brak mechanizmu**, natomiast zachowanie fizycznej MPC po wyjęciu karty pozostaje **INFERRED** (brak testu fizycznego, poza zakresem).

### 5.2 Lista nieudowodnionego

1. **Kanon 1** — pełna analiza linia po linii kodu dotykającego SysEx/ALSA/wątków/repaintu wykracza poza mój zakres; częściowo pokrywa ją Flint, ale nie ja.
2. **Kanon 3** — patrz 5.1: brak mechanizmu wycofania; fizyczny efekt niezmierzony.
3. **Wersja/feature `-063.so`** — brak `native_audio::install` w mapie to silna, ale **INFERRED** wskazówka, że zbudowano go bez `emulator-audio`.
4. **Szkodliwość `emulator-audio` na fizycznym sprzęcie** — **INFERRED** ( zakaz dotykania fizycznej MPC). Decyzja „NIE włączać" jest natomiast PROVEN z pkt 2.4.
5. **Zmiany poza repo** (`lab.py`, VM, `ac50`, drop-iny) — nieweryfikowalne z repo (pkt 3.3).
6. **Paczka v0.0.7.0** — nie istnieje. `build/arpeggigator.zip` ma mtime 12:54 i zawiera `9beaf588…`, podczas gdy wersja w źródle to już `0.0.7-0`. Wymagany pełny rebuild.

### 5.3 WERDYKT KOŃCOWY

> **Decyzja „build fizyczny BEZ emulator-audio" jest UDOWODNIONA (PROVEN jako decyzja konstrukcyjna) — ale z ważnym zastrzeżeniem: wyłączenie feature'u wyłącza AKTYWACJĘ hooków audio, nie usuwa kodu audio z binarnego `.so`. Musi to być wyraźnie napisane w specyfikacji przepisanego kodu v0.0.7.0.**
>
> **Raport Jurka wytrzymuje cross-check w zakresie liczb i hashy (zero rozbieżności), ale nie jest wolny od błędów klasyfikacji: w 3 miejscach przedstawia jako PROVEN twierdzenie „odrzucony -063", które sam w pkt 8 klasyfikuje jako INFERRED, a w tabeli 6.1 daje status PROVEN modułom, dla których w pkt 8 przyznaje INFERRED. Nie rozstrzyga też kanonu 2.**
>
> **NIE NADAJE SIĘ DO DEPLOYU.** Powody twarde (PROVEN): (1) paczka zawiera `9beaf588…` = `-063.so`, nie zaakceptowany `34a9ec26…`; (2) paczka jest sprzed zmiany wersji na `0.0.7-0`; (3) nie istnieje dowód stanu „jak przed wyjęciem karty", a `autorun.sh` nie ma żadnej ścieżki wycofującej; (4) nie zbadano zachowania na fizycznym sprzęcie. Rebuild v0.0.7.0 musi zostać wykonany od zera z czystego `b912eed`, bez `emulator-audio`, z nowym pomiarem wszystkich hashy i z jawnym rozwiązaniem problemu wyjęcia karty (kanon 3) — inaczej paczka nie może być przekazana do użycia.

---

*ERNIE — cross-check, PROVEN vs INFERRED. Nie zmieniono `src/`, `build/`, `Cargo.*`. Jedyny zapisany plik: `docs/VP-0.0.7.0-ERNIE-CROSSCHECK.md`. 🪖☕*
