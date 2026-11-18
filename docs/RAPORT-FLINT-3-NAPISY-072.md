# RAPORT FLINT-3 — napisy kosmetyczne v0.0.7.2

**Data:** 2026-11-18 ~22:10Z
**Agent:** FLINT-3 (zespół Jeremiasza)
**Repo:** `/Users/fafi/Downloads/ArpeggiGator`, branch `release/0.0.7.2`
**HEAD przed zmianą:** `542b33fa88f83a74fd4a79478c3f118b106657f4`
**Zakres:** WYŁĄCZNIE `src/splash.rs`. Bez commitów, bez pushy, bez dotykania maszyn/karty/MPC.
**Zgoda fafi:** 3 zmiany — `ACORDATOR → ACCORDATOR`, `MODE: ACORD → MODE: ACCORD`, podwójny freeze w MUTATORze → pojedynczy.

## TRZY ZDANIA-KANON

1. TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.
2. CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (MPC_MOD_FEATURES=emulator-audio).
3. MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.

## 1. Co zmieniono — lokacje i numery linii (po zmianie)

| # | Zmiana | Stara linia | Nowa linia |
|---|--------|-------------|------------|
| 1 | Stałe `MODE_ACCORD`, `ACCORDATOR_TITLE` | — | `src/splash.rs:22-23` |
| 2 | Funkcja testowalna `mutator_param_line` | — | `src/splash.rs:182-189` |
| 3 | `motif_label` → `_motif_label` (bez warningu, bez zmiany wywołania) | 180 | 195 |
| 4 | `format!("{intensity_label}  {motif_label}  FRZ:…")` → `mutator_param_line(intensity_label, freeze)` | 194-197 | 209 |
| 5 | `"MODE: ACORD"` → `MODE_ACCORD` | 271 | 283 |
| 6 | `text("ACORDATOR", 13, 8, 88, 19, 12.0)` → `text(ACCORDATOR_TITLE, 13, 8, 88, 19, 11.0)` | 1370 | 1384 |
| 7 | `#[cfg(test)] mod tests` — 3 testy | — | `src/splash.rs:1644-1664` |

Plik ma po zmianie 1664 linie. `git diff --stat`: `1 file changed, 42 insertions(+), 7 deletions(-)`.

## 2. Dokładny diff (`git diff --no-color -- src/splash.rs`)

```diff
diff --git a/src/splash.rs b/src/splash.rs
index 9e94448..0c2922d 100644
--- a/src/splash.rs
+++ b/src/splash.rs
@@ -17,6 +17,11 @@ const SHADOW: u32 = 0xff075c39;
 const MID: u32 = 0xff18a852;
 const HIGHLIGHT: u32 = 0xffa8ed67;
 
+// Exact captions for the 0.0.7.2 spelling fixes; kept as constants so the
+// unit tests can assert the literals rather than a re-typed copy.
+const MODE_ACCORD: &str = "MODE: ACCORD";
+const ACCORDATOR_TITLE: &str = "ACCORDATOR";
+
 /// Paint custom B1/B2/B3 captions while SHIFT is held and active overlay for
 /// ARP or MELODY engine modes.
 ///
@@ -174,10 +179,20 @@ unsafe fn draw_melody_view(
     }
 }
 
+/// Build the MUTATOR B2 caption. The intensity label already carries the
+/// freeze motif (e.g. "FRZ OFF"), so appending `FRZ:*` again would print it
+/// twice. Kept separate from rendering so the exact literal is testable.
+fn mutator_param_line(intensity_label: &str, freeze: bool) -> String {
+    format!(
+        "{intensity_label} FRZ:{}",
+        if freeze { "ON" } else { "OFF" }
+    )
+}
+
 unsafe fn draw_mutator_view(
     graphics: &mut Graphics,
     intensity_label: &str,
-    motif_label: &str,
+    _motif_label: &str,
     held_pads: u8,
     hold: bool,
     rate: &str,
@@ -191,10 +206,7 @@ unsafe fn draw_mutator_view(
 
         // Right side info panel. The native firmware repaints the B3 softkey
         // caption ("Filter"), so surface freeze here where it stays visible.
-        let param_str = format!(
-            "{intensity_label}  {motif_label}  FRZ:{}",
-            if freeze { "ON" } else { "OFF" }
-        );
+        let param_str = mutator_param_line(intensity_label, freeze);
         draw_engine_right_panel(
             graphics,
             "MODE: MUTATOR",
@@ -268,7 +280,7 @@ unsafe fn draw_acordator_view(
         let param_str = format!("{mood_label}  {style_label}");
         draw_engine_right_panel(
             graphics,
-            "MODE: ACORD",
+            MODE_ACCORD,
             &param_str,
             hold,
             rate,
@@ -1367,7 +1379,9 @@ unsafe fn draw_shift_labels(
             }
             EngineMode::Acordator => {
                 graphics.colour(INK);
-                graphics.text("ACORDATOR", 13, 8, 88, 19, 12.0);
+                // One character wider than the old "ACORDATOR"; size trimmed
+                // 12.0 -> 11.0 so it still fits the same 88px B1 cell.
+                graphics.text(ACCORDATOR_TITLE, 13, 8, 88, 19, 11.0);
             }
             EngineMode::Orbitator => {
                 graphics.colour(0xffffffff);
@@ -1627,3 +1641,24 @@ unsafe fn draw_note(graphics: &mut Graphics, x: i32, chewing: bool) {
         }
     }
 }
+
+#[cfg(test)]
+mod tests {
+    use super::{ACCORDATOR_TITLE, MODE_ACCORD, mutator_param_line};
+
+    #[test]
+    fn mutator_freeze_off_is_not_duplicated() {
+        assert_eq!(mutator_param_line("SUBTLE", false), "SUBTLE FRZ:OFF");
+    }
+
+    #[test]
+    fn mutator_freeze_on_keeps_the_single_suffix() {
+        assert_eq!(mutator_param_line("WILD", true), "WILD FRZ:ON");
+    }
+
+    #[test]
+    fn accord_captions_have_the_correct_spelling() {
+        assert_eq!(MODE_ACCORD, "MODE: ACCORD");
+        assert_eq!(ACCORDATOR_TITLE, "ACCORDATOR");
+    }
+}
```

