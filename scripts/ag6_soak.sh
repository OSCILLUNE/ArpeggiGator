#!/usr/bin/env bash
# 0.0.6 staged soak monitor. Read-only: polls the guest, never presses controls.
# Usage: scripts/ag6_soak.sh MINUTES LABEL
set -uo pipefail
ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
MINUTES="${1:-30}"
LABEL="${2:-soak}"
LAB="${LAB:-/Users/fafi/Downloads/virtual-mpc-sample}"
PORT="${PORT:-2022}"
KEY="$LAB/out/keys/id_ed25519"
STAMP="$(date +%Y%m%d_%H%M%S)"
OUT="$ROOT/docs/logs/ag6/soak_${LABEL}_${STAMP}.jsonl"
STOP="$ROOT/docs/logs/ag6/soak_${LABEL}_${STAMP}.STOP"
N=$(( MINUTES * 6 ))
START_TS="$(date -u +'%Y-%m-%d %H:%M:%S')"

for ((i=1; i<=N; i++)); do
  TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  DATA="$(ssh -o BatchMode=yes -o ConnectTimeout=5 -i "$KEY" -p "$PORT" root@127.0.0.1 \
    'printf "restarts="; systemctl show ac50 -p NRestarts --value; \
     printf "pid="; systemctl show ac50 -p MainPID --value; \
     printf "hb="; tail -1 /tmp/arpeggigator-6-events.log 2>/dev/null; \
     printf "abrt="; journalctl -u ac50 -b --since "$START_TS" --no-pager 2>/dev/null | grep -ciE "double free|ABRT|SEGV|dumped"' 2>/dev/null)"
  printf '{"t":"%s","i":%d,"raw":%s}\n' "$TS" "$i" "$(python3 -c 'import json,sys; print(json.dumps(sys.argv[1]))' "$DATA")" >> "$OUT"
  RESTARTS="$(printf '%s' "$DATA" | sed -n 's/^restarts=//p' | head -1)"
  ABRT="$(printf '%s' "$DATA" | sed -n 's/^abrt=//p' | head -1)"
  if [ "${ABRT:-0}" != "0" ] || { [ -n "${RESTARTS:-}" ] && [ "$RESTARTS" -gt 0 ] 2>/dev/null; }; then
    printf '%s STOP restarts=%s abrt=%s\n' "$TS" "$RESTARTS" "$ABRT" > "$STOP"
    echo "STOP at $TS: restarts=$RESTARTS abrt=$ABRT -> $OUT"
    exit 2
  fi
  sleep 10
done
echo "OK $MINUTES min: $OUT"
