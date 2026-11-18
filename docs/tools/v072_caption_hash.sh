#!/bin/bash
# CZESIEK-3 / weryfikacja wizualna v0.0.7.2 — obiektywna kontrola napisów LCD 320x240.
# Tylko lokalna obrobka obrazu: sips + shasum + cmp. Nie uruchamia probe'ow,
# nie laczy sie z maszynami DEV/QA, nie dotyka fizycznej MPC, nie modyfikuje src/.
#
# UZYCIE:
#   bash docs/tools/v072_caption_hash.sh crop <screen.png> <prefix>
#   bash docs/tools/v072_caption_hash.sh diff <prefixA> <prefixB>
#
# Regiony (wspolrzedne z src/splash.rs, LCD 320x240; PROVEN z odczytu kodu):
#   B1_TITLE   x=13  y=5   w=88  h=19  -> napis MUTATOR / ACCORDATOR (lewy gorna cella B1)
#   MODE_TITLE x=185 y=54  w=118 h=16  -> napis "MODE: MUTATOR" / "MODE: ACCORD"
#   PARAM_LINE x=185 y=72  w=118 h=16  -> napis "SUBTLE FRZ:OFF" (prawy panel)
# Margines 1px dodany dla MODE_TITLE i PARAM_LINE (x=184,y=53/71, 119x18).

set -euo pipefail

REGIONS="B1_TITLE:19x88@5,13 MODE_TITLE:18x119@53,184 PARAM_LINE:18x119@71,184"

crop_one() {
  local src="$1" prefix="$2"
  echo "# crop: $src -> $prefix"
  for spec in $REGIONS; do
    local name="${spec%%:*}" rest="${spec#*:}"
    local wh="${rest%@*}" off="${rest#*@}"
    local h="${wh%x*}" w="${wh#*x}"
    local oy="${off%,*}" ox="${off#*,}"
    sips -c "$h" "$w" --cropOffset "$oy" "$ox" "$src" --out "${prefix}_${name}.png" >/dev/null
    sips -s format bmp "${prefix}_${name}.png" --out "${prefix}_${name}.bmp" >/dev/null
    printf '%s  %s\n' "$(shasum -a 256 "${prefix}_${name}.png" | awk '{print $1}')" "${prefix}_${name}.png"
  done
}

diff_one() {
  local a="$1" b="$2"
  echo "# diff: $a vs $b (BMP bez 54-bajtowego naglowka; 0 = identyczny region)"
  for spec in $REGIONS; do
    local name="${spec%%:*}"
    local fa="${a}_${name}.bmp" fb="${b}_${name}.bmp"
    if [[ ! -f "$fa" || ! -f "$fb" ]]; then
      echo "$name  BRAK_PLIKU"
      continue
    fi
    local nbytes
    # cmp zwraca 1 gdy pliki sie roznia — pod pipefail trzeba to zjesc.
    nbytes=$( { cmp -l <(tail -c +55 "$fa") <(tail -c +55 "$fb") || true; } | wc -l | tr -d ' ')
    local total
    total=$(($(wc -c < "$fa") - 54))
    printf '%-12s differing_bytes=%s / %s (%.2f%%)\n' "$name" "$nbytes" "$total" \
      "$(python3 -c "print(100.0*$nbytes/$total)")"
  done
}

case "${1:-}" in
  crop) crop_one "$2" "$3" ;;
  diff) diff_one "$2" "$3" ;;
  *) echo "UZYCIE: $0 crop <screen.png> <prefix> | $0 diff <prefixA> <prefixB>" >&2; exit 2 ;;
esac
