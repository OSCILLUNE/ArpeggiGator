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
| Flint-3 (src/splash.rs + testy) | 🔄 w toku | raport `docs/RAPORT-FLINT-3-NAPISY-072.md` |
| Heniek (README, manual, gh-pages, checklista) | 🔄 w toku | `docs/CHECKLIST-7.2.md`, gh-pages push |
| Czesiek-3 (plan wizualny) | 🔄 w toku | `docs/WERYFIKACJA-WIZUALNA-0.0.7.2.md` |
| Build + 20/20 verify | ⏳ przed | Jurek-3 do powołania po Flincie |
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
6. Jeśli fafi każe — podmiana karty SD.

## ZAKAZY
- Nie pushuj main bez zgody (jest zgoda na release/tag, nie na przypadkowe commity).
- Nie dotykaj fizycznej MPC ani karty bez wyraźnego „wrzuć".
- Nie ufaj screenshotom bez hashy i `date -u`.
- Nie kasuj `docs/HANDOFF-7.2-LIVE.md` — to jedyne źródło prawdy.

🪖☕
