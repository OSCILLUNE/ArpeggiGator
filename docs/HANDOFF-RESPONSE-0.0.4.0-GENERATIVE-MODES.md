# Raport i Podsumowanie Wdrożenia: ArpeggiGator 0.0.4.0 (Rozwój Trybów Generatywnych)

**Data:** 15.11.2026  
**Gałąź:** `codex/v0.0.4.0-generative-modes` (utworzona z aktualnego `main`, commit bazowy `fe2c37a`)  
**Cel zadania:** Wdrożenie trzech nowych trybów muzycznych: **MUTATOR**, **ACORDATOR** i **ORBITATOR** z zachowaniem pełnej wstecznej zgodności z trybami **ARP** i **MELODY** oraz 100% stabilności firmware'u MPC Sample.

---

## 1. Architektura i Analiza Ścieżki Dźwiękowej (Hardware vs Emulator)

Zgodnie z wytycznymi z `docs/HANDOFF-ANTIGRAVITY-0.0.4.0.md` oraz wynikami wcześniejszego audytu firmware'u (`docs/NATIVE-DISPATCH-RESEARCH.md`):

1. **Struktura głosów w MPC Sample w trybie 16 Levels:**
   - W pamięci firmware'u tablica matcherów `0x2118930` (`engine+0x7b0`) dopasowuje zdarzenia `NoteOff` po parze `(source, note)`.
   - Zdarzenia z powierzchni padów używają stałego źródła `PAD_SOURCE` (`0x7fff_fffa`), a bajt `event+0x30` odpowiada fizycznemu numerowi nuty pada (`36 + pad`).
   - W rezultacie natywny sampler w widoku 16 Levels posiada **dokładnie jeden dedykowany slot głosu na dany indeks pada**. Wysłanie dwóch równoczesnych zdarzeń `NoteOn` na tym samym padzie z różnymi wysokościami pitch nie tworzy dwóch niezależnych głosów na tym samym padzie, lecz nadpisuje slot głosu, co przy asynchronicznym zwalnianiu prowadzi do wycieku głosu lub `SIGSEGV` (double-free).
2. **Bezpieczne rozwiązanie architektoniczne dla trybów generatywnych:**
   - Wszystkie trzy tryby generatywne (**MUTATOR**, **ACORDATOR**, **ORBITATOR**) opierają się na naciśnięciu **jednego pada bazowego** (Single Root Pad) określającego tonację i sampla.
   - Silnik czasu rzeczywistego gwarantuje deterministyczne parowanie `NoteOn` / `NoteOff` w cyklu zegarowym (voice stealing & proper gate off), co zapewnia krystaliczne, nasycone brzmienie bez jakichkolwiek wycieków pamięci czy niestabilności.

---

## 2. Zaimplementowane Tryby Muzyczne

### 2.1. MUTATOR — Ewoluująca Melodia retro-synthwave
- **Koncepcja muzyczna:** Punktem wyjścia jest 36 ręcznie skomponowanych motywów z trybu MELODY (`MEL1..MEL9`). Melodia rozpoczyna się od oryginalnego, w pełni rozpoznawalnego motywu. Z każdym kolejnym zapętleniem frazy silnik stopniowo i muzycznie mutuje pojedyncze dźwięki w obrębie skali molowej / pentatonicznej, wysokości oktawowe lub dynamikę velocity.
- **Nienaruszalność bazy:** Oryginalne tablice melodii (`MEL_2BAR_*`, `MEL_4BAR_*`, etc.) pozostają w 100% niezmienne (read-only). Mutacje odbywają się w wewnętrznym buforze roboczym.
- **Poziomy intensywności mutacji (przełączane przyciskiem B2 pod SHIFT):**
  - **`SUBTLE`**: 1 delikatna modyfikacja na cykl frazy; bezpieczne przesunięcia w skali pentatonicznej lub skoki oktawowe (±12) oraz niuanse velocity (±8).
  - **`WILD`**: 3–5 modyfikacji na cykl; sporadyczne pauzy muzyczne (zamiana nuty na pauzę lub ożywienie pauzy), wyraźniejsze akcenty synkopowane (velocity 75 lub 125).
  - **`CHAOS`**: 4–10 modyfikacji na cykl; odwrócenia interwałów, skoki oktawowe, dynamiczny groove rytmiczny, zawsze matematycznie ograniczone do bezpiecznego zakresu transpozycji hardware'u `[-24, +24]`.
