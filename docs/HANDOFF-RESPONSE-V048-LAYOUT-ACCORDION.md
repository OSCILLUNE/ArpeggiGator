# Odpowiedź na handoff v0.0.4.8 — panel i akordeon

## Wynik

- Commit `d44f131` (`feat(ui): redesign right panel layout and add accordion gator for acordator mode (v0.0.4.8)`), tag `v0.0.4.8`; baza `07e42fe`.
- Wydanie `d44f131` zostało włączone do lokalnej gałęzi `main` przez fast-forward. Tag `v0.0.4.8` wskazuje commit wydania; niniejsza odpowiedź znajduje się w kolejnym commicie dokumentacyjnym. Zmiany nie zostały wypchnięte do zdalnego repozytorium.
- Zmieniono wyłącznie `src/splash.rs`: wspólny panel pięciu silników ma parametry na y=72 i kartę RATE/HOLD w x=184..301, y=96..183. Pełne tło overlayu pozostaje bez zmian.
- Scena ACORDATOR nie rysuje konsoli DJ; krokodyl ma zieloną paletę i akordeon z guzikami basowymi, miechem oraz klawiaturą. Końce instrumentu i łapki przesuwają się wraz z miechem.
- `cargo test --offline`: 83 passed, 0 failed (testy hosta). Nie wykonano wizualnej walidacji ani testów odsłuchowych w emulatorze.

## Emulator i napotkane problemy

- Pierwotnie uruchomiono bibliotekę przez tymczasowy `LD_PRELOAD` w `/run`; po restarcie maszyny powróciła poprzednia wersja. To był błąd w pierwszym sposobie przekazania wersji do testów.
- Następnie zbudowano wersję z `emulator-audio`, zarchiwizowano poprzedni moduł jako `/media/MPCSAMPLE/autorun/arpeggigator-before-v048.so` i zaktualizowano `/media/MPCSAMPLE/autorun/arpeggigator.so` za pomocą `scripts/sync-emulator-autorun.py --audio-lab`.
- SHA-256 nowej biblioteki na karcie i w `/run/arpeggigator.so`: `9149e393d4d5c60cc9d7cd9be388309927a7c05d689231c0019f4fbc17088c8e`. Stara kopia: `528d6244a521f3fdda4fc9f5571c204abc1209b64cfcfdd84828a72d2005c58e`.
- Uruchomiono skrypt autorun z karty, który skopiował bibliotekę do `/run` i zrestartował aplikację; `ac50` był active, NRestarts=0, a biblioteka była zmapowana w procesie. Pełnego restartu VM po tej aktualizacji nie przeprowadzono, więc pełny cykl startowy nadal wymaga potwierdzenia.
- Kompilacja początkowo nie powiodła się z powodu odmowy zapisu do domyślnego cache Zig; użyto `ZIG_GLOBAL_CACHE_DIR=/private/tmp/arpeggigator-zig-cache`. Skrypt `scripts/build.sh` automatycznie synchronizuje standardowy build i może zastąpić build audio na karcie; po jego użyciu trzeba ponownie świadomie zsynchronizować `--audio-lab`.

## Zalecane testy i słabe punkty

1. W emulatorze sprawdzić wszystkie pięć trybów: czy parametry, 22-pt Rate, instrukcja i status HOLD nie nakładają się ani nie są przycinane; przełączać wartości B2/B3, Rate i HOLD, również dla długich etykiet.
2. W ACORDATOR sprawdzić czytelność instrumentu i łapek przy wszystkich fazach animacji, w bezruchu oraz przy grze; potwierdzić brak pozostałości konsoli, migotania i powidoków. Sprawdzić ekran po nawigacji poza 16 Levels i po powrocie.
3. Miech jest animowany stałą sekwencją opartą na `now_ms / 100`, a nie zegarem BPM czy rzeczywistymi uderzeniami; `active` zależy od `held_pads > 0`. Zweryfikować zachowanie przy HOLD i zdecydować, czy wymagana jest synchronizacja z rzeczywistym rytmem i odtwarzaniem po puszczeniu padów.
4. Wykonać pełny restart VM, potwierdzić identyczny hash biblioteki w `/run` i na karcie oraz działanie UI i dźwięku. Sprawdzić włączanie trybów, odtwarzanie, zmianę Rate, HOLD, STOP oraz stabilność; rejestrować restart zaplanowany oddzielnie od awarii.
5. Testy hosta nie sprawdzają renderingu ani odsłuchu. Build audio jest przeznaczony do emulatora; powyższe wyniki nie potwierdzają zgodności ze sprzętem. Podczas testów zapisu używać oddzielnej kopii projektu.
