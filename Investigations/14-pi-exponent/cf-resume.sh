#!/usr/bin/env bash
# Resumable version of cf.sh: same interval-certified continued fraction of pi
# (L = the instrument's pi truncated to 99 dozenal digits, U = L + 12^-99, a
# quotient is accepted only when the CFs of L and U agree), but the complete
# quotients x, y and the index k are written to STATE after every certified
# quotient, so a killed run continues where it stopped.
# Run from the repository root. Usage: cf-resume.sh [OUT] [STATE]
D=target/release/dz-cli.exe
OUT=${1:-Investigations/14-pi-exponent/cf-resume.out}
STATE=${2:-Investigations/14-pi-exponent/cf-resume.state}
disp() { sed -n 's/.*"display":"\([^"]*\)".*/\1/p'; }
if [ -s "$STATE" ]; then
  k=$(sed -n 1p "$STATE"); x=$(sed -n 2p "$STATE"); y=$(sed -n 3p "$STATE")
else
  P=$(sed 's/^([0-9]* s) ~//' Investigations/14-pi-exponent/pi-scale100.out)
  L="${P:0:101}"                                   # "3;" + 99 digits
  ONE="0;$(printf '0%.0s' $(seq 1 98))1"           # 12^-99
  x=$($D eval "$L" --json | disp); y=$($D eval "$L+$ONE" --json | disp)
  k=0; : > "$OUT"
fi
while :; do
  a=$($D eval "floor($x)" --json | disp); b=$($D eval "floor($y)" --json | disp)
  [ -z "$a" ] || [ -z "$b" ] && { echo "error: empty quotient at index $k" >> "$OUT"; exit 1; }
  [ "$a" != "$b" ] && { echo "stop: L gives $a, U gives $b at index $k" >> "$OUT"; break; }
  echo "a$k=$a" >> "$OUT"
  x=$($D eval "1/(($x)-$a)" --json | disp); y=$($D eval "1/(($y)-$b)" --json | disp)
  k=$((k+1))
  printf '%s\n%s\n%s\n' "$k" "$x" "$y" > "$STATE.tmp" && mv "$STATE.tmp" "$STATE"
done