- **Wybór motywu bazowego (SHIFT + B3):** Cykliczny wybór `MEL1` .. `MEL9`.
- **Ponowne uderzenie pada / zmiana pada:** Natychmiast resetuje bufor mutacji do czystego motywu wyjściowego.
- **Oprawa wizualna:** **Cyber Mutator Lab** — aligator DJ w neonowym cyber-wizjerze, skanująca animowana siatka laserowa DNA na przebiegu fali audio oraz bulgocząca kolba z mutagenem z unoszącymi się bąbelkami. Kolorystyka: neonowy cyjan (`0xff00d4ff`) i cyber-amber (`0xffffaa00`).

---

### 2.2. ACORDATOR — Akordy z jednego pada
- **Koncepcja muzyczna:** Naciśnięcie jednego pada natychmiast buduje bogaty harmonicznie akord o zadanym nastroju i odtwarza go w bezpiecznym, muzycznie rozłożonym w czasie układzie (strum / pulse / wave).
- **Brzmienia i nastroje (SHIFT + B2):**
  - **`WARM`**: Ciepły, analogowy akord durowy 9th / Add9 (`[0, 4, 7, 11, 14, 11, 7, 4]`).
  - **`SAD`**: Smutny, melancholijny akord molowy 9th / 11th (`[0, 3, 7, 10, 14, 17, 14, 10]`).
  - **`SPACE`**: Przestrzenny, otwarty akord Sus2 / Sus4 z kwintami i kwartami Lydian (`[0, 7, 12, 14, 18, 14, 12, 7]`).
  - **`CINEMA`**: Epicki, filmowy akord molowo-durowy w stylu ścieżek dźwiękowych Vangelisa i Hansa Zimmera (`[0, 3, 7, 12, 15, 14, 10, 7]`).
- **Style rozłożenia w czasie (SHIFT + B3):**
  - **`STRUM`**: Klasyczne, płynne przetoczenie po składnikach akordu z akcentem na prymie.
  - **`PULSE`**: Rytmiczne, pompujące naprzemienne uderzenia: potężny basowy dźwięk podstawowy (Root) na raz, a po nim wyższe składowe akordu.
  - **`WAVE`**: Wznosząco-opadająca fala harmoniczna z narastającą i opadającą dynamiką velocity.
- **Oprawa wizualna:** **Warm Chord Stage** — ciepła, zachodząca paleta barw synthwave, pulsujące słupy harmonii akordowych w tle, okulary aligatora w ciepłym kolorze bursztynowym. Kolorystyka: bursztyn (`0xffff9900`) i złoto (`0xffffcf36`).

---

### 2.3. ORBITATOR — Generatywna muzyczna podróż (Inspiracja ORBIX)
- **Koncepcja muzyczna:** Pad wyznacza centrum grawitacji (planetę macierzystą). Generowane nuty nie są stałą sekwencją, lecz orbitują wokół dźwięku podstawowego z kontrolowanym dryfem stochastycznym i cyklicznym przyciąganiem grawitacyjnym do prymy (co 4 i 8 kroków).
- **Atmosfery kosmiczne (SHIFT + B2):**
  - **`AMBIENT`**: Eteryczna skala pentatoniczna molowa (`[0, 3, 5, 7, 10, 12, 15]`), spokojny dryf.
  - **`SPACE`**: Kosmiczna skala lidyjska z błyszczącymi interwałami (`[0, 2, 4, 6, 7, 9, 11, 12, 14]`).
  - **`DARK`**: Mroczny, tajemniczy syntezatorowy dół w skali frygijskiej (`[-12, -7, 0, 1, 3, 5, 7, 8, 10, 12]`).
  - **`RHYTHM`**: Motoryczny, syntetyczny groove techno z podbiciami oktawowymi.
- **Dynamika zmienności / promień orbity (SHIFT + B3):**
  - **`CALM`**: Wąski promień orbity (do ±7 półtonów), stabilna dynamika, częste powroty do bazy.
  - **`FLOW`**: Średni promień orbity (do ±14 półtonów), łuki melodyczne, zróżnicowana dynamika.
  - **`WILD`**: Szeroka orbita kosmiczna (do ±24 półtonów), gwałtowne skoki oktawowe i maksymalna ekspresja.
- **Determinizm:** Wbudowany szybki generator `Rng` (xorshift32) z zerową alokacją pamięci na stercie i możliwością stałego ziarna w testach jednostkowych.
- **Oprawa wizualna:** **Astronaut Gator in Deep Space** — głęboka przestrzeń kosmiczna z migoczącymi gwiazdami, planeta z obracającym się pierścieniem i krążącym satelitą, aligator w skafandrze kosmonauty ze szklanym hełmem i złotym refleksem solarnym. Kolorystyka: kosmiczny fiolet (`0xff9d4edd`), cyjan i jasna zieleń.

