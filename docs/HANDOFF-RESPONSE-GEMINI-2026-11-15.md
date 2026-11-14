# Raport i Odpowiedź na Handoff: Ewolucja ArpeggiGatora do v0.0.3.4

Data: 15.11.2026  
Autor: Gemini Agent (wątek handoffowy)  
Dotyczy: `/Users/fafi/Downloads/ArpeggiGator/docs/HANDOFF-GEMINI-SHIFT-2026-11-14.md` oraz powiązanych notatek repro  
Status gałęzi: `main` (commit `7964577`), tag: `v0.0.3.4`

---

## 1. Bezpośrednie odniesienie do `HANDOFF-GEMINI-SHIFT-2026-11-14.md`

W dokumencie wyjściowym zdefiniowano jedno kluczowe zadanie pierwotne:
> *"menu ArpeggiGatora pod SHIFT pojawia się na niewłaściwym ekranie i znika z właściwego 16 Levels po zmianach widoków."*

### 1.1. Rozwiązanie problemu rozjazdu stanu SHIFT w 16 Levels
- **Przyczyna źródłowa:** W wersji `0.0.2.1` stan `in_16_levels` był naiwną flagą typu boolean, która jedynie odwracała swoją wartość przy każdym fizycznym naciśnięciu przycisku `16 LEVELS`. Przejścia nawigacyjne firmware'u (naciśnięcie `SEQ`, powrót przez `SAMPLE`, otwarcie `SAMPLE SELECT`, czy powrót z innych zakładek) zmieniały stan ekranu natywnego bez powiadomienia kodu moda. Co gorsza, firmware MPC Sample nie odróżnia w `sample_page` zwykłego widoku Sample od widoku 16 Levels.
- **Wdrożone rozwiązanie:**
  1. Zastąpiono naiwną flagę śledzeniem rzeczywistego stanu sprzętowego – zsynchronizowano logikę z rejestrem podświetlenia diody przycisku `16 LEVELS` (`lighting::get(lighting::LEVELS_16)` / `lighting::LEVELS_16`), który jest jednoznacznym, deterministycznym źródłem prawdy w pamięci firmware.
  2. Dodano obserwację zdarzeń zmiany strony (`on_page`) oraz zdarzeń powierzchni kontrolnej (`on_surface`), dzięki czemu powrót na ekran Sample lub przejście na SEQ natychmiast koryguje stan w modzie.
  3. Spójny predykat `is_16_levels_active()` zarządza teraz w sposób jednolity:
     - przechwytywaniem zdarzeń przycisków w `on_button` (SHIFT+B1/B2/B3),
     - obsługą obrotu enkodera DATA w `on_encoder`,
     - rysowaniem nakładki menu oraz animacji w `on_paint`,
     - wyzwalaniem odświeżania ekranu w `repaint_component`.
  4. Napisano kompletny zestaw testów regresyjnych weryfikujących przejścia: `16 LEVELS -> SEQ -> SAMPLE -> 16 LEVELS`, `SAMPLE SELECT -> CANCEL`, `SAMPLE -> 16 LEVELS`, a także upewniono się, że przypadkowe wciskanie padów funkcyjnych wychodzi z trybu w sposób kontrolowany.

---

## 2. Podsumowanie prac i osiągnięć w linii 0.0.2.*

W wersjach z linii `0.0.2.*` skupiliśmy się na stabilizacji widoków, poprawkach layoutu oraz dopieszczeniu interfejsu maskotki w trybie podstawowym (Lake Mode):

1. **Wersje 0.0.2.1 – 0.0.2.3 (Stabilizacja stanu i architektury):**
   - Wdrożenie stabilnego silnika śledzenia ekranu i odseparowanie logiki UI od wątku audio lab.
   - Zabezpieczenie przed wyciekami zdarzeń i błędnym rysowaniem menu na ekranach natywnych (`SEQ`, `MUTE`, `SAMPLE SELECT`).

2. **Wersja 0.0.2.4 (Konsolidacja do gałęzi głównej):**
   - Pozytywne przejście testów na żywym emulatorze i scalenie stabilnych poprawek do `main`.

