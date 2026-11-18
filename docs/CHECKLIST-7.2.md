# CHECKLIST 7.2 — żywy dokument (zabezpieczenie przed kompresją kontekstu)

**Heniek (konsultant wydania 7.2).** Założona: 2026-11-18 ~22:15Z.
Aktualizuję na bieżąco. Nie jest to raport jednorazowy — czytaj ją przed każdą operacją 7.2.

---

## 0. Zakres 7.2 — ZAMROŻONY (fafi zatwierdził)

Dokładnie trzy zmiany w napisach, **nic poza tym**:

1. `ACORDATOR` → `ACCORDATOR`
2. `MODE: ACORD` → `MODE: ACCORD`
3. MUTATOR: `SUBTLE FRZ OFF FRZ: OFF` → `SUBTLE FRZ:OFF`

Wszystko inne (silnik audio, `/run`, guard, pads-off, 20 kontroli) **bez zmian**.

## 1. Właściciele plików (kto co rusza)

| Agent | Pliki | Status |
|---|---|---|
| FLINT-3 | `src/splash.rs` + testy (3 napisy) | w toku — raport `docs/VP-...` / `docs/*FLINT*7.2*` jeszcze nie pojawił się |
| CZESIEK-3 | `docs/WERYFIKACJA-WIZUALNA-0.0.7.2.md` (plan screenów) | w toku |
| **Heniek** | `README.md`, `docs/manual/index.html`, `docs/manual/USERS-MANUAL-0.0.7.2.md`, gałąź `gh-pages`, ten plik, `docs/KOORDYNACJA-STRONA-7.1.md` | **zrobione** (część strony), bez pusha main |
| fafi | `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, build, weryfikacja, tag, release, push main, karta SD | do zrobienia |

**Zakaz krzyżowania:** Heniek NIE rusza `src/`, `Cargo.*`, `build/`, skryptów, maszyn, karty SD, MPC. Heniek NIE pushuje `main`.

---

## 2. Stan zastany (PROVEN)

| Element | Wartość | Uwaga |
|---|---|---|
| Repo | `/Users/fafi/Downloads/ArpeggiGator` | branch `release/0.0.7.2` |
| HEAD lokalny | `2d81710` (`docs: refresh 20-check verification report for 7.2`) | wcześniej `7f06231` release, `44e4455` docs Heńka |
| `origin/main` | `542b33fa88f83a74fd4a79478c3f118b106657f4` | bez zmian — push 7.2 jeszcze nie wykonany |
| Tag | `v0.0.7.1` = `7d652b0` | **nie zmieniać** przed nowym release'em |
| `origin/gh-pages` | **`66efcd16625e3b3bddb5704996ca22285b4e2958`** | commit `site: 0.0.7.2 label update — ACCORDATOR, MODE: ACCORD` (push Heńka, sukces) |
| Hasz ZIP 7.1 (wciąż na stronie) | `66846aaaf73b06b218ebc63c9ecea8606e97c14e7f9aecd677bc1aea8f4678e9` | `arpeggigator-v0.0.7.1.zip` |
| Hasz lab `.so` 7.1 (wciąż na stronie) | `66e4c6cbd5125c15af83da06b29d7f6cc94e157f1cdfaa0fe847f13899ab6df8` | `arpeggigator-v0.0.7.1-lab.so` |
| **Nowy ZIP 7.2** | **`a882223bfea2d9e96e4e1d8d2067974c35952ad688526fe24122354fe1936fba`** | `build/arpeggigator-v0.0.7.2.zip` (Jurek-3, `docs/RAPORT-JUREK-3-BUILD-072.md:25`) |
| **Nowy lab `.so` 7.2** | **`3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8`** | `build/arpeggigator-v0.0.7.2-lab.so` (jw., linia 26; `cmp=0` z `.so` w paczce) |
| Banner | `assets/readme-user-banner.webp` | przywrócony przez fafi w `542b33f`, zachowany |

### Zmiany Heńka — **ZATWIERDZONE** (commit `44e4455`) i wypchnięte na gh-pages

- `README.md` — `ACORDATOR`→`ACCORDATOR`, dopisek „0.0.7.2 (in progress)", link/hashe 7.1 bez zmian, banner i BETA/backup nienaruszone.
- `docs/manual/index.html` — jw. + link do nowego pliku MD.
- `docs/manual/USERS-MANUAL-0.0.7.2.md` (rename z 0.0.7.1) + treść 7.2.
- **Do zrobienia po release'ie 7.2:** podmienić w 4 miejscach link `v0.0.7.1` → `v0.0.7.2` i hashe na powyższe nowe, usunąć adnotację „in progress" (main = fafi, gh-pages = Heniek). Heniek nie pushuje `main`.

---

## 3. Rekomendowana kolejność operacji 7.2 (bramki)

```
0. ZAMROŻENIE ZAKRESU
   → tylko 3 napisy. Każda dodatkowa zmiana = osobna decyzja fafi.

