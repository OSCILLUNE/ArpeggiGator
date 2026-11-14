# ArpeggiGator — przekazanie dla nowego wątku

Data: 14.11.2026. Rozmawiaj z użytkownikiem po polsku. Projekt:
`/Users/fafi/Downloads/ArpeggiGator`. Użytkownik oczekuje dalszej samodzielnej
pracy i konkretnych testów; nie przedstawiaj hipotez ani pomiarów jednostkowych
jako udowodnionej naprawy.

## Stan repozytorium i wersji

- `main` i tag `v0.0.2.1` na GitHubie wskazują na `87a95c6` (fast-forward z
  `a7f010c`, bez przepisywania historii). Wcześniej bieżący kod był tylko na
  `experiment/repaint-cadence-observer`; szczegóły w nieśledzonej notatce
  `docs/NOTATKA-SYNC-MAIN.md` od Antigravity.
- Badania synchronizacji znajdują się na osobnej gałęzi
  `codex/transport-sync-research`, opartej na `v0.0.2.1`. Funkcja
  `transport-sync-experiment` jest niedokończona i **nie jest częścią**
  `main` ani wersji 0.0.2.1. Nie traktuj jej jako wersji gotowej użytkownikowi.
- Wersja 0.0.2.1 ma audio tylko w opcjonalnym buildzie emulatora;
  standardowa paczka sprzętowa go nie włącza. `scripts/build.sh` automatycznie
  synchronizuje standardowy build na kartę działającego emulatora i może
  nieoczekiwanie zastąpić build audio. Użyj `scripts/build-emulator-audio.sh`
  i `scripts/sync-emulator-autorun.py --audio-lab` tylko świadomie.
- Nie dodawaj ani nie usuwaj przypadkowo istniejących nieśledzonych `.DS_Store`,
  starszych handoffów, instrukcji QA ani notatki Antigravity.

## Co użytkownik zatwierdził

Nagrywanie ARP ma trzymać siatkę transportu. Dodanie i usunięcie pada ma
wchodzić na kolejnym kroku; odsłuch i zapis mają korzystać z tych samych
wygenerowanych nut. Punktem odniesienia jest stockowy NOTE REPEAT. Użytkownik
chce móc dogrywać trzeciego pada do dwóch już grających bez chaosu. PLAY ma
odtwarzać zapis, a aktywne pady mogą grać równolegle; trzeba uniknąć duplikacji
fizycznych uderzeń i ponownej arpeggiacji nut odtwarzanych z sekwencji.

SHIFT+B3 dodaje 0–3 nuty: kolejne oktawy powyżej pada o najniższym numerze.
SHIFT+B2 układa cały zestaw, łącznie z tymi nutami, według wybranego wzorca.
Nie wracaj do wariantu powtarzającego cały akord w kolejnych oktawach.

## Środowisko i ostrożność

- Bieżące testy: `/Users/fafi/Downloads/virtual-mpc-sample`, panel
  `http://127.0.0.1:8765/`, SSH wg tamtejszego `lab.py`, firmware MPC Sample
  1.3.0.12. Najpierw sprawdź faktyczne porty, PID, zamontowaną kartę i hash:
  maszyna mogła zostać zrestartowana lub zmieniona między wątkami.
- Antigravity przygotował oddzielną, czystą QA w
  `/Users/fafi/Downloads/virtual-mpc-sample-qa`. **Na razie jej nie używaj ani
  nie zmieniaj.** Notatka QA wspomina porty, ale również podaje niejednoznaczny
  port panelu; sprawdzaj faktyczny stan przed późniejszym użyciem.
- Oryginalny `/data/TempProjects` został przywrócony po ostatnim teście; usługa
  `ac50` była aktywna, PID 817, `NRestarts=0`. Ostatnio na karcie i w `/run`
  działał build audio 0.0.2.1 o SHA-256
  `16095f5c738bf9af45d5e79e6cfab0bc153abecb6c1e4389937376e839772f71`.
  To obserwacja z chwili zakończenia, nie gwarancja obecnego stanu.
- Nagrywanie wyłącznie na osobnych kopiach dyskowych projektu, nie na
  oryginalnym `/data/TempProjects`. Kopie już istnieją: `/data/ag-record-test`,
  `/data/ag-eval-002`, `/data/ag-record-0021-arp`,
  `/data/ag-record-0021-repeat`, `/data/ag-transport-test-arp` oraz
  `/data/ag-visual-0021`. Nie usuwaj ich ani zasobów Internal/External.
  Zatrzymaj `ac50`, podłącz kopię `mount --bind`, uruchom usługę; po próbie
  zatrzymaj usługę, odmontuj i uruchom ją z oryginałem. Potwierdź `mount`.
