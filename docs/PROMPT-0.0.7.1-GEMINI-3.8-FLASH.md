# PROMPT DLA GEMINI 3.8 FLASH — weryfikacja paczki v0.0.7.1 PRZED włożeniem karty

Skopiuj cały blok poniżej jako pierwszą wiadomość do Gemini 3.8 Flash.

---

Przejmujesz wątek ArpeggiGator od Jeremiasza. Twoim zadaniem **nie jest** włożenie karty do MPC. Twoim zadaniem jest **zweryfikować paczkę v0.0.7.1 i wydać werdykt GO / NO-GO**, zanim fafi włoży kartę do fizycznej MPC Sample.

## NATYCHMIAST PRZECZYTAJ CAŁE (bez skrótów):

1. `/Users/fafi/Downloads/ArpeggiGator/docs/HANDOFF-0.0.7.1-FINAL.md`
2. `/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-VERIFY-OUTPUT.md`
3. `/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-KRYSTYNA-BEZPIECZENSTWO.md`
4. `/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-FLINT-KOD-LINIA-PO-LINII.md`
5. `/Users/fafi/Downloads/ArpeggiGator/docs/VP-0.0.7.0-JUREK-INTEGRALNOSC.md`
6. Plik `HANDOFF-PRZED-WLOZENIEM-0.0.7.1.md` wewnątrz `build/arpeggigator-v0.0.7.1.zip`.
7. `/Users/fafi/Downloads/ArpeggiGator/docs/RELEASE-0.0.7.1.md` i raport A/B `/Users/fafi/Downloads/ArpeggiGator-0.0.6.5/docs/RAPORT-FINAL-0.0.6.5.md`.

To nie jest opcjonalne. Bez lektury nie wydawaj werdyktu.

## TRZY ZDANIA-KANON (powtarzaj przy każdej decyzji):

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.**
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`). Nie zakładać, że build emulatora jest buildem dla fizycznego sprzętu.**
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.**

## 20 OSTRZEŻEŃ

W handoffie (`docs/HANDOFF-0.0.7.1-FINAL.md`) są dokładnie 20 ponumerowanych ostrzeżeń. Odczytaj je i odhacz **każde z osobna**. Nie wolno ich skrócić do „wszystko sprawdzone".

## STAN ZASTANY (zweryfikowany 2026-11-18 ~17:55Z)

- Repo: HEAD bazowy `b912eed…` (czysty `v0.0.6.0`), wersja `0.0.7-1`.
- `cargo test`: **84/84 PASS** (82 + 2 testy regresji błędu pierwszego PAD). Build wykonany, paczka w `build/arpeggigator-v0.0.7.1.zip`.
- Fix błędu „pierwszy PAD po SHIFT+B1 w Mutator/Acordator: animacja tak, muzyka nie" — A/B QA: baseline `0/20` prób z nutami, po fixie `19/19` ważnych prób.
- Feature `emulator-audio` przemianowana na `native-audio` — bez niej mod jest tylko nakładką UI.
- Logi przeniesione z `/tmp` do `/run`; wątek tempo ma `stop_monitor()`; `Drop` gasi stan.
- `scripts/verify-v070.py` wykonuje **20 jawnych kontroli** — wynik w `docs/VP-0.0.7.0-VERIFY-OUTPUT.md`.
- **Backup fizycznej MPC NIE został wykonany** (fafi zrezygnował). Nie ma obrazu `/dev/mmcblk0` ani rollbacku `dd`.
- **QA ma czysty system/firmware/usługi, ale `/content` jest puste (0 plików).** Nie nazywaj jej „jak ze sklepu".
- Fizyczna MPC była pingowalna (`192.168.155.1`), ale root-SSH zamknięty. **Nie dotykaj jej.**

## TWOJA PRACA — KOLEJNOŚĆ

### FAZA 1 — weryfikacja paczki (bez dotykania maszyn)
```
cd /Users/fafi/Downloads/ArpeggiGator
date -u
git rev-parse HEAD
git status --short
shasum -a 256 build/arpeggigator-v0.0.7.1.zip build/arpeggigator.so
unzip -l build/arpeggigator-v0.0.7.1.zip
unzip -t build/arpeggigator-v0.0.7.1.zip
python3 scripts/verify-v070.py
cat docs/VP-0.0.7.0-VERIFY-OUTPUT.md
unzip -p build/arpeggigator-v0.0.7.1.zip autorun/autorun.sh
nm -D build/arpeggigator.so | grep -E 'g_probe|GATOR_NATIVE_AUDIO'
```
Oczekiwane: 20/20 PASS, `g_probe` i `GATOR_NATIVE_AUDIO` obecne, guard `44507c7a…` przed pierwszym `cp`, wszystkie zapisy w `autorun.sh` wyłącznie do `/run`, `autorun.check` spójny.

### FAZA 2 — weryfikacja kodu linia po linii (bez zmian)
Przeczytaj samodzielnie `autorun.sh` z paczki, `src/lib.rs` (cfg gates, `Drop`), `src/diag.rs`, `src/tempo.rs`, `scripts/build.sh`, `scripts/package-pad-cleanup.py`, `scripts/pads-off.sh`. Potwierdź lub odrzuć twierdzenia z raportów agentów. Każde twierdzenie: PROVEN (komenda) albo INFERRED.

### FAZA 3 — screen w momencie naciśnięcia przycisku (QA, nie fizyczna MPC)
```
python3 scripts/ag6_probe.py qa WERYFIKACJA-070 \
  '{"enter16":true,"mode":3,"hold":["SHIFT","SCREEN 1"]}'
```
Wymagane: `06_screen1200_PRESSED.png` i `09_device_PRESSED.png` zrobione **przed** `release`, `NRestarts=0`, `ABRT=0`, screen przed ≠ screen wciśnięty. Porty: QA panel `8766`, SSH `2032`, VNC `5920`; klucz `~/Downloads/virtual-mpc-sample-qa/out/keys/id_ed25519`.

### FAZA 4 — werdykt
Wypisz:
1. Odhaczone 20/20 ostrzeżeń (które PROVEN, które INFERRED).
2. Pełną listę plików paczki z hashami.
3. Dowód, że autorun nie zapisuje poza `/run`.
4. Jasne rozróżnienie: „po restarcie jak przed" (PROVEN z kodu) vs „w chwili wyjęcia karty" (NIE — mod zostaje w pamięci).
5. **GO albo NO-GO dla fizycznej MPC.**

## ZAKAZY (twarde)

- Nie wkładaj karty do fizycznej MPC. Nie deployuj. Nie rebootuj fizycznej maszyny.
- Nie wykonuj R2/R3 z planu backupu (to zapisy: `rm`, `dd of=`, `systemctl restart`).
- Nie pushuj, nie resetuj OpenCode.
- Nie używaj `build/arpeggigator-qa.zip` ani starego `build/arpeggigator.zip` sprzed rebuildu.
- Nie uznawaj heartbeat/OCR za dowód bez odczytu z systemu.
- Jeśli czegokolwiek nie umiesz udowodnić → napisz wprost: **„NIE NADAJE SIĘ DO DEPLOYU"**.

Przejmujesz wątek. Nie pocieszaj — diagnozuj. 🪖☕
