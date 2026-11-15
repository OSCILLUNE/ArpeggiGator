# ArpeggiGator — Handoff v0.0.4.8: Redesign Prawego Panelu i Krokodyl z Akordeonem (ACORDATOR)

**Zalecany model:** **GPT 6.1-Terra** (tani w tokenach, precyzyjny w pixel-arcie i układzie współrzędnych 2D)  
**Zalecany poziom wysiłku:** **Medium** (Średni)  
**Główny plik do edycji:** [`src/splash.rs`](file:///Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/gemini_shift_handoff/src/splash.rs)  
**Stan bazowy:** `main` na commit `07e42fe` (tag `v0.0.4.7`, 83/83 testy pass).

---

## 1. Cel Wydania v0.0.4.8

W tym wydaniu realizujemy dwa zadania graficzno-interfejsowe:
1. **Zadanie A:** Przebudowa i uporządkowanie prawego panelu kontrolnego (`draw_engine_right_panel`) we wszystkich 5 trybach silnika.
2. **Zadanie B:** Całkowity redesign sceny w trybie **ACORDATOR** – zastąpienie ciemnego DJ-a za deckami **krokodylem w standardowych kolorach, grającym na animowanym akordeonie**.

---

## 2. Zadanie A: Redesign Prawego Panelu Kontrolnego

Wszystkie 5 silników (`ARP`, `MELODY`, `MUTATOR`, `ACORDATOR`, `ORBITATOR`) korzysta ze wspólnej funkcji rysującej prawy panel:
`unsafe fn draw_engine_right_panel(...)` w `src/splash.rs` (linie ~64–113).

### Oczekiwany nowy układ pionowy (od góry do dołu):

#### Górny blok (Karta Trybu i Parametrów):
- **Linia 1 (Góra):** `MODE: [NAZWA]` – bez zmian (np. `MODE: ARP`, `MODE: MUTATOR`, `185, 54`, rozmiar fontu 13.0, kolor `mode_color`).
- **Linia 2:** Parametry silnika z `SHIFT + B2` i `SHIFT + B3` (`param_line`, np. `UP  +0`, `2 BARS  A`, `MID  POP`, `7TH  STRUM`, `AMBIENT  WILD`).
  - Przenieś tutaj wartości parametrów (np. `y = 72`, rozmiar 12.0–13.0, kolor `param_color`).
  - Z tego miejsca **usuwamy** dawny napis `HOLD: ON/OFF` (przenosimy go na sam dół).

#### Dolny blok (Karta RATE, Instrukcja Enkodera i Status HOLD):
- Karta z ramką: `rect(184, 96, 118, 88)` z tłem `card_bg` i obramowaniem `card_border`.
- **Nagłówek karty (góra karty):** `graphics.text("TURN DATA: RATE", 186, 100, 114, 12, 10.5)` (kolor `mode_color`).
- **Wartość tempa (środek-góra):** duży neonowy wskaźnik podziałki (np. `1/16` w kolorze `LIME` `0xffa6ff00`, czcionka 20.0–22.0, np. `y = 114`, wysokość 24).
- **Instrukcja wciśnięcia pokrętła (pomiędzy Rate a Hold):**
  - Zwięzły, czytelny napis instruktażowy:
    `graphics.text("PUSH DATA: HOLD", 186, 142, 114, 12, 10.0);`
    *(kolor: stonowany jasnoszary/kremowy `0xffc0c7d0` lub `card_border` w jaśniejszym odcieniu).*
- **Status HOLD (na samym dole karty):**
  - Wskaźnik statusu zatrzasku:
    `graphics.colour(if hold { LIME } else { 0xffff4053 });`
    `graphics.text(if hold { "HOLD: ON" } else { "HOLD: OFF" }, 186, 158, 114, 16, 12.0);`

---

## 3. Zadanie B: Krokodyl Grający na Akordeonie (Tryb ACORDATOR)

Obecnie w trybie ACORDATOR funkcja `draw_acordator_stage` (linie ~290–315) rysuje:
- Filar harmoniczny,
- Konsolę DJ-ską `draw_dj_booth(...)`,
- Krokodyla o nienaturalnej, ciemnozielonej skórze (`draw_acordator_gator`, linie ~1010–1080).

### Wymagane zmiany:

1. **Usunięcie DJ Booth:**
   - W `draw_acordator_stage` usuń wywołanie `draw_dj_booth(graphics, active, now_ms)`.
2. **Krokodyl w naturalnych, żywych kolorach:**
   - Zastąp paletę `ACORD_DARK_*` standardowymi, klasycznymi kolorami ArpeggiGatora (takimi jak w trybie Melody):
     - `GREEN` (`0xff28a745`),
     - `MINT` (`0xff34d399`),
     - `INK` (`0xff0d1117`),
     - `CREAM` (`0xfff0fdf4`),
     - `GOLD` (`0xffffc107`),
     - Jasny brzuch: `0xff8bc34a` z prążkami `0xff558b2f`.
3. **Akordeon w łapach krokodyla:**
   - Narysuj akordeon trzymany przez aligatora na wysokości klatki piersiowej / brzucha:
     - **Prawa strona (klawiatura melodyczna):** prostokątna obudowa akordeonu (np. bordowa/czerwona `0xffb71c1c` lub perłowa) z białymi i czarnymi klawiszami pionowymi (`CREAM` i `INK`).
     - **Lewa strona (basy):** mniejsza obudowa z siatką guzików basowych (małe kropki `CREAM` / `GOLD`).
     - **Miech (Bellows) w środku:** harmonijkowe plisy (paski pionowe naprzemiennie ciemne i jasne).
4. **Animacja miecha (Bellows Pump) w rytm gry:**
   - Gdy `active` jest `true` (odtwarzane są akordy lub wciśnięty jest pad):
     - Niech miech akordeonu rytmicznie rozszerza się i zwęża!
     - Przykładowa formuła:
       `let bellows_spread = if active { [0, 2, 5, 8, 10, 8, 4, 1][((now_ms / 100) % 8) as usize] } else { 2 };`
     - Prawa i lewa ręka krokodyla trzymające boki akordeonu powinny przesuwać się na osi X razem z pracą miecha.
5. **Głowa i ekspresja krokodyla:**
   - Krokodyl delikatnie kiwa głową w rytm gry (`bob = if active { ... } else { 0 }`),
   - Wesoły, muzykalny wyraz pyska grajka.

---

## 4. Wytyczne Techniczne dla GPT 6.1-Terra

1. **Układ współrzędnych ekranu:**
   - Całkowity wymiar okna nakładki: `x = 14..306` (szerokość 292 px), `y = 48..188` (wysokość 140 px).
   - Scena graficzna (lewa strona): `x = 14..180` (szerokość 166 px).
   - Panel kontrolny (prawa strona): `x = 180..306` (pionowa linia na `x = 180`).
   - Zachowaj nienaruszony obrys tła osi X: `graphics.rect(14, 48, 292, 140)`, aby zapobiec powidokom fali sampla.
2. **Kompilacja i testy:**
   - Po wprowadzeniu zmian uruchom testy jednostkowe:
     ```bash
     export PATH="/opt/homebrew/bin:/opt/homebrew/Cellar/rustup/1.29.3/bin:$PATH"
     cargo test --offline
     ```
   - Wszystkie 83 testy (oraz ewentualne nowe) muszą przejść na zielono (`83 passed; 0 failed`).
3. **Commit i podsumowanie:**
   - Sformatuj commit jako: `feat(ui): redesign right panel layout and add accordion gator for acordator mode (v0.0.4.8)`
   - Utwórz tag `v0.0.4.8`.
   - Przedstaw krótkie, konkretne podsumowanie wykonanych prac i koordynatów dla wątku zarządzającego.
