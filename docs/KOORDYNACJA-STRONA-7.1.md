# KOORDYNACJA — strona WWW a release v0.0.7.1 (Heniek → fafi, Jeremiasz)

**Heniek (Strażnik Pamięci i Ciągłości).** `date -u`: 2026-11-18 21:31Z.
Charakter dokumentu: **analiza i propozycja**. Nie zmieniłem żadnego istniejącego pliku, nie pushowałem, nie dotknąłem maszyn, karty SD ani fizycznej MPC.

## 0. Stan zastany (PROVEN — komendy, nie domysły)

| Fakt | Dowód |
|---|---|
| Jesteśmy na `release/0.0.6.4`; w trakcie mojej analizy Jeremiasz commitował 7.1: HEAD to teraz **`5d03eb3 release: v0.0.7.1 (native-audio, first-pad fix, one version two artifacts)`**, tracked **czyste**. Bloker commita zniknął — został push/tag. | `git status -sb`, `git log --oneline -4` (stan o 21:31Z) |
| `docs/manual/` jest **nieśledzony** (`?? docs/manual/`) — strona nie istnieje jeszcze w żadnym commicie. | `git status -sb` |
| Nie ma `docs/index.html`; jedyny `index.html` w repo to `docs/manual/index.html`. | `find . -name index.html` |
| Remote: `git@github.com:OSCILLUNE/ArpeggiGator.git`. Nie ma brancha `gh-pages`. `.github/` zawiera tylko `ci.yml` (Rust checks) — **brak workflow Pages**. | `git remote -v`, `git branch -a`, `find .github` |
| `gh` CLI **nie jest zainstalowany** (`gh not found`). | `which gh` |
| Artefakt MPC: `build/arpeggigator-v0.0.7.1.zip` (308 077 B), SHA-256 `66846aaaf73b06b218ebc63c9ecea8606e97c14e7f9aecd677bc1aea8f4678e9`, sidecar `.sha256` = OK, `unzip -t` czysty (7 wpisów). | `unzip -l`, `shasum`, `docs/RELEASE-0.0.7.1.md` |
| Artefakt emulatora/QA: `build/arpeggigator-v0.0.7.1-lab.so` (goły `.so` do `lab.py preload`, **nie ZIP**), SHA-256 `66e4c6cbd5125c15af83da06b29d7f6cc94e157f1cdfaa0fe847f13899ab6df8` — nazwa i hash **potwierdzone** w `docs/MANIFEST-0.0.7.1.md:1–14`. | `find build`, `docs/MANIFEST-0.0.7.1.md` |

---

## 1. Czym jest `docs/manual/index.html`?

**To instrukcja obsługi (Users Manual) w formie jednego statycznego HTML-a — nie jest to strona produktu/landing page.** 95 linii, `lang="pl"`. Struktura:

1. Drzewko interfejsu (SAMPLE → 16 LEVELS → nakładka),
2. Skróty B1/B2/B3/DATA/STOP/PLAY (tabela),
3. Parametry B2/B3 dla 5 trybów,
4. **Screeny** z v0.0.6.0 — 5 trybów × 2 widoki (plain / shift),
5. Procedury (podmiana sampli, freeze, STOP),
6. **„Dla testujących AI"** — wewnętrzna procedura QA.

**Zależności (wszystko lokalne, zero buildu):**
- CSS **inline** w `<style>` (linie 7–14) — bez zewnętrznego arkusza, bez frameworka, bez CDN, bez czcionek z zewnątrz.
- **Zero JavaScript**, zero fetch/API.
- 10 plików PNG w `docs/manual/screens/` (`01_ARP_plain.png` … `05_ORBITATOR_shift.png`, łącznie ~3,1 MB), wołane ścieżkami relatywnymi `screens/…` (linie 58–75).
- Jeden link do `USERS-MANUAL-0.0.6.0.md` (linia 19) — pełna wersja tekstowa w tym samym katalogu.

