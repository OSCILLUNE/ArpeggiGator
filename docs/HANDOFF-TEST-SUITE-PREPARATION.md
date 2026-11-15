# Handoff: Przygotowanie i Egzekucja Pakietu Testów Eksploracyjno-Regresyjnych ArpeggiGator (DEV)

**Data utworzenia:** 15.11.2026  
**Autor dokumentu:** Wątek Zarządzający (Manager)  
**Środowisko docelowe:** **WYŁĄCZNIE ŚRODOWISKO TESTOWE DEV** (`http://127.0.0.1:8765/`, SSH port `2022`, katalog `/Users/fafi/Downloads/virtual-mpc-sample`)  
**TWARDA REGUŁA BEZWZGLĘDNA:** **ZAKAZ DOTYKANIA ŚRODOWISKA QA (port 8766)** pod żadnym pozorem bez osobnej, wyraźnej zgody użytkownika!  
**Zasada egzekucji:** **WYCHWYTYWAĆ PROBLEMY I RAPORTOWAĆ, ALE ICH NIE NAPRAWIAĆ!** Żadnego modyfikowania kodu silnika w trakcie sesji testowej.

---

## 1. Rekomendacja Modeli do Podziału Zadań

1. **Wątek Przygotowujący Testy (Architekt Scenariuszy i Narzędzi):**
   - **Rekomendowany model:** **Sol** (lub **Gemini 3.8 Pro**) z wysokim/średnim poziomem myślenia (High/Medium Reasoning).
   - **Zadanie:** Zaprojektowanie precyzyjnej matrycy kroków testowych, przygotowanie skryptu automatyzującego zrzuty całego panelu MPC i weryfikację stabilności (`ac50`).
2. **Wątek Wykonawczy (Tester / Egzekutor):**
   - **Rekomendowany model:** **Gemini 3.8 Flash** (lub **GPT 6.1-Terra**).
   - **Zadanie:** Odpalanie kolejnych scenariuszy, analiza zrzutów całego panelu (multimodalna percepcja ekranu + kolorów padów), logowanie stanu `ac50` i raportowanie anomalii.

---

## 2. Kluczowe Wymagania Techniczne Testów

### A. Zrzuty Ekranu CAŁEGO MPC SAMPLE (Wyświetlacz + Pady)
Zwykły zrzut samego ekranu 320x240 (`/screen.png`) to za mało – agent musi widzieć **kolory i stany padów 1..16**, aby wiedzieć, w którym trybie i na jakiej planszy się znajduje.

**Rekomendowane narzędzie do zrzutu całego panelu:**
W systemie dostępny jest Google Chrome w trybie headless, który generuje pełny zrzut HTML/SVG panelu:
```bash
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --headless \
  --screenshot=/Users/fafi/Downloads/ArpeggiGator/test_reports/screen_step_01.png \
  --window-size=1200,800 \
  "http://127.0.0.1:8765/"
```
Dodatkowo stan kolorów padów można pobrać w formacie JSON z panelu:
```bash
curl -s http://127.0.0.1:8765/state | python3 -c 'import sys, json; print(json.load(sys.stdin)["pads"])'
```

### B. Główny Priorytet: Stabilność i Brak Resetów (Test 2-Minutowy)
Najważniejszy cel: **żeby proces MPC nie crashował ani nie resetował się co 2 minuty**.
Przed i po każdym kroku testowym skrypt/agent musi sprawdzać:
```bash
python3 /Users/fafi/Downloads/virtual-mpc-sample/lab.py ssh "systemctl show ac50 -p NRestarts,ActiveState,SubState"
```
Jeżeli `NRestarts` wzrośnie lub `ActiveState != active`, test natychmiast rejestruje **CRASH RAPORT** (zrzut z `journalctl -u ac50 -n 50` oraz `dmesg`).

---

## 3. Zakres Testów i Matryca Scenariuszy

### CZĘŚĆ 1: Silniki Klasyczne (ARP & MELODY)

#### 1. Tryb ARP (Arpeggiator)
- **Wejście do trybu:** Wciśnięcie `16 LEVELS` na stronie `SAMPLE`. Weryfikacja: aligator w trybie ARP, pady fioletowe z zielonym padem bazowym.
- **Wzorce arpeggia (`SHIFT + B2`):**
  - Cykliczne przełączanie: `Up` -> `Down` -> `UpDown` -> `DownUp` -> `Converge` -> `Diverge` -> `Random` -> `Played` -> `Bouncing`.
  - Weryfikacja wizualna: zrzut całego panelu dla każdego wzorca.
- **Dodatkowe oktawy (`SHIFT + B3`):**
  - Przełączanie liczby dodawanych nut/oktaw (0, 1, 2, 3 oktawy).