## 3. Wynik `cargo test --offline`

```
running 87 tests
...
test splash::tests::accord_captions_have_the_correct_spelling ... ok
test splash::tests::mutator_freeze_off_is_not_duplicated ... ok
test splash::tests::mutator_freeze_on_keeps_the_single_suffix ... ok
...
test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests arpeggigator
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**87/87 PASS** = 84 bazowe (82 + 2 regresje pierwszego pada z 7.1) + 3 nowe testy. Zgodnie z oczekiwaniem.

Przed zmianą ten sam komendant dawał `84 passed; 0 failed`.

Ostrzeżenia kompilatora: 29 warningów, **wszystkie pre-existing** (`unused import: page` w `src/lib.rs`, `dead_code` w `src/splash.rs` i `src/splash_art.rs`). Żaden nie pochodzi z nowego kodu — `_motif_label` jest celowo prefiksowany, stąd brak `unused_variables`.

`rustfmt --check --edition 2024 src/splash.rs`: nowe linie są zgodne z rustfmt. Pozostałe różnice w raporcie rustfmt to wyłącznie wcześniejsze, niezmienione fragmenty pliku (importy, długie linie 53/54/57/102 itd.) — nie zostały dotknięte.

## 4. Uwaga o szerokości napisu ACCORDATOR

`graphics.text()` w `vendor/mpc-sample-sdk/src/draw.rs:98-147` przekazuje tekst do firmware'u w `TextBox { x, y, width, height, flags: 0x24, align: 1, opacity: 0.7 }`. Szerokość pola jest twarda — tekst może zostać obcięty przez firmware, jeśli się nie zmieści.

Analiza porównawcza tego samego wiersza etykiet (`draw_shift_labels`, `src/splash.rs`):

| Etykieta | Znaki | x | Pole | Rozmiar | Znaków na px |
|----------|-------|---|------|---------|--------------|
| ARP | 3 | 17 | 80 | 15.0 | 80/3 = 26.7 |
| MELODY | 6 | 17 | 80 | 14.0 | 80/6 = 13.3 |
| MUTATOR | 7 | 15 | 84 | 13.0 | 84/7 = 12.0 |
| ORBITATOR | 9 | 14 | 86 | 12.0 | 86/9 = 9.6 |
| ACORDATOR (stare) | 9 | 13 | 88 | 12.0 | 88/9 = 9.8 |
| ACCORDATOR (gdyby 12.0) | 10 | 13 | 88 | 12.0 | 88/10 = 8.8 |
| **ACCORDATOR (użyte 11.0)** | **10** | **13** | **88** | **11.0** | **88/10 = 8.8, ale przy mniejszej czcionce** |

Wniosek: ORBITATOR (9 znaków przy 12.0 w polu 86 px) jest już ciasno dopasowany. Dodanie dziesiątego znaku przy tym samym rozmiarze 12.0 wymagałoby ~11% więcej miejsca w polu, które powiększyło się tylko o ~2% (86 → 88 px). Ryzyko obcięcia było realne.

Zastosowano dozwoloną korektę: **rozmiar czcionki 12.0 → 11.0** (o ~8,3%). Pozycja `(13, 8)`, pole `88×19`, kolor `INK` i ramka komórki (`label_cell(graphics, 13, 5, 88, ...)`) pozostały bez zmian. Kolejna komórka zaczyna się dopiero przy `x=111`, więc poszerzenie pola nie było bezpieczne bez ryzyka kolizji.

Nie wykonano screenu w emulatorze (brak dostępu do maszyny; zgodnie z ograniczeniami zadania nie dotykano maszyny/karty/MPC). Weryfikacja jest więc statyczna, oparta na kodzie i porównaniu z sąsiednimi etykietami. Jeśli Czesiek zrobi screen, wystarczy potwierdzić, że ACCORDATOR w B1 nie jest ucięty.

## 5. Co zostało nietknięte (świadomie)

- Wywołanie `draw_mutator_view(...)` w `src/splash.rs:56` (`draw_arp_overlay`) — bez zmian; parametr `_motif_label` przyjmuje ten sam `b3_label` co wcześniej (stąd wariant z prefiksem `_` był bezpieczniejszy niż usuwanie parametru z sygnatury).
- `MODE: MUTATOR`, `MODE: MELODY`, `MODE: ARP`, `MODE: ORBITATOR` — bez zmian.
- Pozostałe napisy, kolory, pozycje, logika audio, MIDI, SD, sample — bez zmian.
- Żadne inne pliki `src/` nie zmodyfikowano. `git status` dla `src/` pokazuje tylko ` M src/splash.rs`.
- Nie wykonano commit/push. Nie zbudowano paczki SD. Nie dotknięto fizycznej MPC.