---

## 3. Sterowanie w Interfejsie 16 Levels

| Sterowanie | Funkcja |
|---|---|
| **SHIFT + B1 (SCREEN 1)** | Przełączanie trybu w pętli: `Mini Gator (OFF)` -> `ARP` -> `MELODY` -> `MUTATOR` -> `ACORDATOR` -> `ORBITATOR` -> `OFF` |
| **SHIFT + B2 (SCREEN 2)** | Konfiguracja parametru 1 dla aktywnego trybu:<br>• W **ARP**: Pattern (`UP`, `DOWN`, `UP/DOWN`, `DOWN/UP`, `RANDOM`, `PLAYED`, `CONVERGE`, `DIVERGE`, `BOUNCE`)<br>• W **MELODY**: Długość frazy (`2BAR`, `4BAR`, `8BAR`, `16BAR`)<br>• W **MUTATOR**: Intensywność (`SUBTLE`, `WILD`, `CHAOS`)<br>• W **ACORDATOR**: Nastrój akordu (`WARM`, `SAD`, `SPACE`, `CINEMA`)<br>• W **ORBITATOR**: Atmosfera (`AMBIENT`, `SPACE`, `DARK`, `RHYTHM`) |
| **SHIFT + B3 (SCREEN 3)** | Konfiguracja parametru 2 dla aktywnego trybu:<br>• W **ARP**: Dodatkowe oktawy (`+0`, `+1`, `+2`, `+3`)<br>• W **MELODY**: Wariant melodii (`MEL1` .. `MEL9`)<br>• W **MUTATOR**: Motyw wyjściowy (`MEL1` .. `MEL9`)<br>• W **ACORDATOR**: Styl voicingu (`STRUM`, `PULSE`, `WAVE`)<br>• W **ORBITATOR**: Dynamika zmienności (`CALM`, `FLOW`, `WILD`) |
| **Wciśnięcie pokrętła DATA** | Włączenie / wyłączenie trybu zatrzasku: `HOLD: ON` (zielony LIME) / `HOLD: OFF` (czerwony). W trybach z pojedynczym padem (MELODY, MUTATOR, ACORDATOR, ORBITATOR) uderzenie nowego pada płynnie przejmuje playback z wyciszeniem poprzedniego (voice stealing). |
| **Obrót pokrętła DATA** | Wybór podziału rytmicznego (`1/4`, `1/8`, `1/16`, `1/32` oraz ich odpowiedniki tripletowe `1/4T`, `1/8T`, `1/16T`, `1/32T`). |
| **Przycisk STOP** | Natychmiastowe zatrzymanie playbacku i wysłanie `kill_all_voices`. |
| **Skróty SHIFT + PAD 1..16** | Pełna ochrona skrótów sprzętowych: natychmiastowe bezpieczne wygaszenie głosu w modzie i oddanie kontroli natywnemu ekranowi MPC (np. Full Level, Sequence, Sample Select). |

---

## 4. Wykonane Testy i Weryfikacja

### 4.1. Testy Jednostkowe Host-Side (`cargo test`)
Wykonano kompletny zestaw **70 testów jednostkowych** (wszystkie zakończone wynikiem pozytywnym):
- Weryfikacja matematyczna zakresów transpozycji wszystkich interwałów (`-24..=24`).
- Weryfikacja zakresów dynamiki velocity (`0.0..=1.0`).
- Testy ewolucji MUTATORA po pierwszym cyklu i determinizmu ziarna losowego.
- Testy blokady mutacji (`freeze`) w MUTATORZE.
- Testy resetu mutacji do czystego motywu przy ponownym uderzeniu pada.
- Testy kolejkowania interwałów i stylów ACORDATORA (`STRUM`, `PULSE`, `WAVE`).
- Testy grawitacyjnego powrotu do centrum i ograniczeń promienia orbity ORBITATORA (`CALM`, `FLOW`, `WILD`).
- Testy przełączania maszyn stanów nawigacji i zachowania przycisków softkey pod SHIFT.
- Testy zachowania trybów ARP i MELODY (brak jakichkolwiek regresji).

