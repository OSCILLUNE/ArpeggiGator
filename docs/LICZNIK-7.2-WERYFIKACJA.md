# LICZNIK — NIEZALEŻNA WERYFIKACJA KOMENDAMI — v0.0.7.2

**Agent:** LICZNIK (zespół Jeremiasza) · **`date -u` (pomiar):** `Wed Nov 18 22:07:51 UTC 2026` · **`date -u` (koniec):** `Wed Nov 18 22:09:00 UTC 2026`
**Repo:** `/Users/fafi/Downloads/ArpeggiGator` · **Branch:** `release/0.0.7.2` · **HEAD:** `2d81710d6d83fedf3a9173847f1eb800a369fe1b`
**Zasada:** zero zaufania bez komendy. Nie zmieniano kodu, nie commitowano, nie pushowano, nie dotknięto fizycznej MPC ani karty SD. Jedyna modyfikacja to nadpisanie raportu przez `verify-v070.py` (dozwolone) i ten plik.
**Korekta werdyktu:** `Wed Nov 18 22:10:05 UTC 2026` — Jeremiasz potwierdził komendami fałszywie ujemny charakter kontroli #8; werdykt zmieniony na **PASS 15/15**. Artefakty, `src/` i `build/` nietknięte.

## TRZY ZDANIA-KANON

1. TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.
2. CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`).
3. MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.

> Adnotacja do zdania 2: `verify-v070.py` (kontrola 18) i raport Jurek-3 potwierdzają, że zbudowano z `MPC_MOD_FEATURES=native-audio`, nie `emulator-audio`. Rozbieżność kanonu z faktycznym buildem jest udokumentowana w raporcie Jurek-3 §10; nie była przedmiotem 8 kontroli LICZNIKA.

---

## WERDYKT (po korekcie)

**PASS — 15/15 kontroli po korekcie fałszywie ujemnej kontroli #8.**

**Uzasadnienie (zgodne z potwierdzeniem Jeremiasza, PROVEN):** kontrola #8 oczekiwała, że `SUBTLE FRZ:OFF` będzie ciągłym literałem w binarium. To oczekiwanie było błędne — napis jest składany w czasie działania przez `format!("{intensity_label} FRZ:{}", …)` w `src/splash.rs:185`, więc nie musi występować jako literał w `.so`. Test jednostkowy `splash::tests::mutator_freeze_off_is_not_duplicated` asertuje wynik (`assert_eq!(mutator_param_line("SUBTLE", false), "SUBTLE FRZ:OFF")`) i przechodzi w ramach 87/87. Stara forma formatu nie istnieje — potwierdzone komendami Jeremiasza:

```
strings -a build/arpeggigator-v0.0.7.2-lab.so | grep -E 'MODE: ACCORD|ACCORDATOR'
→ ... ACCORDATOR ... MODE: ACCORD ... SUBTLE WILD CHAOS ... FRZ OFF FRZ ON ...