- **Sterowanie RATE pokrętłem DATA:**
  - Zmiana podziałki: 1/4 -> 1/8 -> 1/16 -> 1/32 oraz triole (1/4T .. 1/16T).
  - Weryfikacja zaktualizowania karty `RATE` w prawym panelu.
- **Funkcja HOLD:**
  - Wciśnięcie pokrętła DATA (`PUSH DATA`): przełączenie `HOLD: ON` (zielone) / `HOLD: OFF` (czerwone).
  - Test uderzenia padów przy `HOLD: ON` – utrzymywanie grania bez trzymania palca.
- **Transport i Stop:**
  - Uruchomienie `PLAY`, zatrzymanie `STOP`, podwójny `STOP` (< 650 ms).

#### 2. Tryb MELODY
- **Przełączenie trybu (`SHIFT + B1`):** Przejście z `ARP` do `MELODY`.
- **Długość frazy (`SHIFT + B2`):** Test ustawień długości: 1 Bar, 2 Bars, 4 Bars.
- **Wariant frazy (`SHIFT + B3`):** Przełączanie wariantów melodycznych.
- **Dynamiczna zmiana prymy (Root):** Uderzanie różnych padów 0..15 i weryfikacja płynnego przejścia melodii.
- **Test ciągłości:** Granie ciągłe przez min. 2–3 minuty bez restartu usługi `ac50`.

---

### CZĘŚĆ 2: Silniki Generatywne (-TOR)

#### 1. Tryb MUTATOR
- **Przełączenie trybu (`SHIFT + B1`):** Wybór `MUTATOR`.
- **Intensywność mutacji (`SHIFT + B2`):** Zmiana wartości (np. Low / Med / High).
- **FREEZE (`SHIFT + B3`):** Włączenie zamrożenia mutacji – weryfikacja czy sekwencja staje się stałą pętlą.
- **Reset mutacji:** Ponowne uderzenie padu bazowego – weryfikacja powrotu do motywu wyjściowego.
- **Bezpieczeństwo granic:** Weryfikacja braku wychodzenia wysokości dźwięków poza zakres fizyczny padów 0..15.

#### 2. Tryb ACORDATOR
- **Przełączenie trybu (`SHIFT + B1`):** Wybór `ACORDATOR`.
- **Weryfikacja maskotki:** Aligator grający na akordeonie z animowanym miechem (wersja v0.0.4.8).
- **Akord / Nastrój (Mood) (`SHIFT + B2`):** Cyklowanie skal i nastrojów.
- **Styl strummingu (`SHIFT + B3`):** Zmiana artykulacji akordu.
- **Stres-test tempa:** Szybkie obracanie pokrętła DATA (1/4 do 1/32T) przy grającym akordzie – weryfikacja braku wiszących nut (stuck notes) i braku wycieków pamięci.

#### 3. Tryb ORBITATOR
- **Przełączenie trybu (`SHIFT + B1`):** Wybór `ORBITATOR`.
- **Wybór atmosfery / orbity (`SHIFT + B2`):** Cyklowanie atmosfer o zmiennej długości skal.
- **Dynamika (`SHIFT + B3`):** Test skrajnych dynamik (WILD / CHAOS).
- **Stres-test sterty JUCE:** Długie generowanie nut wokół prymy – weryfikacja braku kolizji wątków odrysowywania (`PaintGuard` / `UiEventGuard`) i braku SIGSEGV.

---

### CZĘŚĆ 3: Test Przejść Ekranowych i Zatrzymania

- **Live Sample Browsing:**
  - W trakcie grania w 16 Levels wyjście do ekranu `SAMPLE` (wciśnięcie `SAMPLE`).
  - Weryfikacja: dźwięk gra w tle, uderzanie w pady odsłuchuje sample.
  - Zatrzymanie podwójnym `STOP` (< 650 ms).
  - Powrót do `16 LEVELS` – sprawdzenie czystości ekranu (Zero-Ghosting) i zgaszenia padów.

---

## 4. Oczekiwany Wynik Końcowy

Wątek wykonawczy ma przygotować raport w katalogu `docs/QA-TEST-REPORT-DEV-[DATA].md` zawierający:
1. Tabelę statusów (PASS / FAIL / ANOMALIA) dla każdego trybu i podtrybu.
2. Zrzuty ekranu całego urządzenia dla kluczowych kroków (zapisane w dedykowanym katalogu `docs/qa_screens/`).
3. Licznik restartów usługi `ac50` (wymagane: `NRestarts=0` przez całą sesję).
4. Listę wychwyconych anomalii (bez prób ich poprawiania).