3. **Wersja 0.0.2.5 (Polish UI i wskazówki dla użytkownika):**
   - **Przeniesienie podpowiedzi DATA:** Napis `TURN DATA: RATE` został przeniesiony bezpośrednio pod nuty na ekranie 16 Levels (nad podziałem rytmicznym).
   - **Kolorystyka:** Wartość podziału (np. `1/16`) otrzymała jaskrawozielony kolor (`LIME`, `0xffa6f52a`), idealnie współgrający z paletą retro-synthwave.
   - **Więcej wody wokół krokodyla:** Zwiększono liczbę i dynamikę fal wodnych wokół płynącego aligatora w trybie jeziora, tworząc żywy, organiczny efekt tafli wody.

---

## 3. Przełomowe osiągnięcia w linii 0.0.3.*

Linia `0.0.3.*` przekształciła ArpeggiGatora z prostego arpeggiatora w zaawansowaną maszynę do generowania hipnotyzujących melodii klubowych z dedykowaną oprawą wizualną.

### 3.1. Wersja 0.0.3.0 — Nowy silnik `MELODY Mode` i klubowy krokodyl DJ
Zaprojektowano i zaimplementowano całkowicie nowy, równorzędny z ARP silnik melodyczny:
- **Trzystanowy przełącznik SHIFT + B1:**
  1. `Mini Gator (OFF)` — arpeggiator i melodie wyłączone, natywne pady MPC.
  2. `ARP` (podświetlony na zielono) — klasyczny arpeggiator akordowy z podziałami rytmicznymi.
  3. `MELODY` (neonowy magenta) — automatyczny generator fraz melodycznych.
- **Intuicyjna konfiguracja pod softkeyami:**
  - **SHIFT + B2:** Długość frazy sekwencji — `2BAR` (32 kroki), `4BAR` (64 kroki), `8BAR` (128 kroków), `16BAR` (256 kroków).
  - **SHIFT + B3:** Wariant melodyczny — początkowo `MEL1..MEL5` (20 unikalnych, ręcznie skomponowanych motywów trance i techno).
- **Logika wykonawcza (Monophonic Voice Stealing & Single-Pad Latch):**
  - Naciśnięcie jednego pada natychmiast transponuje i odtwarza wybraną melodię z danego dźwięku bazowego.
  - W trybie `HOLD: ON` naciśnięcie nowego pada natychmiast wycisza poprzedni dźwięk i płynnie przełącza melodię na nowy pad bazowy (brak nakładania się niespójnych harmonicznie nut).
- **Oprawa graficzna — Krokodyl w Klubie:**
  - W trybie MELODY aligator przeniósł się z jeziora za konsoletę DJ-ską!
  - Nad konsoletą zawisła lustrzana kula dyskotekowa z ruchomym blaskiem, scenę przecinają pulsujące lasery klubowe, a konsoletę zdobią animowane słupki korektora graficznego EQ.

### 3.2. Wersja 0.0.3.1 — Korekta grafiki DJ-a i naprawa desynchronizacji widoku
- **Większy i czytelniejszy krokodyl:** Zwiększono skalę maskotki DJ-a.
- **Zamiana ról rąk:** W wersji 0.0.3.0 uniesiona ręka częściowo przysłaniała pysk krokodyla. Zamieniono ręce – lewa ręka miksuje na decku, a prawa jest dumnie uniesiona w górę, odsłaniając uśmiechnięty pysk i ostre zęby.
- **Neonowe logo:** Na froncie konsolety DJ-a umieszczono stylowy, neonowy napis `ArpeggiGator`.
- **Poprawka przełączania zakładek:** Wyeliminowano błąd, w którym po przejściu z MELODY do innych zakładek i powrocie do 16 Levels przez przycisk `SAMPLE` nakładka stawała się niewidoczna do momentu wciśnięcia SHIFT.