- Nie używaj `/run` jako miejsca kopii dla prób nagrywania: tmpfs 387 MB
  spowodował komunikat MPC o braku miejsca na historię cofania. W testach
  dyskowych `/data` miało około 2,3 GB wolnego miejsca.
- Tymczasowe ustawienie kernelowego `core_pattern` zostało przywrócone do
  `|/usr/bin/az01-coredump %P %s %t %c %e`. Jeśli je zmienisz, przywróć
  bez względu na wynik. Nie resetuj VM ani nie restartuj aplikacji bez potrzeby;
  wcześniej użytkownik zauważył zamierzony restart i nie był to dowód awarii.

## Sprawdzone wyniki — od prostszych zadań do trudniejszych

### 1. Uporządkowanie i weryfikacja bieżącego stanu

Przeczytaj ten dokument i `docs/RECORDING-INVESTIGATION-0.0.2.1.md`, potem
sprawdź `git status`, `git log`, hashe zdalne, stan VM i jej karty. Potwierdź,
że oryginalny projekt nie jest przykryty bind mountem. Rozróżniaj **własne
zamierzone restarty** od `NRestarts` i wpisów systemd o awarii.

### 2. HOLD i skróty — zweryfikowana poprawka oraz otwarty LED

Na buildzie 0.0.2.1 włączono ARP/HOLD i zatrzaśnięto pady 1 i 9. Przed i po
SHIFT+PAD 16 maska `PREVIEW_STATE` wynosiła `196865`; pad 16 nie dołączył do
HOLD, a poziom audio w obu pomiarach wynosił około 0,45. Ekran Project został
sprawdzony wizualnie. Zielone LED padów 1 i 9 pozostały jednak zapalone także
w Project (`[0,127,0]` w panelu), czego użytkownik nie chce. HOLD i audio mają
pozostać aktywne, po powrocie ma wrócić prawidłowe podświetlenie.

`in_16_levels` jest heurystyką i może być nieaktualne po nawigacji. SDK nie
udostępnia potwierdzonego predykatu widoczności dokładnego widoku 16 Levels;
`page::visible` dotyczy Sample i dodatkowo wymaga próbki. Najpierw ustal
wiarygodną ścieżkę wejścia/wyjścia, potem testuj LED, zachowanie HOLD i skróty.

### 3. Rozmazywanie krokodyla i odświeżanie ekranu

Użytkownik zgłosił, że szybko zapętlany sample i poruszająca się linia pozycji
sampla rozmazują animację krokodyla. W próbie 0.0.2.1 przy 1/32, na kopii,
miernik potwierdził audio (peak około 0,46); w kilku pojedynczych zrzutach
ekranu nie udało się jednoznacznie uchwycić rozmazania. To **nie obala**
zgłoszenia: próbkuj dłużej i różne próbki/fazy, porównuj stop/play oraz
rzeczywisty obszar brudny JUCE. `src/splash.rs` rysuje krokodyla nad przebiegiem,
a natywny playhead może wyzwalać częściowe odmalowania; przyczyna pozostaje
hipotezą. Wcześniejszy obraz wyglądał na poszarpany, ale nie był kontrolowaną
serią dowodową.

Poważne powiązane ryzyko: `draw::start_repaint_pump` wywołuje
`sites::repaint_component` z wątku `mod-repaint`, chociaż dokumentacja tej
funkcji wymaga żywego obiektu na jego wątku UI. Ostatni zapis malowanego
komponentu sam nie dowodzi jego czasu życia. Podczas dłuższej próby eksperymentu
zaobserwowano SIGSEGV w wątku `mod-repaint` (dmesg), lecz nie ma stack trace;
nie stwierdzono definitywnie przyczyny. W eksperymencie pompę wyłączono, co
może osłabiać odświeżanie nakładki; późniejszy SIGABRT nadal wystąpił.
Znajdź bezpieczne odświeżanie na wątku UI i sprawdź animację oraz LED.

### 4. Nagrywanie i synchronizacja — w toku

Porównanie na dwóch identycznych kopiach: 128 BPM, dwa takty, zapisane
`quantisation.pulses=240`, swing 0.5, strength 0.0; pady 1 i 9, po około
0,65 s pad 16. NOTE REPEAT zapisał 11 nut co 240 impulsów, długość 180.
ARP zapisał 10 nut co 240, pierwsze 7 długości 125, potem 53, 72 i 95.
ARP miał gate 50%, repeat w tej próbie 75%; krótsze nuty mogą wynikać ze
zwalniania, pętli lub kwantyzacji — nie ustalono przyczyny. Czasy przycisków
nie były sprzętowo zsynchronizowane; porównanie fazy nie jest rozstrzygnięte.