1. MERGE FLUXÓW AGENTÓW  [bramka: konflikty rozwiązane przez właściciela pliku]
   → FLINT-3: src/splash.rs + testy
   → CZESIEK-3: docs/WERYFIKACJA-WIZUALNA-0.0.7.2.md
   → Heniek: publiczne teksty (już gotowe, osobny commit fafi)
   → UWAGA: nigdy więcej `git merge -s ours` — to właśnie zabiło ładny README.
     Po merge: git diff --stat i ogląd README/banneru.

2. BUMP WERSJI (fafi)
   → Cargo.toml / Cargo.lock: 0.0.7-2
   → CHANGELOG.md: wpis 0.0.7.2 (3 napisy)
   → sprawdzenie zgodności z README/manual (tabela w §2)

3. BUILD (fafi)
   → canonical build; dwa artefakty: SD ZIP + lab .so
   → nowe SHA-256 → ZAPISAĆ TUTAJ w §2 przed jakąkolwiek podmianą na stronie
   → MANIFEST-0.0.7.2.md

4. WERYFIKACJA 20/20 (fafi)
   → python3 scripts/verify-v070.py
   → cargo test (z testami FLINT-3)
   → testy MUSZĄ dotykać widocznych napisów, nie tylko stałych — patrz §5

5. A/B WIZUALNE NA QA (CZESIEK-3 + fafi)  [bramka: twarda]
   → screeny 1200x950, plain + SHIFT, wszystkie 5 trybów
   → ACCORDATOR mieści się w polu 88 px, bez przycięcia
   → MODE: ACCORD, MUTATOR: SUBTLE FRZ:OFF
   → dopiero po PASS: tag

6. TAG + PUSH MAIN (fafi)
   → tag v0.0.7.2 na commicie release
   → git push origin release/0.0.7.2:main && git push origin v0.0.7.2
   → main i gh-pages to OSOBNE pushy — nie myl ich

7. RELEASE + STRONA (fafi + Heniek)
   → upload 2 artefaktów do Releases pod v0.0.7.2
   → release body EN z nowymi hashami
   → HENIEK podmienia linki i hashe w README + index.html + MD + RELEASE-BODY
     (dopiero gdy release istnieje!) → push gh-pages OSOBNO
   → fafi pushuje zmiany README na main
   → grep -R 'v0.0.7.1' w publicznych plikach → tylko historyczne wzmianki

8. KARTA SD — wyłącznie na wyraźne polecenie fafi
   → backup najpierw; zero dotykania fizycznej MPC bez zgody
