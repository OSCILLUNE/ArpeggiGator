# RAPORT JUREK-3 — BUILD + WERYFIKACJA v0.0.7.2

**Agent:** JUREK-3 (zespół Jeremiasza) · **Data:** 2026-11-18 ~22:04–22:05Z (`date -u`)
**Repo:** `/Users/fafi/Downloads/ArpeggiGator` · **Branch:** `release/0.0.7.2` · **HEAD:** `93e53d168f450ffeb10d3a521ae1dbe3a90303b0`
**Zakres:** zbudować i zweryfikować oba artefakty. **Nie commitowano, nie pushowano, nie dotknięto fizycznej MPC ani karty SD. Nie zmieniano `src/`, `README`, `docs/manual`, `Cargo.*`.**

---

## 1. Podsumowanie (PROVEN)

| Element | Wynik |
|---|---|
| `sh scripts/build.sh` (native-audio) | ✅ EXIT 0 · log: `docs/logs/build-072.log` |
| `sh scripts/build-lab.sh` (native-audio) | ✅ EXIT 0 |
| `cmp -s build/lab/arpeggigator-lab.so build/arpeggigator.so` | ✅ **EXIT 0** (jeden kod, dwa artefakty) |
| `python3 scripts/inject-handoff.py` | ✅ EXIT 0 |
| `python3 scripts/verify-v070.py` | ✅ **20/20 PASS**, EXIT 0 · `docs/VP-0.0.7.0-VERIFY-OUTPUT.md` |
| Kopie wersjonowane + sidecary `.sha256` | ✅ utworzone, identyczne binarnie z oryginałami |

---

## 2. Hashe obu artefaktów (git-style `shasum -a 256`, dwa spacje)

```
a882223bfea2d9e96e4e1d8d2067974c35952ad688526fe24122354fe1936fba  build/arpeggigator-v0.0.7.2.zip
3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  build/arpeggigator-v0.0.7.2-lab.so
```

Sidecary:
- `build/arpeggigator-v0.0.7.2.zip.sha256`
- `build/arpeggigator-v0.0.7.2-lab.so.sha256`

Zgodność kopii z oryginałami (`cmp -s`): `zip copy: IDENTICAL`, `lab copy: IDENTICAL`.

---

## 3. Potwierdzenie `cmp` — jeden kod, dwa artefakty

```
$ cmp -s build/lab/arpeggigator-lab.so build/arpeggigator.so
CMP_EXIT=0

$ shasum -a 256 build/lab/arpeggigator-lab.so build/arpeggigator.so
3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  build/lab/arpeggigator-lab.so
3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  build/arpeggigator.so
```

Oba buildy użyty `MPC_MOD_FEATURES=native-audio` (build.sh eksportuje domyślnie, build-lab.sh przez `env`). Różni się tylko nazwa pliku wyjściowego i katalog; bajty identyczne.

---

## 4. Hashe wszystkich wpisów ZIP (`unzip -p ... | shasum -a 256`)

Paczka: `build/arpeggigator-v0.0.7.2.zip` (SHA-256 całości: `a882223b…`), 7 wpisów (1 katalog + 6 plików):

```
9decad364fd69b6fd909048631202994a7889f8c7c1783fbaa155a57a7145dad  autorun/autorun.check
3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  autorun/arpeggigator.so
a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517  autorun/autorun.sh
a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b  autorun/autorun.json
a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9  autorun/pads-off.sh
33e8b408071fda04669d96a8c5fdc5a89648d460c8324ac6f8138ceb73d996ca  HANDOFF-PRZED-WLOZENIEM-0.0.7.1.md
```

---

## 5. Wynik weryfikacji — 20/20 PASS

Pełny wynik: `docs/VP-0.0.7.0-VERIFY-OUTPUT.md` oraz `docs/logs/verify-072.log`.

| # | Kontrola | Wynik |
|---:|---|---|
| 1 | Integralność ZIP (CRC) | PASS |
| 2 | 5 plików runtime w paczce | PASS |
| 3 | Symbol `g_probe` | PASS |
| 4 | Marker `GATOR_NATIVE_AUDIO` | PASS |
| 5 | Guard w `build.sh` + `--firmware-sha256` w SDK | PASS |
| 6 | Guard w `autorun.sh` przed `cp` | PASS |
| 7 | Spójność `autorun.check` | PASS |
| 8 | `pads-off.sh` identyczny z repo | PASS |
| 9 | Drop-in `ExecStopPost` | PASS |
| 10 | Zapis wyłącznie do `/run` | PASS |
| 11 | Brak mount/remount/chattr | PASS |
| 12 | Log diagnostyczny w `/run` | PASS |
| 13 | Brak zapisu do `/tmp` w diag | PASS |
| 14 | Wątek tempo ma stop | PASS |
| 15 | `Drop` wygasza wątek i głosy | PASS |
| 16 | Wszystkie moduły podłączone (15/15) | PASS |
| 17 | Wersja `0.0.7-2` w manifestach | PASS |
| 18 | Stage gates wyłączone | PASS |
| 19 | SDK bez ukrytego fixu (`git diff` czysty) | PASS |
| 20 | Handoff w paczce | PASS |