**Czy da się opublikować jako GitHub Pages z `/docs` bez buildu?** Technicznie **tak** — to czysty statyk, ścieżki są relatywne. Ale:
- Ponieważ plik leży w `docs/manual/`, adres będzie `https://oscillune.github.io/ArpeggiGator/manual/` (nie `/`), bo nie ma `docs/index.html`.
- **Włączenie Pages ze źródła `/docs` publikuje cały katalog `docs/`** — patrz §3. To jest realny bloker bezpieczeństwa, nie formalność.

---

## 2. Co dokładnie znaczy „wypchnąć stronę na main" — rekomendacja

Rozważane warianty:

| Wariant | Ocena |
|---|---|
| **A. Skopiować do `docs/index.html`** | Działa (landing pod `/`), ale powstają dwie kopie tej samej strony do synchronizacji. Niepotrzebny duplikat. |
| **B. Zostawić w `docs/manual/`** | Kanoniczna kopia już tam jest. Wymaga albo cienkiego `docs/index.html` (przekierowanie/link), albo linku z README. **Rekomendowane jako źródło treści.** |
| **C. Osobny katalog `site/` na main** | GitHub Pages **nie potrafi** serwować dowolnego folderu z brancha — tylko `/docs`, root, albo osobny branch. Sam `site/` nic nie da bez Actions. Odrzucić. |
| **D. Włączyć Pages z `/docs` na main** | Bez buildu, ale **publikuje 41 plików z wnętrza firmy** (§3). Odrzucić w obecnej formie. |

### Rekomendacja Henka: **B + czysty snapshot na branchu `gh-pages`**

1. Kanoniczna treść zostaje w `docs/manual/index.html` (+ `screens/`, + MD) na `main` — bez duplikatów.
2. Opcjonalnie dodać cienki `docs/index.html` z nagłówkiem i linkiem do `manual/` (3–5 linii, bez copy-paste treści).
3. **Źródłem Pages ustawić branch `gh-pages`** zawierający tylko 4 rzeczy: `index.html`, `screens/`, `USERS-MANUAL-0.0.6.0.md`, `.nojekyll`. W ten sposób „strona jest na main" (treść tam mieszka i jest wersjonowana), ale serwer publiczny nie widzi handoffów, logów i planów backupu.
4. Jeśli fafi **upiera się** przy `Source: /docs` na `main`: minimum to `_config.yml` z Jekyll `exclude:` wszystkiego poza `manual/`. To jednak **czarna lista** — każdy nowy plik w `docs/` domyślnie wycieknie. Odradzam; `gh-pages` jest jednorazowy i bezpieczny.

### Komendy (do zatwierdzenia przez fafi; **nie wykonałem żadnej z nich**)

```sh
# 0. zawsze najpierw: nic nie nadpisujemy
git -C /Users/fafi/Downloads/ArpeggiGator status -sb

# 1. commit treści na main (dopiero po zamknięciu 7.1 przez Jeremiasza!)
git add docs/manual/index.html docs/manual/screens docs/manual/USERS-MANUAL-0.0.6.0.md docs/index.html
git commit -m "docs: publish ArpeggiGator users manual site"

# 2. czysty snapshot gh-pages w osobnym worktree (git 2.54 obsługuje --orphan)
git worktree add --orphan /private/var/folders/k2/h_qnbjqn0nn4ttqxm2frgckw0000gn/T/ag-gh-pages gh-pages
cp docs/manual/index.html docs/manual/USERS-MANUAL-0.0.6.0.md /private/var/folders/k2/h_qnbjqn0nn4ttqxm2frgckw0000gn/T/ag-gh-pages/
cp -R docs/manual/screens /private/var/folders/k2/h_qnbjqn0nn4ttqxm2frgckw0000gn/T/ag-gh-pages/
touch /private/var/folders/k2/h_qnbjqn0nn4ttqxm2frgckw0000gn/T/ag-gh-pages/.nojekyll
git -C /private/var/folders/k2/h_qnbjqn0nn4ttqxm2frgckw0000gn/T/ag-gh-pages add -A
git -C /private/var/folders/k2/h_qnbjqn0nn4ttqxm2frgckw0000gn/T/ag-gh-pages commit -m "site: clean GitHub Pages snapshot"

# 3. push (wykonuje fafi po zgodzie)
git push origin main
git push origin gh-pages

# 4. UI: repo → Settings → Pages → Source: Deploy from a branch → gh-pages / (root) → Save

# 5. weryfikacja — kluczowy test, że wyciek jest zablokowany:
curl -I https://oscillune.github.io/ArpeggiGator/manual/     # 200
curl -I https://oscillune.github.io/ArpeggiGator/manual/screens/01_ARP_plain.png  # 200
curl -I https://oscillune.github.io/ArpeggiGator/HANDOFF-0.0.7.0-PLAN.md         # musi być 404!
```