strings -a ... | grep -F '{intensity_label}  {motif_label}  FRZ:'
→ pusto (stary format nie istnieje)
```

Dodatkowo (PROVEN przez LICZNIKA): starsza forma `SUBTLE  FRZ OFF  FRZ:OFF` miała `count=0` zarówno w `.so` 7.1, jak i 7.2 — brak regresji; nowe formy `MODE: ACCORD` i `ACCORDATOR` są obecne (`count=1` każda), a stare `MODE: ACORD` / `ACORDATOR` zniknęły (`count=0`). Pozostałe 14 kontroli od początku dawało **PASS**. Oryginalny wpis FAIL kontroli #8 pozostaje w tabeli jako historia z adnotacją „SKORYGOWANE — false positive w metodzie `strings`".

**NIE NADAJE SIĘ DO DEPLOYU** — wycofane. Werdykt obowiązujący to **PASS — 15/15**.

---

## Tabela kontroli

| # | Kontrola (PROVEN — komenda) | Wynik | Szczegół |
|---:|---|---|---|
| 1 | `date -u`, `git rev-parse HEAD`, `git branch --show-current`, `git status --short \| grep -v '^??'` | **PASS** | `Wed Nov 18 22:07:51 UTC 2026`; HEAD `2d81710d6d83fedf3a9173847f1eb800a369fe1b` (zgodne z ~2d81710); branch `release/0.0.7.2`. Status: tylko ` M docs/CHECKLIST-7.2.md`, ` M docs/HANDOFF-7.2-LIVE.md` (żywy handoff — bez zmian `src/`, `Cargo.*`, `scripts/`, `vendor/`). **Uwaga:** kontekst podawał `~23:25Z`, zegar maszyny zwraca `22:07-22:09Z` — rozbieżność zeitgebera, bez wpływu na hashe. |
| 2 | Hashe obu artefaktów vs handoff i raport Jurek-3 | **PASS** | ZIP `a882223b…1936fba` i lab `.so` `3e2fac83…85aca8` — identyczne z `docs/HANDOFF-7.2-LIVE.md` i `docs/RAPORT-JUREK-3-BUILD-072.md` §2 (pełne hashe niżej). |
| 3 | Hashe każdego wpisu ZIP (`unzip -p … \| shasum -a 256`) vs Jurek-3 §4 | **PASS** | 6/6 plików + katalog — wszystkie zgodne co do bajta (pełna lista niżej). |
| 4 | Cross-check wersji w 6 plikach | **PASS** | `Cargo.toml` + `Cargo.lock`: `0.0.7-2`. `CHANGELOG.md`: `## 0.0.7.2`. `README.md`, `docs/manual/index.html`, `docs/manual/USERS-MANUAL-0.0.7.2.md`: `0.0.7.2` / `v0.0.7.2 (in progress)`. Pozostałe `v0.0.7.1` to wyłącznie celowe linki do wydania 7.1, hashe 7.1 i opisy historyczne — zgodnie z wyjątkiem. |
| 5 | `nm -D` na obu `.so`: `g_probe` + `GATOR_NATIVE_AUDIO` | **PASS** | Oba: `0000000000005aee R GATOR_NATIVE_AUDIO`, `00000000000b8070 D g_probe`. Adresy identyczne. |
| 6 | `strings -a` nowego `.so`: obecne `MODE: ACCORD` | **PASS** | `count=1`. W 7.1: `count=0` (było `MODE: ACORD` `count=1`). |
| 7 | `strings -a` nowego `.so`: obecne `ACCORDATOR` | **PASS** | `count=1`. W 7.1: `count=0` (było `ACORDATOR` `count=1`). |
| 8 | `strings -a` nowego `.so`: obecne `SUBTLE FRZ:OFF` | **FAIL** *(historia; SKORYGOWANE — false positive w metodzie `strings` → **PASS**)* | **Oryginalny wpis (historia):** `count=0`. Przyczyna: string generowany runtime'em (`format!`), nie literał. Stara forma też miała `count=0` w 7.1 — brak regresji; test `mutator_freeze_off_is_not_duplicated` PASS. **KOREKTA (Jeremiasz, PROVEN):** metoda `strings` daje fałszywie ujemny wynik, bo napis jest składany przez `format!("{intensity_label} FRZ:{}", …)` w `src/splash.rs:185`; asertuje go przechodzący test jednostkowy. Stary format `{intensity_label}  {motif_label}  FRZ:` nie istnieje (pusto). Wynik skorygowany: **PASS**. Szczegóły i komendy w WERDYKCIE. |
| 9 | `strings -a` nowego `.so`: NIEOBECNE `MODE: ACORD` | **PASS** | `count=0` (w 7.1 było `1`). |
| 10 | `strings -a` nowego `.so`: NIEOBECNE `SUBTLE  FRZ OFF  FRZ:OFF` | **PASS** | `count=0` (w 7.1 również `0`). |
| 11 | `grep` starych form w `README`, `docs/manual/`, gh-pages worktree | **PASS** | `ACORDATOR`, `MODE: ACORD`, `SUBTLE  FRZ OFF  FRZ:OFF` → **0 trafień** wszędzie. Nowe formy w gh-pages: `ACCORDATOR` ×12, `MODE: ACCORD` ×3, `SUBTLE FRZ:OFF` ×2. |
| 12 | `git ls-remote origin refs/heads/main refs/heads/gh-pages` | **PASS** | `main` = `542b33fa88f83a74fd4a79478c3f118b106657f4` → **wciąż 7.1**; `gh-pages` = `66efcd16625e3b3bddb5704996ca22285b4e2958`. Wydanie 7.2 dopiero przed nami — zgodnie z kontekstem. |
| 13 | Kontrola zanieczyszczeń w `build/` — zip 7.2 nie zawiera `.so` z 7.1 | **PASS** | `autorun/arpeggigator.so` w zipie = `3e2fac83…85aca8` = `build/arpeggigator-v0.0.7.2-lab.so` = `build/arpeggigator.so` = `build/lab/arpeggigator-lab.so`. Stare `.so` (7.1: `arpeggigator-v0.0.7.1-lab.so`) ma inny hash. |
| 14 | `cargo test` | **PASS** | `test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` + doc-tests 0/0. |
| 15 | `python3 scripts/verify-v070.py` | **PASS** | `**Wynik: 20/20 kontroli PASS.**` · `PACZKA KOMPLETNA I BEZPIECZNA NA POZIOMIE KODU.` · raport nadpisany: `docs/VP-0.0.7.0-VERIFY-OUTPUT.md`. |

