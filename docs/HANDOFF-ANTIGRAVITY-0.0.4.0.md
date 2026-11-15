# ArpeggiGator 0.0.4.0 — handoff dla Antigravity

Rozmawiaj z użytkownikiem po polsku. Użytkownik chce rozpocząć prace nad trzema nowymi trybami muzycznymi: **MUTATOR**, **ACORDATOR** i **ORBITATOR**. Celem jest działające, muzycznie inspirujące rozwinięcie istniejących ARP i MELODY, z zachowaniem dotychczasowych funkcji. Nazwy zapisuj dokładnie tak jak powyżej.

## Start i bezpieczeństwo pracy

- Przeczytaj ten plik oraz `docs/HANDOFF-RESPONSE-GEMINI-2026-11-15.md` (jeśli jest dostępny). Sprawdź faktyczny stan repozytorium, `git status`, `git log`, tagi i worktree; mógł się zmienić. W chwili sporządzania handoffu `main` wskazywał na `fe2c37a`, a tag `v0.0.3.4` na `7964577`. Główny katalog `/Users/fafi/Downloads/ArpeggiGator` był na starszej gałęzi `codex/transport-sync-research`; osobne worktree zawierało `main`. Ten handoff może być plikiem nieśledzonym w głównym katalogu, niewidocznym w innym worktree — przeczytaj go stamtąd.
- Pracuj na osobnej gałęzi od aktualnego `main`, np. `codex/v0.0.4.0-generative-modes`, w odpowiednim osobnym worktree. Nie przełączaj gałęzi w checkoutcie zajętym przez inny wątek. Zachowaj cudze zmiany i nieśledzone pliki; nie rób resetów, czyszczenia ani nadpisywania. Nie scalaj ani nie publikuj bez wyraźnego polecenia użytkownika.
- Zapoznaj się z istniejącym `src/lib.rs`, silnikami, schedulerem, obsługą audio, HOLD, nawigacją i sposobem budowania. `README` może być nieaktualny. Sprawdź, czy build emulatora i sprzętowy rzeczywiście mają te same funkcje: historycznie audio było dostępne tylko w opcjonalnym buildzie emulatora. Nie ogłaszaj wsparcia sprzętowego bez sprawdzenia.
- `scripts/build.sh` może automatycznie zmienić moduł na karcie emulatora. Przeczytaj skrypty przed uruchomieniem; zachowaj możliwość powrotu do wcześniejszego buildu. Nie używaj `virtual-mpc-sample-qa`. Nie nadpisuj oryginalnego projektu emulatora, szczególnie podczas nagrywania; do nagrywania użyj osobnej kopii dyskowej. Nie usuwaj istniejących projektów ani zasobów. Sprawdzaj faktyczny PID, restarty, wersję i hash; własny celowy restart nie jest awarią.

## Wspólna wizja

Trzy tryby powinny generować zdarzenia nutowe dla istniejącego instrumentu/sampla MPC, a nie wymagać osobnej syntezy audio. Inspiracja ORBIX dotyczy generatywnego charakteru, nie kopiowania projektu GrainBrick ani deklarowania syntezy granularnej. Wciśnięty pad określa nutę bazową; wykorzystaj istniejące tempo, ustawienia odtwarzania i HOLD tam, gdzie to ma sens. Ogranicz wysokość i velocity do poprawnych zakresów. Generowanie ma być muzyczne, nie wyłącznie przypadkowe.

Poniższe sterowanie jest propozycją, którą można dopasować do realnych ograniczeń interfejsu: zachowaj istniejące skróty ARP/MELODY i natywne zachowanie poza 16 Levels, przedstaw użytkownikowi czytelne nazwy i stany. Nie wymyślaj niezweryfikowanych kombinacji, które kolidują z MPC. Zachowaj zmiany ustawień przewidywalne i sprawdź wyłączanie wszystkich aktywnych nut przy zmianie trybu, STOP i przejściu między ekranami. Dotychczasowy kod może czyścić HOLD przy nawigacji; nie zakładaj odmiennego zachowania bez analizy i decyzji.

## 1. MUTATOR — ewoluująca melodia

Punkt wyjścia to istniejące frazy MELODY. Melodia jest rozpoznawalna, lecz co kilka taktów stopniowo zmienia pojedyncze elementy: wysokość w skali, pauzę, oktawę lub rytm. Ma być możliwe usłyszenie motywu i jego ewolucji, bez niekontrolowanego chaosu.