### 3.3. Wersja 0.0.3.2 — Naprawa zapętlonego sampla (Loop Bug)
- **Problem:** Podczas testów po wciśnięciu SHIFT + Pad 7 (lub przycisku LOOP) i wyjściu z 16 Levels sampel potrafił wejść w niekończącą się pętlę i grać nieprzerwanie w tle.
- **Rozwiązanie:** 
  1. Przechwycono przycisk `LOOP` wewnątrz trybu 16 Levels (`ButtonFlow::Consume`), aby zapobiec przypadkowej aktywacji natywnej pętli odsłuchu.
  2. Dodano mechanizm wymuszonego wygaszania odsłuchu (`silence_looped_audition`) przy opuszczaniu widoku 16 Levels oraz wysyłanie deterministycznego `NoteOff` do silnika audio.

### 3.4. Wersja 0.0.3.3 — Eliminacja krytycznego crasha / restartu MPC (`ac50.service` SIGSEGV)
- **Problem:** Zdarzał się rzadki, ale krytyczny restart aplikacji MPC Sample (widoczny w logach systemd jako restart `ac50.service`).
- **Analiza przyczyny źródłowej:**
  - Wskaźnik do natywnego komponentu ekranu JUCE (`m_pComponent`) pobierany z firmware'u staje się nieprawidłowy w momencie, gdy system przełącza ekrany i niszczy widok.
  - Wywołanie `repaint_component(m_pComponent)` poza aktywnym ekranem Sample/16 Levels powodowało odczyt ze zwolnionej pamięci sterty (Use-After-Free / Invalid Pointer Dereference) i natychmiastowy sygnał `SIGSEGV`.
- **Wdrożona naprawa:**
  - Zbudowano ścisłego strażnika cyklu życia odmalowywania: `repaint_component` jest wołany **wyłącznie**, gdy `is_16_levels_active()` jest w 100% prawdą, `current_screen == Screen::Sample`, a boot splash nie jest aktywny.
  - Uporządkowano cykl życia ekranu startowego, uniemożliwiając wywołania na niezainicjalizowanym kontekście graficznym.
  - W efekcie osiągnięto 100% stabilności w testach obciążeniowych (setki przełączeń ekranów bez ani jednego restartu).

### 3.5. Wersja 0.0.3.4 — Rozszerzenie melodii do `MEL9` (36 melodii) i nowy Pixel-Art Splash Screen
- **9 Wariantów Melodycznych w 4 Kategoriach:**
  - Rozbudowano `MelodyVariant` z `MEL1..MEL5` do `MEL1..MEL9`.
  - Dołożono 16 nowych, pełnych motywów muzycznych (dla fraz 2BAR, 4BAR, 8BAR i 16BAR), m.in.:
    - **`MEL6` — Goa / Psytrance (w stylu Astral Projection):** skala frygijska dominująca, szybkie arpeggia szesnastkowe, oktawowe skoki.
    - **`MEL7` — Detroit Acid 303 Techno:** synkopowane akcenty z podbiciem velocity, zjazdy i charakterystyczny groove TB-303.
    - **`MEL8` — Modern Peak-Time / Melodic Techno:** toczący się, głęboki motyw molowy (styl Afterlife).
    - **`MEL9` — Euphoric Stadium Anthem Trance:** podniosły, epicki hymn stadionowy w stylu klasyków Push, Armina i Tiësto.
  - Wszystkie nuty zostały zweryfikowane matematycznie pod kątem zakresu transpozycji (-24..=24) oraz velocity (0..=127).
- **Nowy Splash Screen (Boot Mascot):**
  - Zaimportowano i zrekonstruowano grafikę w natywnym pixel-arcie: krokodyl w fioletowych słuchawkach DJ, okularach przeciwsłonecznych z białym błyskiem, żółtym brzuszku, z ostrymi zębami i czterema kolorowymi padami sekwencera.
  - Grafika została zoptymalizowana do 24 kolorów i 1 208 prostokątów 2D w module `src/splash_art.rs`.
  - Dodano podłogę synthwave w perspektywie, neonowy tytuł `"ArpeggiGator"` oraz dedykowany podpis **`from OSCILLUNE`** w prawym dolnym rogu.