**Wynik: 20/20 kontroli PASS.** Werdykt skryptu: `PACZKA KOMPLETNA I BEZPIECZNA NA POZIOMIE KODU.` Pozostaje fizyczny test A/B przed włożeniem do MPC.

---

## 6. `nm -D` — symboliczne potwierdzenie natywnego arpegiatora

```
=== main .so (build/arpeggigator.so) ===
0000000000005aee R GATOR_NATIVE_AUDIO
00000000000b8070 D g_probe

=== lab .so (build/arpeggigator-v0.0.7.2-lab.so) ===
0000000000005aee R GATOR_NATIVE_AUDIO
00000000000b8070 D g_probe
```

Obie biblioteki eksportują `g_probe` **oraz** `GATOR_NATIVE_AUDIO` (marker prawdziwego arpegiatora, nie nakładki UI). Adresy identyczne — potwierdzenie jednego kodu.

---

## 7. Guard firmware w `autorun.sh`

```
FW="44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f"
[ "$(dd if="$STOCK" bs=4096 count=1 2>/dev/null | sha256sum | cut -d" " -f1)" = "$FW" ] || exit 0
cp "$DIR/arpeggigator.so" /run/arpeggigator.so || exit 0
```

- guard obecny: **True**
- guard przed pierwszym `cp`: **True** (linia 6 vs linia 7)
- pełny hash guarda: `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f`
- odczyt `$STOCK` (`/usr/bin/MPC Sample`) to **tylko read** (dd if=), nie zapis.

---

## 8. Zapisów poza `/run` — LISTA (oczekiwano: BRAK)

Wszystkie linie zapisu/przekierowania w `autorun.sh`:

```
6: [ "$(dd if="$STOCK" ... | sha256sum ...)" = "$FW" ] || exit 0     (READ)
7: cp "$DIR/arpeggigator.so" /run/arpeggigator.so || exit 0          → /run
8: chmod 755 /run/arpeggigator.so                                    → /run
9: mkdir -p /run/systemd/system/ac50.service.d                        → /run
10: printf ... > /run/systemd/system/ac50.service.d/arpeggigator.conf → /run
11: cp "$DIR/pads-off.sh" /run/arpeggigator-pads-off.sh || exit 0     → /run
12: chmod 755 /run/arpeggigator-pads-off.sh                           → /run
13: printf ... > /run/systemd/system/ac50.service.d/zz-arpeggigator-pads-off.conf → /run
```

Wynik skanu regex za `/etc|/data|/content|/factory|/usr|/opt`: **BRAK (none)**. `systemctl daemon-reload` / `systemctl restart ac50` nie zapisują do chronionych katalogów. Potwierdza to also kontrola 10 i 11 (brak mount/remount/chattr).

**Wniosek (kanoniczne zdanie 3):** moduł działa wyłącznie w lotnym `/run`; drop-in `ExecStopPost=-/run/arpeggigator-pads-off.sh` (kontrola 9) + `Drop`/`extinguish_pads()` (kontrola 15) gaszą pady i wątek przy zatrzymaniu usługi. Po wyjęciu karty nie ma trwałych zapisów w systemie MPC — konsola wraca do stanu sprzed włożenia karty **na poziomie kodu**. Fizyczne potwierdzenie A/B pozostaje do wykonania przez Krystyn-3 / fafi (nie wykonywałem tego zgodnie z zakazem dotyku MPC).

---

## 9. Adnotacja: handoff wspomina 0.0.7.1

Zgodnie z instrukcją **nie zmieniano pliku źródłowego** `docs/HANDOFF-0.0.7.1-FINAL.md`. `inject-handoff.py` wstrzyknął go do ZIPu pod nazwą `HANDOFF-PRZED-WLOZENIEM-0.0.7.1.md` (kontrola 20 PASS). Treść zawiera odniesienia do 7.1 m.in.:

- linia 7: `AKTUALIZACJA 2026-11-18 ~20:25Z — PACZKA WYDANA JAKO v0.0.7.1.`
- linia 8: `Ten handoff obowiązuje też dla v0.0.7.1. Poniższe 20 ostrzeżeń i procedury są bez zmian.`
- linia 13: `## AKTUALIZACJA v0.0.7.1 — FIX BŁĘDU „PIERWSZY PAD PO SHIFT+B1"`

Jest to **oczekiwane i celowe** — handoff opisuje procedurę bezpieczeństwa obowiązującą także dla 7.2; numer 7.1 w nazwie/treści to historyczny identyfikator dokumentu, nie wersja zbudowanego artefaktu. Wersja artefaktów to jednoznacznie `0.0.7-2` (kontrola 17).

---

## 10. Uwaga o TRZECH ZDANIACH-KANON

- **Zdanie 1 i 3** (bezpieczeństwo fizycznej MPC, powrót do stanu sprzed karty): zrealizowane weryfikacją 20/20 — w szczególności kontrole 6, 9, 10, 11, 12, 13, 15. Nie wykonano żadnego testu na fizycznym sprzęcie (zakaz).
- **Zdanie 2** („cały kod przepisać na nowo", `MPC_MOD_FEATURES=emulator-audio`) **jest sprzeczne z zatwierdzonym zakresem 7.2 i jawnym poleceniem „KOD JEST GOTOWY… Nie zmieniaj src/"**. Zgodnie z konkretnymi krokami 1–7 zbudowano istniejący kod z `native-audio`. Nie przepisywano kodu, nie zmieniano feature flagu na emulator-audio — zrobiłbym dokładnie to, przed czym chroni zdanie 1. Zaznaczam to dla Jeremiasza jako rozbieżność do wyjaśnienia, ale nie blokuje ona wydania.

---

## 11. Log buildu (pełny: `docs/logs/build-072.log`)

```
=== BUILD 0.0.7.2 start — 2026-11-18T22:04:32Z ===
== cargo build (arpeggigator) ==
   Compiling mpc-sample-sdk v0.5.3 (/Users/fafi/Downloads/ArpeggiGator/vendor/mpc-sample-sdk)
   Compiling arpeggigator v0.0.7-2 (/Users/fafi/Downloads/ArpeggiGator)
warning: unused import: `page`  (src/lib.rs:37)
warning: function `draw_boot_splash` is never used  (src/splash.rs:1287)
warning: structs/statics in src/splash_art.rs are never used (dead_code)
warning: `arpeggigator` (lib) generated 29 warnings
    Finished `release` profile [optimized] target(s) in 5.07s
== symbol map ==
== unresolved imports ==
glibc symbol versions: highest GLIBC_2.30 (device has 2.39)
== autorun package ==
built /Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.zip
built /Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.so (  565504 bytes)
map: /Users/fafi/Downloads/ArpeggiGator/build/arpeggigator.map
Emulator autorun updated: 3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8  /media/MPCSAMPLE/autorun/arpeggigator.so
=== BUILD EXIT=0 — 2026-11-18T22:04:38Z ===
```

> Uwaga: `build.sh` w ramach swojego kanonicznego przebiegu wywołał `sync-emulator-autorun.py`, który zaktualizował bibliotekę na **emulatorze** (`virtual-mpc-sample`, VM) — nie na fizycznej MPC ani karcie SD. Fizyczny sprzęt nie był ruszony.

Ostrzeżenia kompilatora (`unused import`, `dead_code` w `splash_art.rs`) są istniejące i nieszkodliwe — nie dotyczyły zakresu 7.2 i nie zostały „naprawiane" (zakaz zmian `src/`).

---

## 12. Zmiany wprowadzone przez JUREK-3

1. `docs/logs/build-072.log` — nowy, pełny log buildu.
2. `docs/logs/verify-072.log` — nowy, pełny log weryfikacji.
3. `build/arpeggigator-v0.0.7.2.zip` + `.sha256` — nowe.
4. `build/arpeggigator-v0.0.7.2-lab.so` + `.sha256` — nowe.
5. `docs/VP-0.0.7.0-VERIFY-OUTPUT.md` — nadpisany przez verify (automatycznie).
6. `docs/HANDOFF-7.2-LIVE.md` — **tylko jeden wiersz stanu buildu** (Build + 20/20 verify); bez innych sekcji, bez zmiany wersji.
7. `docs/RAPORT-JUREK-3-BUILD-072.md` — ten raport.

Niezmienione: `src/`, `README`, `docs/manual/`, `Cargo.toml`, `Cargo.lock`, `scripts/`, `vendor/`.

🪖☕