Proponowane poziomy: **SUBTLE** — rzadkie drobne zmiany; **WILD** — częstsze i bardziej wyraziste; **CHAOS** — szerokie, ale nadal ograniczone muzycznie. Zaproponuj jasne zasady zmian i ich interwał. Przydatne jest zamrażanie aktualnej wersji oraz powrót do oryginalnego motywu, jeśli uda się to obsłużyć bez konfliktów; jeśli nie, opisz ograniczenie. Nie zmieniaj na stałe oryginalnych danych MELODY. Zacznij od tego trybu jako pierwszego kamienia milowego.

## 2. ACORDATOR — akordy z jednego pada

Pad określa dźwięk bazowy; silnik buduje z niego akord, np. molowy, durowy lub bardziej przestrzenny, i odtwarza go równocześnie albo rozkłada w czasie. Użytkownik chce klimaty ciepłe, smutne, kosmiczne i filmowe. Zaproponuj niewielki, czytelny zestaw brzmieniowych wariantów i sposób przełączania; przejścia akordów co kilka taktów mogą być kolejnym wariantem, jeśli działają przewidywalnie.

To wymaga wielu współbrzmiących nut: najpierw sprawdź, czy istniejąca ścieżka MPC potrafi bezpiecznie wysłać i zakończyć wiele głosów. Nie zakładaj, że monofoniczny MELODY obsłuży akord bez zmian. Limituj polifonię, paruj Note On/Off i sprawdzaj zmianę pada, HOLD, stop, zmianę widoku oraz trybu. Jeżeli architektura nie pozwala bezpiecznie na pełny akord, przedstaw wynik i ograniczenie, zamiast ogłaszać funkcję gotową.

## 3. ORBITATOR — generatywna muzyczna podróż

Tryb inspirowany ideą ORBIX: pad uruchamia ewoluującą generację nut wokół nuty bazowej, a nie stałą gotową melodię. Używaj wybranej skali, kontrolowanych kroków, pauz, ograniczonego zakresu i powracających motywów. Ma tworzyć muzyczne, zmieniające się frazy.

Proponowane klimaty: **AMBIENT / SPACE / DARK / RHYTHM**; poziomy zmienności: **CALM / FLOW / WILD**. Dobierz praktyczną liczbę opcji do małego ekranu. HOLD pozwala kontynuować grę, a nowy pad zmienia bazę w przewidywalnym momencie; określ moment przełączenia i sposób kończenia starych nut. Użyj ograniczonego generatora losowego bez alokacji na ścieżce czasu rzeczywistego, z możliwością stałego ziarna w testach. Gra powinna móc różnić się między sesjami, ale testy muszą być powtarzalne.

Pomysł wizualny na przyszłość: krokodyl astronauta i orbitujące nuty. Grafika nie może blokować funkcjonalnego MVP ani pogorszyć stabilności; nie dodawaj ryzykownych wywołań repaint.

## Kolejność i kryteria ukończenia

1. Sprawdź stan bazowy oraz istniejące testy. Zaimplementuj i przetestuj MUTATOR; zachowaj czytelny etap prac.
2. Zbadaj możliwość polifonii i dodaj ACORDATOR, jeśli jest bezpieczna.
3. Dodaj ORBITATOR. Integruj stopniowo, weryfikując regresje ARP i MELODY po każdej zmianie.
4. Testuj deterministycznie logikę muzyczną: granice nut, skale, zmiany, pauzy, powtarzalność ziarna, parowanie nut, przełączenia trybów i HOLD. W emulatorze sprawdź rzeczywiste audio, sterowanie, nawigację i stabilność; jeżeli testujesz zapis, korzystaj z osobnej kopii projektu. Rozróżnij test hosta, emulatora i fizycznego urządzenia. Sprawdź również, czy istniejąca synchronizacja do transportu rzeczywiście działa — nie zakładaj jej naprawienia.
5. Zaktualizuj opis trybów, sterowania, ograniczeń i instrukcje budowania w dokumentacji. 0.0.4.0 to cel rozwoju; nie nazywaj go gotowym wydaniem i nie twórz taga ani publikacji przed ukończeniem i weryfikacją. Jeśli jakiś tryb nie jest bezpiecznie wykonalny, zachowaj działające etapy i jasno opisz przeszkodę.

Nie traktuj dawnych twierdzeń o „100% stabilności”, sprzętowej gotowości albo naprawionym nagrywaniu jako dowodu: oceniaj na podstawie konkretnych, powtarzalnych testów. Na koniec podaj gałąź, zmiany, jak słuchać i sterować każdym trybem, wykonane testy oraz to, co pozostało nieweryfikowane.