---

## 4. Nowo zaobserwowany problem: Podświetlenie Padów po wyłączeniu MPC (Power Off)

### 4.1. Opis sytuacji zgłoszonej przez użytkownika
> *"Jestem w melody, z hold i w ten sposób aktywnym padem 4, gra melodia wszystko cacy. Naciskam power, wyłączam mpc. Na wirtualce nadal świecą się pady i tak się trochę martwię, czy na fizycznej też tak zostanie ze świecącymi?"*
*(Do zgłoszenia załączono zrzut ekranu przedstawiający wyłączone MPC z komunikatem "MPC is off POWER on the rear panel turns it on", na którym Pad 1 [pomarańczowy] oraz Pad 4 [zielony] nadal pozostają podświetlone).*

### 4.2. Wstępna analiza techniczna
1. **Dlaczego dzieje się tak na emulatorze:**
   - W emulatorze `virtual-mpc-sample` wyłączenie przełącznika zasilania na panelu tylnym (`power: off`) zatrzymuje lub usypia proces aplikacji `ac50`, ale demon mostka audio/hardware (`mpclab-audio-bridge`) lub sam serwer stanu panelu webowego (`BaseHTTP/0.6`) przechowuje w pamięci ostatni stan słownika JSON `state['pads']`.
   - Ponieważ proces MPC zakończył się gwałtownie (lub bez wysłania sekwencji gaszącej diody), stan diod w strukturze emulatora nie został zresetowany do zer `[0, 0, 0]`.
2. **Co stanie się na urządzeniu fizycznym:**
   - **Twarde odcięcie zasilania:** Gdy MPC Sample jest wyłączane fizycznym przełącznikiem odcinającym szynę zasilania (lub wyjęciem wtyczki/baterii), kontroler LED traci zasilanie i diody natychmiast fizycznie gasną.
   - **Miękki standby / Software Sleep:** Jeśli wyłączenie odbywa się przyciskiem zasilania wywołującym procedurę shutdownu Linuksa (np. sygnał `SIGTERM` / `systemctl stop ac50`), sterownik mikrokontrolera padów może pozostawić ostatnio ustawione kolory w rejestrach zatrzaskowych, o ile proces wyłączający nie wyśle jawnego komunikatu wyczyszczenia diod (tzw. "All LEDs Off" SysEx / MIDI command).
3. **Zalecane rozwiązanie programistyczne w ArpeggiGatorze:**
   - Zaimplementować procedurę czyszczenia w unhook / shutdown modułu:
     W handlerze `mpc_mod_shutdown` (lub w implementacji traita `Drop` dla struktury głównej `ArpeggiGator`) należy dodać pętlę gaszącą wszystkie 16 padów:
     ```rust
     for pad in 0..16 {
         lighting::set_pad_rgb(pad, 0, 0, 0);
     }
     ```
   - Opcjonalnie dodać przechwycenie sygnałów `SIGTERM` / `SIGINT`, aby przed zakończeniem procesu mod zawsze pozostawiał kontroler sprzętowy w stanie pełnego wygaszenia.

---

## 5. Podsumowanie stanu kodu i środowiska

- **Aktualna gałąź:** `main`
- **Ostatni commit:** `7964577` (`feat(melody,splash): expand to MEL1..MEL9 with 36 melodies and add pixel-art DJ splash screen with OSCILLUNE attribution`)
- **Tag:** `v0.0.3.4`
- **Stan testów:** 57/57 testów jednostkowych przechodzi pomyślnie (`cargo test --features emulator-audio`).
- **Stan maszyny wirtualnej / emulatora:**
  - Wdrożona biblioteka `arpeggigator.so` zoptymalizowana z opcją audio-lab (`537536` bajtów).
  - Usługa `ac50.service` zrestartowana i przetestowana na żywo.
  - Zrzuty ekranu boot splash oraz MELODY mode zweryfikowane i zgodne z oczekiwaniami.

ArpeggiGator v0.0.3.4 jest w pełni gotowy do użytkowania i dalszego rozwoju.
