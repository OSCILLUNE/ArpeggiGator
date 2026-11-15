# ArpeggiGator v0.0.4.1 — Handoff Techniczny (Podsumowanie Kamieni Milowych 0.0.4.0 i 0.0.4.1)

Dokument podsumowujący realizację zadań z `docs/HANDOFF-ANTIGRAVITY-0.0.4.0.md` oraz kolejnego etapu poprawek wizualnych **v0.0.4.1**.

---

## 1. Status Gałęzi i Wydań Git

- **v0.0.4.0 (Generative Modes Engine):**
  - Commit bazowy: `dc4580e`
  - Scalony do `main`, oznaczony tagiem `v0.0.4.0` i wypchnięty do zdalnego repozytorium GitHub: `origin/main` oraz `origin/v0.0.4.0`.
  - Zawiera wdrożenie trzech nowych trybów generatywnych: **MUTATOR**, **ACORDATOR**, **ORBITATOR** z pełną kompatybilnością wsteczną z **ARP** i **MELODY**.

- **v0.0.4.1 (Mascot Visual Overhaul & Glasses Fix):**
  - Gałąź robocza: `feature/v0.0.4.1-gator-visual-overhaul` scalana do `main` z tagiem `v0.0.4.1`.
  - Usunięcie artefaktu graficznego (podwójne/błędne ramki na okularach w nowych trybach).
  - Wdrożenie dedykowanych maskotek pixel-art dla wszystkich trybów generatywnych:
    - **MUTATOR:** Dwugłowy zmutowany aligator (:D) z obiema rękami w dole na gramofonach,
    - **ACORDATOR:** Ciemnozielony aligator z klasyczną twarzą i czystymi okularami,
    - **ORBITATOR:** Aligator cały na biało w kombinezonie NASA, hełmie kosmicznym i złotej osłonie Apollo.

---

## 2. Architektura i Zrealizowane Tryby

### 2.1. MUTATOR (`src/mutator.rs`)
- **Koncepcja:** Ewolucyjny generator melodii oparty na 36 motywach z trybu MELODY.
- **Działanie:**
  - Pierwszy cykl (1. takt) frazy odtwarzany jest w 100% wiernie.
  - Od drugiego cyklu deterministyczny PRNG (xorshift32) wprowadza losowe, ale kontrolowane muzycznie mutacje w ramach skali: transpozycję wysokości, pauzy (resty), zamiany kroków.
  - 3 poziomy intensywności mutacji:
    - `SUBTLE` (15% szansy na mutację kroku),
    - `WILD` (35% szansy na mutację kroku),
    - `CHAOS` (65% szansy + losowe wydłużanie/skracanie bramki i velocity).
  - Stan `FREEZE`: zatrzymuje dalszą ewolucję i zapętla aktualnie wygenerowany wariant.
  - Ponowne uderzenie padu: resetuje bufor do oryginalnego motywu.
  - Ścisłe ramy wysokości nut: zakres transpozycji sprzętowej MPC `[-24, +24]` półtonów.

### 2.2. ACORDATOR (`src/acordator.rs`)
- **Koncepcja:** Generowanie struktur akordowych z pojedynczego uderzonego padu bazowego.
- **Kluczowe odkrycie architektoniczne (Single Voice Slot per Pad):**
  - Analiza deasemblacji firmware MPC Sample (`docs/NATIVE-DISPATCH-RESEARCH.md`, funkcja `0x2118930`) wykazała, że natywna tablica głosów dopasowuje zdarzenia po `(source, note)`.
  - W trybie 16 Levels wszystkie zdarzenia padów dzielą ten sam `PAD_SOURCE` (`0x7fff_fffa`) i ten sam numer nuty źródłowej (`36 + pad`).
  - Próba wysłania wielu równoczesnych `NoteOn` na tym samym padzie prowadzi do uszkodzenia tablicy głosów i krytycznego błędu (double-free / SIGSEGV przy zwalnianiu).
