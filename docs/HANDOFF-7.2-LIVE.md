# HANDOFF 7.2 — JEDEN ŻYWY PLIK (aktualizuj na bieżąco)

**Jeremiasz.** Start: 2026-11-18 ~22:20Z. Repo: /Users/fafi/Downloads/ArpeggiGator, branch `release/0.0.7.2`, baza `542b33fa88f83a74fd4a79478c3f118b106657f4`.

> Cel: oszczędzać kontekst. Wszystkie decyzje, hashe i statusy lądują TUTAJ. Nowa AI czyta ten plik jako pierwszy.

## TRZY ZDANIA-KANON
1. TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.
2. CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`).
3. MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.

## ZAKRES 7.2 (zatwierdzony przez fafi)
Tylko 3 napisy + testy + aktualizacja strony. Zero zmian w audio, logice i bezpieczeństwie.

| # | Gdzie | Jest | Będzie |
|---|---|---|---|
| 1 | `src/splash.rs:271` | `MODE: ACORD` | `MODE: ACCORD` |
| 2 | `src/splash.rs:1370` | `ACORDATOR` | `ACCORDATOR` |
| 3 | `src/splash.rs:194-197` | `SUBTLE  FRZ OFF  FRZ:OFF` | `SUBTLE FRZ:OFF` |

## STAN (aktualizuj)
| Element | Status | Hash / uwaga |
|---|---|---|
| Branch | ✅ | `release/0.0.7.2` @ `542b33f` |
| Wersja | ✅ | `0.0.7-2` w Cargo.toml + Cargo.lock, wpis w CHANGELOG |
| Flint-3 (src/splash.rs + testy) | ✅ review HEŃKA (warunkowo) | `7f06231`, 87/87; **brak screena 88 px = bloker tagu** — patrz `REVIEW HEŃKA` |
| Heniek (README, manual, gh-pages, checklista) | 🔄 w toku | `docs/CHECKLIST-7.2.md`, gh-pages push |
| Czesiek-3 (plan wizualny) | 🔄 w toku | `docs/WERYFIKACJA-WIZUALNA-0.0.7.2.md` |
| Build + 20/20 verify | ✅ 20/20 PASS | Jurek-3: ZIP `a882223b…`, lab `.so` `3e2fac83…`, `cmp=0`; log `docs/logs/build-072.log`; raport `docs/RAPORT-JUREK-3-BUILD-072.md` |
| A/B wizualne na QA (10×) | ⏳ przed | Krystyna-3 po buildzie |
| Tag + Release (EN) | ⏳ przed | ja przez UI |
| Karta SD | ⏳ tylko na życzenie fafi | — |

## ZDALNE STANY (PROVEN, 2026-11-18)
- `main` = `542b33f` (7.1 + strona Heńka + banner `readme-user-banner.webp`).
- `gh-pages` = `e3c0168` → po 7.2 będzie nowy hash (Heniek).
- Release `v0.0.7.1` = `7d652b0`, Assets: ZIP `66846aaa…`, lab `.so` `66e4c6cb…`.

## KOLEJNOŚĆ (Heniek do potwierdzenia w §11)
1. Flint-3 kończy kod → ja sprawdzam diff i `cargo test`.
2. Heniek kończy stronę + checklistę → ja commituję jego pliki na 7.2.
3. Jurek-3: build obu artefaktów + `verify-v070.py` 20/20 + hashe.
4. Krystyna-3: screeny QA MUTATOR/ACCORDATOR wg planu Czesiek-3, `hold` przed release.
5. Tag `v0.0.7.2`, release po angielsku, nowe hashe w opisie.
6. **WYMÓG FAFI:** na koniec wystawić 0.0.7.2 do testów na QA **z trwałym autorunem na karcie** (`lab.py autorun build/arpeggigator-v0.0.7.2.zip`) — ma przetrwać restarty. Nie używać `preload` jako rozwiązania końcowego. Dopiero potem ewentualna fizyczna karta SD.

## DODATKOWY AGENT
- **Licznik** — szybka, niezależna weryfikacja komendami (hashe, wersja w 4 plikach, spójność release/strony). Raport: `docs/LICZNIK-7.2-WERYFIKACJA.md`.

## REVIEW HEŃKA — KOD FLINT-3 (2026-11-18 ~23:15Z)

Przeczytałem `docs/RAPORT-FLINT-3-NAPISY-072.md` i `git show 7f06231 -- src/splash.rs` (`git diff HEAD` pusty — kod jest już commitowany). Nie zmieniłem raportu Flinta ani `src/`. Pełne uwagi w `docs/CHECKLIST-7.2.md` §5.

- **Testy:** 2 z 3 asertują widoczny napis (`mutator_param_line` → `"SUBTLE FRZ:OFF"` / `"WILD FRZ:ON"`, funkcja użyta w renderze przy `src/splash.rs:209`). Trzeci test sprawdza tylko stałe `MODE_ACCORD`/`ACCORDATOR_TITLE` — stałe są użyte przy renderze (linie 283, 1384), ale test nie wykryje podmiany call-site; brak testu negatywnego.
- **Font 11.0:** nieudowodniony jako bezpieczny. 10 znaków × 11.0 ≈ 110 jednostek vs stare 9 × 12.0 ≈ 108 — przy modelu liniowym ~1,9% *szerszy*, nie węższy. Realne glify mogą to zrekompensować, ale to INFERRED.
- **Screen 1200×950 KONIECZNY:** żaden test nie dotyka `TextBox { width: 88 }` ani rozmiaru czcionki; firmware przycina tekst, a w środowisku nie ma OCR/PIL. Wymagany zrzut przed release'em (ACCORDATOR w B1 ze SHIFT, `MODE: ACCORD`, `SUBTLE FRZ:OFF`) + hash + `date -u`, wg planu Czesieka-3 i `docs/tools/v072_caption_hash.sh`.

**Werdykt:** akceptuję kod FLINT-3 do buildu (jest już w `7f06231`, 87/87 + 20/20), **warunkowo** — bez screena QA potwierdzającego brak ucięcia w polu 88 px nie uznaję 7.2 za zamkniętą przed tagiem/release'em.

## ZAKAZY
- Nie pushuj main bez zgody (jest zgoda na release/tag, nie na przypadkowe commity).
- Nie dotykaj fizycznej MPC ani karty bez wyraźnego „wrzuć".
- Nie ufaj screenshotom bez hashy i `date -u`.
- Nie kasuj `docs/HANDOFF-7.2-LIVE.md` — to jedyne źródło prawdy.

🪖☕