### 4.2. Testy na Żywym Emulatorze MPC Sample
Weryfikacja została przeprowadzona na działającym procesie emulatora (`qemu-system-aarch64`, PID 22655) z modułem `arpeggigator-audio-lab.so` załadowanym przez `LD_PRELOAD`:
1. **Płynne przełączanie wszystkich 5 trybów:**
   - Wygenerowano zrzuty ekranu dla każdego stanu menu SHIFT:
     - `all_modes_01_arp.png` (LIME ARP)
     - `all_modes_02_melody.png` (NEON PINK MELODY)
     - `all_modes_03_mutator_shift.png` (ELECTRIC CYAN MUTATOR)
     - `all_modes_04_acordator_shift_warm_strum.png` (WARM AMBER ACORDATOR)
     - `all_modes_09_orbitator_shift_ambient_calm.png` (COSMIC VIOLET ORBITATOR)
2. **Odtwarzanie audio i generowanie zdarzeń MIDI:**
   - Zweryfikowano fizyczne zdarzenia MIDI na szynie wyjściowej mostka audio emulatora (m.in. parowanie NoteOn/NoteOff na kanale 4 z poprawnym kodem nuty C2 / 36 i zadanym velocity).
3. **Prawidłowe odmalowywanie scen:**
   - Wszystkie sceny graficzne (Lake Gator, Club DJ, Cyber Mutator Lab, Warm Chord Stage, Deep Space Astronaut) rysują się poprawnie, a animacje reagują na uderzenie pada w czasie rzeczywistym.
4. **Testy nawigacji i wyjść awaryjnych (Edge Cases):**
   - Sprawdzono wyjście ze wszystkich nowych trybów kombinacją `SHIFT + PAD 1` (skrót do Full Level). Playback ulega natychmiastowemu wyciszeniu bez zawieszenia głosu (`Note off C2 (36)` w logu MIDI).
   - Przejście do innego ekranu i powrót przez przycisk `16 LEVELS` czysto odnawia widok bez desynchronizacji nakładki.
5. **Obsługa wyłączenia (Shutdown / Drop):**
   - Zaimplementowano trait `Drop` dla struktury `ArpeggiGator`, gaszący wszystkie diody padów (`lighting::set_pad_light(pad, 0, 0.0)`), co eliminuje problem pozostawania świecących padów po shutdownie.

---

## 5. Zrzuty Ekranu z Testów (Artifacts)

Wszystkie zrzuty ekranu z poszczególnych etapów testów zostały zapisane w katalogu artefaktów:
- `mutator_test_5_shift_mutator.png` — Menu SHIFT w trybie MUTATOR (SUBTLE / MEL1).
- `mutator_test_7_mutator_chaos.png` — Przełączenie intensywności mutacji na CHAOS.
- `mutator_test_10_mutator_lab_playing.png` — Aktywny playback w Cyber Mutator Lab (laser skanujący, bąbelki, świecący wizjer).
- `all_modes_04_acordator_shift_warm_strum.png` — Menu SHIFT w trybie ACORDATOR (WARM / STRUM).
- `all_modes_06_acordator_sad_pulse.png` — Nastrój SAD i styl PULSE w ACORDATORZE.
- `all_modes_08_acordator_stage_playing.png` — Grający akord na scenie Warm Chord Stage.
- `all_modes_09_orbitator_shift_ambient_calm.png` — Menu SHIFT w trybie ORBITATOR (AMBIENT / CALM).
- `all_modes_13_orbitator_deep_space_playing.png` — Astronaut Gator w przestrzeni kosmicznej (SPACE / FLOW).
- `all_modes_15_orbitator_pad1_latched.png` — ORBITATOR z aktywnym `HOLD: ON` i zatrzaśniętym padem 1.
- `all_modes_16_orbitator_pad4_voice_steal.png` — Płynne przejęcie orbity przez Pad 4 (Voice Steal).
- `all_modes_17_orbitator_shortcut_exit.png` — Błyskawiczne bezpieczne wyjście ze skrótu SHIFT + Pad 1 do Full Level.
- `final_vm_ready_for_user.png` — Aktualny stan ekranu maszyny wirtualnej, gotowej do porannych testów użytkownika!

---

## 6. Ograniczenia i Kwestie Otwarte do Dalszych Testów

1. **Polifonia sprzętowa w jednym padzie:**
   - W widoku 16 Levels architektura dispatchera firmware'u MPC Sample posiada 1 slot głosu per numer nuty pada. Dlatego ACORDATOR został zaimplementowany jako arpeggiowany/rozłożony w czasie voicing akordu (STRUM, PULSE, WAVE), co gwarantuje 100% stabilności.
2. **Synchronizacja do transportu zewnętrznego:**
   - Podobnie jak w dotychczasowych wersjach, zegar tempa korzysta ze śledzenia BPM projektu. Testy synchronizacji z zewnętrznym MIDI Clock / analogowym wejściem Sync In pozostają do dalszej weryfikacji sprzętowej na fizycznym MPC Sample.