`src/transport_clock.rs` i `src/transport_playback.rs` to eksperymentalny
prototyp, uruchamiany wyłącznie z `transport-sync-experiment`. Odczytowe badanie
firmware znalazło kandydat pozycji transportu w silniku: engine +0x790 →
transport +0x130, z warunkowym przesunięciem engine +0x668; wrapper i funkcja
opisane są w raporcie. Bajt transport +0x1f0 odpowiadał STOP/PLAY przy zgodnym
adresie metody vtable. Granice pętli w obserwowanej klasie są przez
transport +0x258 → pola +0x58/+0xb0. To nie jest opublikowane API, a czas
życia, zachowanie przy zmianie projektu, seek, tempo, współbieżność i firmware
trzeba zweryfikować. Nie przenoś niezweryfikowanych offsetów do sprzętowej paczki.

Prototyp kolejkuje zmiany do granic kroków i zachowuje kursor; host testy
przechodzą. Pierwsza próba na VM zapisała 10 wygenerowanych nut, 10 Note Off
i 10 zdarzeń o długości 124–125. Wykryto błąd zawijania, gdy MPC najpierw
zwraca dokładnie `loop_end`; poprawiono `<` na `<=`, po czym odczyt przechodził
kilka pętli z jednym początkowym resetem. Dłuższa próba obejmowała wszystkie
8 wzorców i 193 pary On/Off, ale inna zakończyła się awarią; potrzebna jest
powtarzalna seria oraz ponowny zapis po zmianie granicy pętli. Uwaga: trace
ring ma 256 wpisów, jego `pulse` jest tylko 16-bitowym fragmentem; wcześniejsze
odczyty `g_arp_audio_last` i liczników osobno miały wyścig i mogły pomijać kroki.

### 5. Samoczynne awarie — najtrudniejsze i blokujące wydanie

Zanotowano wcześniejszy ABRT `double free or corruption (!prev)`, kolejny ABRT
`corrupted double-linked list` podczas nagrywania na małym tmpfs, SIGSEGV w
`mod-repaint` podczas eksperymentu oraz osobny ABRT `corrupted double-linked
list` po wyłączeniu tej pompy. Nie ma dowodu, że wszystkie mają jedną przyczynę,
ani że sam brak miejsca powoduje uszkodzenie sterty. Zachowaj PID, journal,
dmesg i jeśli możliwe zrzut pamięci na izolowanej kopii, potem porównaj wariant
stock, 0.0.2.1 i eksperyment przy identycznym obciążeniu. Nie oznaczaj builda
jako gotowego do testów użytkownika, dopóki awarie są nierozpoznane lub
odtwarzalne. Nie pomyl ponownego uruchomienia usługi przez test z awarią.

## Weryfikacja i następne kroki

1. Zachowaj stabilny `main` na tagu 0.0.2.1, pracuj na gałęzi badawczej.
2. Na początek napraw rozpoznawanie widoczności i LED lub zbierz kontrolowany
   materiał rozmazywania; bez ryzykownego dereferencjonowania starego komponentu.
3. Zidentyfikuj przyczynę awarii i ustanów pomiar stabilności przed wydaniem.
4. Sprawdź eksperymentalny zegar przy STOP/PLAY, zmianie tempa, seek, granicy
   pętli, 1/4–1/64 i triolach, zmianie projektu, kilku padach oraz HOLD.
   Uwaga: stale założone 960 PPQ i długość pętli wymagają potwierdzenia.
5. Powtórz porównanie z NOTE REPEAT z potwierdzonym stanem REC i pozycją,
   uwzględniając zdarzenia, wysokość, velocity, długość, fazę oraz rzeczywiste
   audio i odsłuch. Odtwórz zapis po restarcie; sprawdź overdub bez duplikacji.
6. Ostatnie testy hosta: `cargo test -q` — 45; `cargo test -q --features
   transport-sync-experiment` — 47. To nie zastępuje testów VM i sprzętu.

Diagnozy i dalsze szczegóły: `docs/RECORDING-INVESTIGATION-0.0.2.1.md`,
`docs/HANDOFF-AUDIO-0.0.2.0-TESTY.md`, `docs/AUDIO-EXPERIMENTS.md`,
`docs/NATIVE-DISPATCH-RESEARCH.md`, `docs/PLAYBACK-RESEARCH.md`. Tymczasowe
skrypty i logi pod `/private/tmp/arp_*` mogą już nie istnieć; nie są artefaktami
wydania.