- **Rozwiązanie:**
  - Zamiast niebezpiecznej równoczesnej polifonii na jednym padzie, ACORDATOR realizuje **rozłożone w czasie schematy voicingowe (Voicing Styles)**:
    - `STRUM` (eleganckie arpeggiowane rozbicie akordu w górę/dół),
    - `PULSE` (rytmiczne, naprzemienne uderzanie prymy i składników akordu),
    - `WAVE` (falujące przenikanie interwałów).
  - 4 nastroje harmoniczne:
    - `WARM` (ciepłe akordy add9 / sus2: interwały 0, 4, 7, 14),
    - `SAD` (molowe, nostalgiczne: 0, 3, 7, 10),
    - `SPACE` (przestrzenne kwartowo-kwintowe: 0, 5, 7, 12),
    - `CINEMA` (filmowe molowe add9: 0, 2, 3, 7).
  - Gwarancja 100% stabilności i bezbłędnego zwalniania głosów.

### 2.3. ORBITATOR (`src/orbitator.rs`)
- **Koncepcja:** Kosmiczna, generatywna podróż nutowa krążąca wokół nuty uderzonego padu (Root Anchor).
- **Działanie:**
  - Wylicza kolejne kroki w oparciu o skalę pentatoniczną / bluesową / ambientową wokół prymy.
  - Posiada grawitacyjny ściągacz (Gravitational Anchor): co 4 lub 8 kroków orbita powraca do stabilnej prymy, zachowując strukturę utworu.
  - 4 atmosfery: `AMBIENT`, `SPACE`, `DARK`, `RHYTHM`.
  - 3 poziomy dynamiki orbit: `CALM` (mały rozrzut), `FLOW` (średni rozrzut), `WILD` (skoki oktawowe i żywiołowa dynamika).

---

## 3. Maskotki i Oprawa Wizualna (Redesign w v0.0.4.1)

Wszystkie maskotki zostały zoptymalizowane pod kątem czytelności na ekranie 320x240 LCD, z eliminacją artefaktów nałożonych ramek:

1. **MUTATOR (`draw_mutator_gator`):**
   - Dwie głowy aligatora (:D) wyrastające z szerokiego korpusu,
   - Lewa głowa patrzy w lewo, prawa w prawo,
   - Niezależny beat-bobbing obu głów z kłapaniem szczęk w trakcie odtwarzania,
   - Okulary cybernetyczne z cyjanowym szkłem i ruchomym blikiem,
   - **Obie ręce w dole:** lewa drapie lewy gramofon, prawa prawy gramofon na konsoli DJ-a,
   - Tło: laboratorium mutagenów z animowanym laserem skanera DNA i bulgoczącą kolbą.

2. **ACORDATOR (`draw_acordator_gator`):**
   - Klasyczna twarz z trybu MELODY (duży pysk w prawo, słuchawki, złoty łańcuch 'G'),
   - Uniesiona lewa ręka hypująca scenę, prawa miksująca na decku,
   - **Kolor ciała:** Głęboka, bogata ciemna zieleń (`0xff0d5a23` / ciemnozielony szmaragd),
   - Czyste okulary fioletowo-magenta bez obcych żółtych nakładek,
   - Tło: 4 ciepłe kolumny rezonansowe pulsujące w odcieniach zachodzącego słońca.

3. **ORBITATOR (`draw_orbitator_gator`):**
   - **Skafander NASA:** Cały w białym skafandrze kosmicznym NASA EMU,
   - Metalowy moduł telemetryczny DCM na piersi z kolorowymi diodami LED,
   - Oficjalna naszywka NASA "meatball" i flaga misji na ramieniu,
   - Biały hełm kosmiczny dopasowany do pyska z czerwonym kołnierzem,
   - Złota lustrzana osłona przeciwsłoneczna Apollo (`0xffffb703`) z rozbłyskiem słońca,
   - Białe rękawice kosmiczne z niebieskim (lewy) i czerwonym (prawy) pierścieniem rozłącznym,
   - Tło: głęboka przestrzeń kosmiczna z migoczącymi gwiazdami i orbitującą planetą.