---

## 3. Ryzyka — konkretne linie

### 3a. W samej stronie (`docs/manual/index.html`) — whitespace bezpieczny, ale treść nieaktualna

**Brak wycieku w HTML-u — POTWIERDZONE:** `grep -rniE '/Users/|token|password|secret|api[_-]?key|0\.0\.4\.9|https?://' docs/manual/` → **zero trafień**. Nie ma haseł, kluczy, tokenów ani lokalnych ścieżek w publikowanym HTML-u.

| Linia | Problem |
|---|---|
| 6 | `<title>ArpeggiGator — Users Manual v0.0.6.0</title>` — nieaktualna wersja (wypuszczamy 7.1). |
| 18 | `<span class="badge">v0.0.6.0 · commit b912eed</span> <span class="badge">build 34a9ec26…</span>` — jw. |
| 19 | Link do `USERS-MANUAL-0.0.6.0.md` — plik jest, ale opisuje stare UI. |
| 55 | `<h2>4. Tryby — screeny z v0.0.6.0</h2>` — do zachowania, ale z dopiskiem „UI bez zmian w 7.1". |
| 85–91 | Sekcja „6. Dla testujących AI": `mode=`, hash moda w `/run`, `NRestarts>0`, ABRT/SEGV, `1200x950`. To **wewnętrzne dane testowe** — nie dla użytkownika końcowego. Przenieść do `.md` albo oznaczyć „dla deweloperów". |
| 93 | `Źródło: tag v0.0.6.0 (b912eed) … Manual przygotowany przez Heńka.` — sygnatura OK, wersja do aktualizacji. |
| — | **Brak sekcji „Pobierz"** — strona nie linkuje do release'ów 7.1. Do dodania (§4). |

**Dodatkowy błąd w pliku `.md` (nie w HTML-u):** `docs/manual/USERS-MANUAL-0.0.6.0.md` linie **68, 69, 74, 75, 80, 81, 86, 87, 92, 93** używają `manual-screens-060/01_ARP_plain.png` itd., ale taki katalog **nie istnieje** — screeny są w `screens/`. Po opublikowania MD obok HTML-a wszystkie 10 obrazków w wersji tekstowej będzie broken. HTML ma poprawnie `screens/…` (linie 58–75).

**Obserwacja poza stroną (do Jeremiasza, ja nie ruszam):** `README.md:67` wciąż mówi „The 0.0.4.9 autorun package installs…" — stara numeracja w pliku, który Jeremiasz ma w robocie.

### 3b. Jeśli Pages zostanie włączone z całego `/docs` — **WYSOKIE ryzyko**

Publikacja obejmie **wszystkie** pliki w `docs/`, nie tylko `manual/`:

- **41 plików `.md` zawiera `/Users/fafi/...`**, np.
  - `docs/FEEDBACK-FAZA0-TESTER.md:131` — `ssh -i /Users/fafi/Downloads/virtual-mpc-sample/out/keys/id_ed25519 -p 2022 root@127.0.0.1`
  - `docs/HANDOFF-0.0.7.0-PLAN.md:71–72` — ścieżki do kluczy SSH DEV i QA (`virtual-mpc-sample*/out/keys/id_ed25519`).
  - `docs/HANDOFF-RESPONSE-REC-0.0.4.2.md:86,89,90` — `file:///Users/fafi/.gemini/antigravity/worktrees/...`.
  - `docs/BOARDING-PODSUMOWANIE.md:5`, `docs/VP-0.0.7.0-ERNIE-CROSSCHECK.md:6` — pełne lokalne ścieżki repo.
- **Topologia laboratorium i IP**: `docs/PLAN-BACKUP-MPC-FIZYCZNA.md:30,37,52,75–77` — root-SSH na `192.168.155.1:2222`, `scp -r root@192.168.155.1:/content`, `ssh -p 2222 -i KEY root@192.168.155.1`.
- **Wewnętrzne notatki NO-GO**, które kłócą się z publicznym przekazem „fafi zatwierdził 7.1": `docs/RELEASE-0.0.7.1.md:27–29` („NIE NADAJE SIĘ DO DEPLOYU bez weryfikacji przez Gemini 3.8 Flash") oraz `docs/RELEASE-0.0.7.0.md` §3 (cztery punkty bez dowodu na fizycznej MPC).
- Katalogi `docs/logs/`, `docs/qa_screens/`, `docs/backups/`, `docs/boarding-2026-11-17/`, `docs/tools/` też weszłyby do serializacji.

Są to **ścieżki do kluczy, nie same klucze** (samego materiału klucza/prywatnego hasła w `docs/` nie znalazłem), ale ujawnienie infrastruktury, nazwy użytkownika, adresu MPC i wewnętrznych zastrzeżeń jest niepożądane. **Dlatego: `gh-pages`, nie `/docs`.**

---

## 4. Jak strona ma się wiązać z release'ami 7.1

1. **Dodać sekcję „Pobierz"** z linkiem do stabilnego tagu, nie do plików na `main`:
   `https://github.com/OSCILLUNE/ArpeggiGator/releases/tag/v0.0.7.1`
   - **MPC / karta SD:** `arpeggigator-v0.0.7.1.zip` — z SHA-256 `66846aaa…4678e9` i informacją o sidecarze `.sha256` (`shasum -a 256 -c`).
   - **Emulator / QA-DEV:** `arpeggigator-v0.0.7.1-lab.so`, SHA-256 `66e4c6cb…ab6df8` (zgodny z `.so` w paczce SD). To **`.so`, nie ZIP** — w sekcji „Pobierz" trzeba to wyraźnie napisać (przeznaczony do `python3 lab.py preload …`, nie na kartę SD). Oba artefakty idą pod ten sam tag `v0.0.7.1` ( MANIFEST, punkt 6).
2. **Wersja na stronie:** `v0.0.6.0` → `v0.0.7.1` w linii 6 i w badge'u linii 18; dodać jedną linię: „v0.0.7.1 — fix milczącego pierwszego pada (A/B: 0/20 → 19/19 ważnych prób)".
3. **Uczciwość screenów:** UI w 7.1 nie zmieniło się względem 0.0.6.0, więc screeny zostają — ale linia 55 powinna brzmieć „screeny z v0.0.6.0 (UI bez zmian w 7.1)". Nie udajemy nowych screenów, których nie ma.
4. **Stabilność linków:** zawsze `releases/tag/v0.0.7.1`, nigdy `raw.githubusercontent.com/.../main/...` — `main` się zmienia przy 7.2.
5. **Aktualizacja przy 7.2:** zmienić 3–4 miejsca (title, badge, sekcja Pobierz, stopka) dopiero po nowym stabilnym tagu i jego weryfikacji. Najprościej trzymać numer wersji w jednym miejscu i generować resztę, ale przy 95 liniach HTML-a wystarczy krótka checklista.