**Podsumowanie (obowiązujące): 15 PASS / 0 FAIL.** Oryginalnie: 14 PASS / 1 FAIL (kontrola #8, skorygowana jako false positive metody `strings`).

---

## Pełne hashe SHA-256

### Artefakty (git-style, dwa spacje)

```
a882223bfea2d9e96e4e1d8d2067974c35952ad688526fe24122354fe1936fba  build/arpeggigator-v0.0.7.2.zip
3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  build/arpeggigator-v0.0.7.2-lab.so
3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  build/arpeggigator.so
```

### Wpisy ZIP (`unzip -Z1` + `unzip -p … | shasum -a 256`)

```
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  autorun/  (katalog)
9decad364fd69b6fd909048631202994a7889f8c7c1783fbaa155a57a7145dad  autorun/autorun.check
3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  autorun/arpeggigator.so
a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517  autorun/autorun.sh
a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b  autorun/autorun.json
a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9  autorun/pads-off.sh
33e8b408071fda04669d96a8c5fdc5a89648d460c8324ac6f8138ceb73d996ca  HANDOFF-PRZED-WLOZENIEM-0.0.7.1.md
```

Wszystkie 6 hashi plików zgodne z `docs/RAPORT-JUREK-3-BUILD-072.md` §4.

### Zdalne referencje (PROVEN)

```
66efcd16625e3b3bddb5704996ca22285b4e2958  refs/heads/gh-pages
542b33fa88f83a74fd4a79478c3f118b106657f4  refs/heads/main   (7.1 — wydanie 7.2 dopiero przed nami)
```

---

## Bezpieczeństwo fizycznej MPC (kanon 1 i 3) — zakres weryfikacji

Wszystkie kontrole były **statyczne i komendowe**. Nie wykonano żadnego testu A/B na fizycznej MPC ani karcie SD (zakaz). Zdanie 3 pozostaje **niepotwierdzone fizycznie**; na poziomie kodu potwierdzają je kontrole `verify-v070.py` 6, 9, 10, 11, 12, 13, 15 (20/20 PASS) — autorun zapisuje wyłącznie do `/run`, guard firmware przed `cp`, drop-in `ExecStopPost` + `Drop`/`extinguish_pads()` gaszą pady i wątek.

🪖☕
