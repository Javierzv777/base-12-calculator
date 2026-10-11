#!/usr/bin/env bash
# P3 of HANDOFF.md: the interval-certified continued fraction of cf-resume.sh,
# rewritten so it scales to a few hundred digits. cf-resume.sh carries the
# complete quotient x_k as a reduced fraction, and DzRational::new runs Euclid
# on every step: fine at 99 digits, tens of hours at 400. Here everything is an
# integer. With T = 12^N and LAMBDA = L*T (L = pi truncated to N dozenal
# digits, so LAMBDA is pi's first N+1 digits read as an integer),
#   x_k = |q_{k-2}*LAMBDA - p_{k-2}*T| / |q_{k-1}*LAMBDA - p_{k-1}*T|,
# so a_k is the quotient of ONE integer divrem (a small quotient, cheap) and
# the convergents p_k, q_k advance with integer mul/add. U = L + 12^-N is the
# same with LAMBDA + 1; while the two CFs agree they share p and q. Every value
# is the binary's; the loop and the state file live here (deleted when the run
# finishes, so a rerun starts over).
# Run from the repository root. Usage: cf-int.sh [OUT] [STATE]
# Env: PI_FILE (default pi-scale400.out) and DIGITS (default 399).
D=target/release/dz-cli.exe
OUT=${1:-Investigations/14-pi-exponent/cf-int.out}
STATE=${2:-Investigations/14-pi-exponent/cf-int.state}
N=${DIGITS:-399}
P=$(sed 's/^([0-9]* s) //; s/^~//' "${PI_FILE:-Investigations/14-pi-exponent/pi-scale400.out}")
LAM="3${P:2:N}"                                   # pi's first N+1 digits, no point
LAM1=$($D eval "$LAM+1")
T="1$(printf '0%.0s' $(seq 1 $N))"                # 12^N
quot() { $D divrem "$1" "$2" | sed -n 's/^quotient: //p'; }
if [ -s "$STATE" ]; then
  read -r k pm2 pm1 qm2 qm1 < "$STATE"
else
  k=0; pm2=0; pm1=1; qm2=1; qm1=0; : > "$OUT"
fi
while :; do
  a=$(quot "$($D eval "abs($qm2*$LAM-$pm2*$T)")" "$($D eval "abs($qm1*$LAM-$pm1*$T)")")
  b=$(quot "$($D eval "abs($qm2*$LAM1-$pm2*$T)")" "$($D eval "abs($qm1*$LAM1-$pm1*$T)")")
  [ -z "$a" ] || [ -z "$b" ] && { echo "error: empty quotient at index $k" >> "$OUT"; exit 1; }
  [ "$a" != "$b" ] && { echo "stop: L gives $a, U gives $b at index $k" >> "$OUT"; rm -f "$STATE"; break; }
  echo "a$k=$a" >> "$OUT"
  p=$($D eval "$a*$pm1+$pm2"); q=$($D eval "$a*$qm1+$qm2")
  pm2=$pm1; pm1=$p; qm2=$qm1; qm1=$q; k=$((k+1))
  echo "$k $pm2 $pm1 $qm2 $qm1" > "$STATE.tmp" && mv "$STATE.tmp" "$STATE"
done