```

---

## 4. PIĘĆ NAJWIĘKSZYCH RYZYK 7.2 + KONTRŚRODKI

| # | Ryzyko | Kontrśrodek |
|---:|---|---|
| 1 | **Przepełnienie pola 88 px** — `ACCORDATOR` jest dłuższe niż stare `ACORDATOR`; napis może się przyciąć / nachodzić na sąsiedni panel. | Nie wystarczy test stringu. Wymagany dowód wizualny ze screena 1200×950 przy trzymanym SHIFT (CZESIEK-3). Jeśli przycięte → STOP, decyzja fafi (skrót nazwy / mniejsza czcionka), nie „jakoś to będzie". |
| 2 | **Zgubienie / podmiana hashy na stronie** — aktualizacja 7.2 może zostawić stare hashe 7.1 przy nowym linku. | Nie ruszać hashy i nazw plików przed release'em (tak jest teraz). Po buildzie wpisać nowe hashe do §2 tej checklisty, potem jednorazowo podmienić w 4 miejscach i zweryfikować `shasum -a 256 -c`. |
| 3 | **Stary tag `v0.0.7.1` w linkach** — przycisk „Download" i sekcja Download zostaną na starym release'ie. | Bramka #7: `grep -R 'releases/tag\|releases/latest'` poREADME, manualu i gh-pages; każdy link sprawdzony `curl -I`. Nie publikować „latest" zanim release 7.2 istnieje. |
| 4 | **Rozjazd wersji Cargo vs README vs manual vs release body** — np. Cargo `0.0.7-2`, strona „7.1", release opisany „7.2". | Jedna tabela stanu w §2 tej checklisty; przed tagiem `grep -R '0.0.7-2\|0.0.7.2'` i ręczne porównanie 4 plików. Właściciel Cargo = fafi, właściciel docs = Heniek — cross-check przed tagiem, nie po. |
| 5 | **Push/merge zagrażający treści publicznej** — Heniek nie ma prawa pusha main, a poprzedni `merge -s ours` już nadpisał README; dodatkowo ryzyko, że gh-pages i main rozjadą się. | Osobne pushy: main = fafi, gh-pages = Heniek. Po każdym pushu `git ls-remote` i `git diff --stat` przed merge. Heniek nie używa `push origin HEAD:main`. |

---

## 5. Przegląd raportu FLINT-3 — WYKONANY (2026-11-18 ~23:15Z)

**Nie zmieniłem raportu FLINT-3 ani `src/`.** Źródła: `docs/RAPORT-FLINT-3-NAPISY-072.md` (182 linie) oraz `git show 7f06231 -- src/splash.rs` — `git diff HEAD` jest pusty, bo kod jest już commitowany w `7f06231 release: 7.2 spelling fixes and verified build` (HEAD aktualny to `2d81710`).

| Kryterium z dawnej listy | Wynik |
|---|---|
| Raport istnieje | ✅ `docs/RAPORT-FLINT-3-NAPISY-072.md`, przeczytany cały |
| Testy asertują **widoczny napis** | ⚠️ **częściowo** — 2 z 3 mocne, 1 słabsza (uwaga 1) |
| Test negatywny: stary napis nie występuje | ❌ **brak** |
| Pokrycie 3 zmian | ✅ teksty tak; **font 11.0 nieobjęty żadnym testem** |
| Dowód wizualny 1200×950 / 88 px | ❌ **brak** — Flint nie miał maszyny; wymagany od Krystyny-3 wg planu Czesieka-3 |
| Test nie opiera się na heartbeat / OCR | ✅ to czyste testy jednostkowe stringów |

### 3 konkretne uwagi z reviewu

**Uwaga 1 — testy: 2 mocne, 1 słaba, brak testu negatywnego.**
`mutator_freeze_off_is_not_duplicated` i `mutator_freeze_on_keeps_the_single_suffix` asertują dokładne stringi (`"SUBTLE FRZ:OFF"`, `"WILD FRZ:ON"`) zwracane przez `mutator_param_line`, który jest **faktycznie użyty w renderze** (`src/splash.rs:209`) — to są testy widocznego napisu, w porządku. Natomiast `accord_captions_have_the_correct_spelling` sprawdza tylko wartości stałych `MODE_ACCORD`/`ACCORDATOR_TITLE`. Stałe są użyte przy renderze (linie 283 i 1384), więc to nie są martwe stałe, ale test **nie wykryje** podmiany wywołania na inny literał (np. powrotu do `"MODE: ACORD"`). Brakuje też testu negatywnego. Rekomendacja (nieblokująca buildu): helper zwracający napis dla trybu, używany zarówno przez render, jak i test, albo przynajmniej statyczny check `! grep -R -E 'MODE: ACORD\b|ACORDATOR' src/splash.rs`.

**Uwaga 2 — font 11.0 NIE jest udowodniony jako bezpieczny dla pola 88 px.**
Tabela w raporcie Flinta podaje „znaków na px" (88/10 = 8.8 vs 88/9 = 9.8), ale przy prostym modelu liniowym wniosek wychodzi odwrotny: stare `ACORDATOR` = 9 znaków × 12.0 ≈ 108 jednostek, nowe `ACCORDATOR` = 10 znaków × 11.0 ≈ **110** jednostek, czyli ~1,9% **szerszy** napis, nie węższy. Realne szerokości glifów (dodany `C` jest stosunkowo wąski) mogą to zrekompensować, ale to **INFERRED**, nie PROVEN. Korekta 12.0→11.0 jest rozsądnym odruchem, lecz nie daje udowodnionego marginesu bezpieczeństwa. Nie traktuję jej jako dowodu.

**Uwaga 3 — screen 1200×950 jest KONIECZNY, nie opcjonalny.**
Żaden test nie dotyka rozmiaru czcionki ani `TextBox { width: 88 }` w `vendor/mpc-sample-sdk/src/draw.rs:98-147`, a firmware przycina tekst. W środowisku nie ma OCR/PIL (Czesiek-3: `tesseract`, `magick`, `PIL` → BRAK), więc jedynym dowodem jest obraz + hash. Wymagane: `ACCORDATOR` w B1 przy trzymanym SHIFT, `MODE: ACCORD` w panelu, `SUBTLE FRZ:OFF` w MUTATORze — zrzut 1200×950 **przed release'em** (harness robi `06_screen1200_PRESSED.png` przed `release`), porównanie regionów `B1_TITLE/MODE_TITLE/PARAM_LINE` przez `docs/tools/v072_caption_hash.sh`, hash pliku i `date -u`. Bez tego 7.2 nie jest zamknięte.

**Werdykt: AKCEPTUJĘ kod FLINT-3 do buildu** (i tak jest już w `7f06231`, Jurek-3 ma 20/20 i 87/87), **ale warunkowo**: przed tagiem/release'em musi być screen QA z bramki §3.5 potwierdzający brak ucięcia w polu 88 px. Nie akceptuję oznaczenia „7.2 gotowe" bez tego dowodu. Raportu Flinta ani `src/` nie ruszałem.

---

## 6. Log statusów (living)

| Czas (Z) | Zdarzenie | Status |
|---|---|---|
| 22:10 | Odebrano zlecenie aktualizacji strony 7.2 | ✅ |
| 22:12 | Zamiana `ACORDATOR`→`ACCORDATOR`, `ACORD`→`ACCORD` w README, manualu i gh-pages | ✅ |
| 22:14 | Wersja „0.0.7.2 (in progress)", link/hashe 7.1 bez zmian, rename MD do 0.0.7.2 | ✅ |
| 22:15 | `grep \bACORD\b\|\bACORDATOR\b` → 0 trafień; polskie znaki → 0; banner obecny | ✅ |
| 22:16 | Push gh-pages `e3c0168` → `66efcd1` | ✅ |
| 22:17 | Przyjęcie roli konsultanta; założenie tej checklisty | ✅ |
| 23:06 | Merge fluxów FLINT-3 / CZESIEK-3 + bump + build w `7f06231` (wg `HANDOFF-7.2-LIVE.md`) | ✅ |
| 23:06 | verify 20/20 + `cargo test` 87/87 (Jurek-3, log `docs/logs/verify-072.log`) | ✅ |
| 23:15 | **Review raportu FLINT-3 (§5)** — akceptacja warunkowa: brak screena 88 px | ✅ (warunek ⏳) |
| — | A/B wizualne QA (88 px!) — Krystyna-3 wg planu Czesieka-3 | ⏳ **BLOKER tagu** |
| — | Tag + push main (fafi), release, aktualizacja linków/hashy na stronie | ⏳ |

---

**Podpisano:** Heniek 🪖☕ — Strażnik Pamięci i Ciągłości, konsultant wydania 7.2.