---

## 5. Umowa koordynacyjna (7 punktów — do podpisania przez fafi)

1. **Właściciel kodu 7.1 = Jeremiasz.** Commit już jest (`5d03eb3`, czysty tracked). Heniek **nie dotyka** kodu ani plików Jeremiasza (zgodnie z `docs/KOORDYNACJA-HENIEK-JEREMIASZ.md`). **Obecny bloker nr 1:** push `main` + tag `v0.0.7.1` — decyzja fafi.
2. **Kolejność jest twarda:** (a) commit 7.1 ✅ → (b) fafi zatwierdza → (c) push `main` + tag `v0.0.7.1` → (d) upload dwóch artefaktów do Releases → (e) dopiero wtedy linki z strony → (f) Heniek publikuje stronę. Strona nie może obiecywać plików, których jeszcze nie ma.
3. **Strona = Heniek, ale bez pusha bez zgody.** Przygotowuję poprawiony `index.html`, cienki landing i czysty snapshot `gh-pages`. Commit/push wykonuje fafi albo udziela wyraźnej zgody.
4. **Releases bez `gh` CLI.** Upload idzie przez API `curl` z tokenem (`repo` / `contents: write`) albo przez przeglądarkę (github.com → Releases →Draft a new release). Heniek tokenu nie ma — wykonawcą jest fafi. Szkielet (po utworzeniu release'a i poznaniu `<RELEASE_ID>`):
   ```sh
   curl -X POST \
     -H "Authorization: Bearer $GITHUB_TOKEN" \
     -H "Content-Type: application/zip" \
     --data-binary @build/arpeggigator-v0.0.7.1.zip \
     "https://uploads.github.com/repos/OSCILLUNE/ArpeggiGator/releases/<RELEASE_ID>/assets?name=arpeggigator-v0.0.7.1.zip"
   ```
   Bezpieczniejsza wersja: najpierw utworzyć release przez UI, potem tylko dołożyć assety `curl`-em. Tokenu nie logować, nie wpisywać do repo.
5. **Blokery strony:** (i) zgoda fafi na commit/push, (ii) upload obu artefaktów do Releases (nazwy już znane z MANIFEST-u — ten bloker odpadł), (iii) kliknięcie `Settings → Pages → Source: gh-pages` w UI — tego nie zrobię z terminala.
6. **Weryfikacja po publikacji obowiązkowa:** `200` dla `/manual/` i dla 10 PNG-ów **oraz `404` dla `HANDOFF-0.0.7.0-PLAN.md`**. Ten drugi wynik jest dowodem, że wyciek z §3b został zablokowany. Bez tego testu strona nie jest „wypchnięta".
7. **Plan 7.2:** fafi dostaje plan od Jeremiasza; Heniek aktualizuje stronę dopiero po nowym, zweryfikowanym tagu. Zero pusha, zero resetów, zero dotykania fizycznej MPC, karty SD i maszyn DEV/QA bez osobnych ustaleń.

---

## 6. WYKONANIE — 2026-11-18 21:35–21:40Z (zatwierdzone przez fafi)

**Status: WYKONANE.** Push wyłącznie na `gh-pages`; `main` i tag `v0.0.7.1` nietknięte.

### 6.1 Zmiany w głównym worktree (pozostawione NIEZATWIERDZONE — `?? docs/manual/`)

| Plik | Zmiana |
|---|---|
| `docs/manual/index.html` | `v0.0.6.0` → `v0.0.7.1` w `<title>` (l. 6), badge'ach (l. 18), linku do pełnej wersji (l. 19) i stopce (l. 99); nagłówek sekcji 4 z dopiskiem „UI bez zmian w 7.1" (l. 55); usunięto sekcję „6. Dla testujących AI" (heartbeat, `mode=`, hash `/run`, `NRestarts`, ABRT/SEGV); z procedury FRZ usunięto wzmiankę o heartbeat (l. 81); **dodano sekcję „6. Pobierz"** z dwoma artefaktami, hashami SHA-256 i linkiem do tagu `v0.0.7.1` (l. 85–98). |
| `docs/manual/USERS-MANUAL-0.0.6.0.md` → **`USERS-MANUAL-0.0.7.1.md`** | Zmiana nazwy; nagłówek i wersja na 7.1 z zachowaniem provenance UI v0.0.6.0; **naprawione 10 broken links** `manual-screens-060/` → `screens/`; usunięto publiczne sekcje „Przewodnik QA" i „Provenance" (zawierały heartbeat, `NRestarts`, `/run`, numery linii źródła) — zostały w wewnętrznych handoffach; dodano sekcję „Pobierz"; usunięto wzmiankę o heartbeat w opisie FRZ. |

Uwaga dla fafi: `git status --short -- docs/manual/` → `?? docs/manual/`. Nic nie dodałem do indeksu w głównym worktree — możesz zrobić osobny commit na `main` według własnego uznania.

### 6.2 Gałąź `gh-pages`

- Worktree: `/private/var/folders/k2/h_qnbjqn0nn4ttqxm2frgckw0000gn/T/opencode/ag-gh-pages` (orphan, `git worktree add --orphan -b gh-pages`).
- Zawartość (13 plików): `index.html`, `USERS-MANUAL-0.0.7.1.md`, `.nojekyll`, `screens/01…05_{plain,shift}.png`.
- Commit: **`fbc338d393b0f447561cc742a1ef51aedaccefba`** — `site: clean GitHub Pages snapshot for v0.0.7.1`.
- Push: `git push origin gh-pages` → **sukces** (`* [new branch] gh-pages -> gh-pages`).
- `git ls-remote origin refs/heads/gh-pages` → **`fbc338d393b0f447561cc742a1ef51aedaccefba`** (zgodne z lokalnym HEAD).
- `main` = `98934ba4ac99d1bb3c06e6001fc07cfec8c44614`, tag `v0.0.7.1` = `7d652b0db6bb1ac3c42c0216921c305a0bbd87e0` — **bez zmian**.

### 6.3 Testy bezpieczeństwa (przed commitem)

| Test | Wynik |
|---|---|
| `grep -RInE '/Users/fafi\|192\.168\.155\.1\|id_ed25519\|NRestarts\|heartbeat\|PRIVATE KEY\|token\|password'` na 13 publikowanych plikach | **ZERO trafień — PASS** (jedyny wcześniejszy hit to plik wskaźnikowy `.git`, niepublikowany) |
| `HANDOFF-0.0.7.0-PLAN.md` w worktree | nieobecne → **404 — PASS** |
| Liczba plików | 13 = 1 HTML + 1 MD + 10 PNG + `.nojekyll` — **PASS** |
| Stare wersje w HTML | brak `v0.0.6.0` poza oznaczeniem provenance screenów/UI — **PASS** |

### 6.4 Pozostałe po stronie fafi (nie mogę zrobić z terminala)

1. **`Settings → Pages → Source: Deploy from a branch → gh-pages / (root) → Save`.** Bez tego strona nie zacznie się serwować.
2. Po włączeniu: `curl -I https://oscillune.github.io/ArpeggiGator/` → 200; `curl -I https://oscillune.github.io/ArpeggiGator/HANDOFF-0.0.7.0-PLAN.md` → **404**; `curl -I .../screens/01_ARP_plain.png` → 200. **Stan na 21:40Z:** oba zapytania zwracają 404, bo `Pages` nie ma jeszcze ustawionego source — to oczekiwane i nie jest błędem pusha.
3. Potwierdzić upload obu artefaktów do Releases pod tagiem `v0.0.7.1` — linki na stronie prowadzą do strony tagu (bezpieczne, nie 404), ale same pliki muszą tam być. Ja Releases nie dotykałem.
4. Ewentualnie dodać commits z `docs/manual/` na `main`.

---

## 7. JĘZYK — reguła fafi: 100% English na GitHubie (2026-11-18 ~21:50Z)

**Reguła:** wszystko, co widzi przypadkowy odwiedzający repozytorium — release notes, `README.md`, strona GitHub Pages — **po angielsku**. Wewnętrzne `docs/` (handoffy, raporty agentów, koordynacja) zostają po polsku.

### 7.1 Co przetłumaczono

| Zakres | Pliki | Status |
|---|---|---|
| Strona publiczna (główny worktree) | `docs/manual/index.html`, `docs/manual/USERS-MANUAL-0.0.7.1.md` | **PL → EN**, zmienione, **niezatwierdzone** (` M`). Nazwy plików, linki do `screens/*.png`, linki do tagu `v0.0.7.1` i hashe SHA-256 bez zmian. `<html lang="pl">` → `lang="en"`. |
| Gałąź `gh-pages` | te same 2 pliki w worktree | **PL → EN**, commit **`e3c0168f721e7aa5a5af6e33b58840edaf6f7dab`**, push **sukces** (`fbc338d..e3c0168`). |
| `README.md` (główny worktree) | `README.md` | **Niezatwierdzona poprawka** (` M`): lin. 74 `tryby, screeny z v0.0.6.0` → `modes, screenshots from v0.0.6.0`; lin. 7 wersja `0.0.7.0` → `0.0.7.1` (+ wzmianka o fixie). Reszta README była już po angielsku. |
| Treść release'u | nowy plik `docs/RELEASE-BODY-0.0.7.1-EN.md` | **Utworzony**, gotowy do wklejenia przez fafi w UI. Bez polskich fragmentów, bez ścieżek `/Users/fafi`, bez wewnętrznego komunikatu NO-GO i bez nazw agentów. |

### 7.2 Weryfikacja języka i wycieku

| Test | Wynik |
|---|---|
| `grep -E '[ąćęłńóśźżĄĆĘŁŃÓŚŹŻ]'` na obu plikach strony (główny worktree + gh-pages) | **zero trafień — PASS** |
| Skan typowych polskich słów (`bez, lub, nie, wersja, przycisk, tryb, pobierz, screeny…`) | **zero trafień — PASS** |
| `grep -E '/Users/fafi|192.168.155.1|id_ed25519|NRestarts|heartbeat|PRIVATE KEY|token|password'` na 13 publikowanych plikach gh-pages | **zero trafień — PASS** |
| `HANDOFF-0.0.7.0-PLAN.md` w gh-pages | nieobecne — **PASS** |
| Liczba plików gh-pages | 13 (`.nojekyll`, `index.html`, MD, 10 PNG) — **PASS** |
| `git ls-remote origin refs/heads/gh-pages` | **`e3c0168f721e7aa5a5af6e33b58840edaf6f7dab`** — zgodne z lokalnym HEAD |

### 7.3 Pozostaje po stronie fafi

1. Wkleić `docs/RELEASE-BODY-0.0.7.1-EN.md` do opisu release'u `v0.0.7.1` przez UI.
2. Dodać commit na `main` z `README.md` i `docs/manual/*.{html,md}` (ja nie pushuję na `main` bez zgody).
3. Ustawić `Settings → Pages → Source: gh-pages / (root)` (jeśli jeszcze nie ustawione).

**Podpisano:** Heniek 🪖☕ — Strażnik Pamięci i Ciągłości.
**Dokument powstał:** 2026-11-18 21:31Z; **uzupełniono o wykonanie:** 21:40Z; **§7 JĘZYK:** ~21:50Z. Nie pushowano na `main`; nie zmieniono `src/`, `Cargo.*`, `build/`, maszyn ani MPC.
