#!/usr/bin/env bash
# Continued fraction of pi computed INSIDE dz-cli (rational mode, exact),
# made rigorous by interval: pi lies in [L, U] with L = the instrument's pi
# truncated to 99 dozenal digits (99 digits verified, see pi-scale100.out)
# and U = L + 12^-99. A partial quotient is certified when the CFs of L and
# U agree on it. The loop lives here; every value is the binary's.
# Run from the repository root. Output: one line per certified quotient.
D=target/release/dz-cli.exe
disp() { sed -n 's/.*"display":"\([^"]*\)".*/\1/p'; }
P=$(sed 's/^([0-9]* s) ~//' Investigations/14-pi-exponent/pi-scale100.out)
L="${P:0:101}"                                   # "3;" + 99 digits
ONE="0;$(printf '0%.0s' $(seq 1 98))1"           # 12^-99
x=$L; y=$($D eval "$L+$ONE" --json | disp)
x=$($D eval "$x" --json | disp)
k=0
while :; do
  a=$($D eval "floor($x)" --json | disp); b=$($D eval "floor($y)" --json | disp)
  [ "$a" != "$b" ] && { echo "stop: L gives $a, U gives $b at index $k"; break; }
  echo "a$k=$a"
  x=$($D eval "1/(($x)-$a)" --json | disp); y=$($D eval "1/(($y)-$b)" --json | disp)
  k=$((k+1))
done
