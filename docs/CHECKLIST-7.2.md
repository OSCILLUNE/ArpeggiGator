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
| HEAD lokalny i `origin/main` | `542b33f` (= `542b33fa88f83a74fd4a79478c3f118b106657f4`) | published 7.1 + strona Heńka + banner |
| Tag | `v0.0.7.1` = `7d652b0` | **nie zmieniać** przed nowym release'em |
| `origin/gh-pages` (przed moim pushem) | `e3c0168` | |
| `origin/gh-pages` (po moim pushem) | **`66efcd16625e3b3bddb5704996ca22285b4e2958`** | commit `site: 0.0.7.2 label update — ACCORDATOR, MODE: ACCORD` |
| Hasz ZIP 7.1 (bez zmian na stronie) | `66846aaaf73b06b218ebc63c9ecea8606e97c14e7f9aecd677bc1aea8f4678e9` | `arpeggigator-v0.0.7.1.zip` |
| Hasz lab `.so` 7.1 (bez zmian) | `66e4c6cbd5125c15af83da06b29d7f6cc94e157f1cdfaa0fe847f13899ab6df8` | `arpeggigator-v0.0.7.1-lab.so` |
| Banner | `assets/readme-user-banner.webp` | przywrócony przez fafi w `542b33f`, zachowany |

### Zmiany Heńka przygotowane, **NIEZATWIERDZONE** w głównym worktree

- `README.md` — `ACORDATOR`→`ACCORDATOR`, dopisek „0.0.7.2 (in progress)", release link i hashe 7.1 bez zmian, banner i BETA/backup nienaruszone.
- `docs/manual/index.html` — jw. + link do nowego pliku MD.
- `docs/manual/USERS-MANUAL-0.0.7.1.md` → **`USERS-MANUAL-0.0.7.2.md`** (rename) + treść 7.2.
- `docs/PODGLAD-STRONY-HENIA.png` — screenshot live strony (z poprzedniego etapu).

**Do zrobienia przez fafi:** `git add README.md docs/manual/` i commit/push na `main`. Heniek tego nie robi.

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

## 5. Przegląd raportu FLINT-3 — kryteria (do wypełnienia, gdy plik pojawi się)

**Nie zmieniam pliku FLINT-3.** Dopisuję tu uwagi po lekturze.

- [ ] Raport istnieje (szukaj `docs/*FLINT*7.2*` / `docs/VP-*7.2*`). **Stan: JESZCZE BRAK** (22:15Z).
- [ ] Testy asertują **dokładny napis widoczny na ekranie** (literał przekazywany do renderera tekstu w `src/splash.rs`), a nie tylko nazwę stałej / wariantu enuma.
- [ ] Test negatywny: stary napis nie występuje.
- [ ] Pokrycie wszystkich 3 zmian: `ACCORDATOR`, `MODE: ACCORD`, `SUBTLE FRZ:OFF`.
- [ ] Dowód wizualny: screen 1200×950 z SHIFT, pole 88 px, brak przycięcia i nadpisania.
- [ ] Test nie opiera się wyłącznie na heartbeat / OCR (kanon z `PROMPT-GEMINI-3.8-FLASH.md`).

**Werdykt (uzupełnić po lekturze):** _(jeszcze nie oceniono)_

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
| — | Merge fluxów FLINT-3 / CZESIEK-3 | ⏳ |
| — | Bump Cargo/CHANGELOG, build, nowe hashe | ⏳ |
| — | verify 20/20 + testy napisów | ⏳ |
| — | A/B wizualne QA (88 px!) | ⏳ |
| — | Tag + push main (fafi), release, aktualizacja linków/hashy na stronie | ⏳ |

---

**Podpisano:** Heniek 🪖☕ — Strażnik Pamięci i Ciągłości, konsultant wydania 7.2.