---

## 4. Sterowanie w Interfejsie Użytkownika

Dostępne w widoku **16 Levels** na ekranie **Sample Trim (A01)**:

| Tryb | Przycisk B1 (Screen 1) | Przycisk B2 (Screen 2) | Przycisk B3 (Screen 3) |
| :--- | :--- | :--- | :--- |
| **OFF** | Przełącza na `ARP` | — | — |
| **ARP** | Przełącza na `MELODY` | Pattern (`UP`, `DOWN`, itp.) | Added notes (`+0` .. `+3`) |
| **MELODY** | Przełącza na `MUTATOR` | Długość frazy (`2BAR` .. `16BAR`) | Wariant melodii (`MEL1` .. `MEL9`) |
| **MUTATOR** | Przełącza na `ACORDATOR` | Intensywność (`SUBTLE` / `WILD` / `CHAOS` / `FREEZE`) | Motyw bazowy (`MEL1` .. `MEL9`) |
| **ACORDATOR** | Przełącza na `ORBITATOR` | Nastrój (`WARM` / `SAD` / `SPACE` / `CINEMA`) | Styl voicing (`STRUM` / `PULSE` / `WAVE`) |
| **ORBITATOR** | Przełącza na `OFF` | Atmosfera (`AMBIENT` / `SPACE` / `DARK` / `RHYTHM`) | Dynamika (`CALM` / `FLOW` / `WILD`) |

- **Pokrętło DATA:** Reguluje tempo/podział nutowy (`Rate`: `1/4` .. `1/32T`).
- **Przycisk LOOP (74):** Włącza/wyłącza tryb `HOLD` (zatrzask odtwarzania).
- **Skrót `SHIFT + Pad` (np. Pad 1 dla Full Level):** Zawsze natychmiast wyłącza aktywne głosy, czyści latche i bezpiecznie przełącza ekran MPC.

---

## 5. Wyniki Testów i Weryfikacja

1. **Testy jednostkowe:**
   - 70/70 testów zaliczonych pozytywnie (`cargo test`).
   - Pokrycie silników: `arp`, `melody`, `mutator`, `acordator`, `orbitator`, `playback`, `scheduler`, `animation_input_tests`.
2. **Weryfikacja na emulatorze `virtual-mpc-sample`:**
   - Poprawne ładowanie biblioteki `arpeggigator-audio-lab.so` w wątku audio natywnego MPC (`ac50.service`).
   - Przetestowano płynne przełączanie wszystkich 6 stanów silnika przyciskiem B1.
   - Zweryfikowano poprawność renderowania klatek bez zniekształceń i artefaktów.
   - Zarejestrowano kompletną dokumentację fotograficzną w `brain/screenshots/` (`v041_verified_*.png`).

---

## 6. Informacje o Środowisku dla Następnego Agenta

- **Główny katalog roboczy:** `/Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/arpeggigator_mutator_modes`
- **Gałąź:** `main` (zawiera tagi `v0.0.4.0` oraz `v0.0.4.1`)
- **Emulator:**
  - Panel HTTP: `http://127.0.0.1:8765`
  - Zrzut ekranu: `GET http://127.0.0.1:8765/screen.png`
  - Zdalne sterowanie: `POST http://127.0.0.1:8765/control`
  - Usługa MPC: `ssh root@127.0.0.1 -p 2022 'systemctl restart ac50'`
  - Ważne: Autorun w emulatorze ładuje bibliotekę z `/run/arpeggigator.so`. Po synchronizacji karty SD należy skopiować `/media/MPCSAMPLE/autorun/arpeggigator.so` do `/run/arpeggigator.so`.
