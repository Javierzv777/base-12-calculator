#!/usr/bin/env bash
# P2 of HANDOFF.md: local exponent of every continued-fraction convergent of pi,
#   lambda_n = -log|pi - p_n/q_n| / log q_n,
# all values computed by dz-cli. p_n, q_n come from the certified quotients
# (p_n = a_n p_{n-1} + p_{n-2}, same for q) in exact rational mode; the error
# d = |L - p_n/q_n| is taken in fixed mode at scale 100 against L = the
# instrument's pi truncated to 99 dozenal digits. To keep log() inside the
# 12-digit working scale, d is split as 12^-z * m with m = 0;<digits after the
# z leading zeros> (z is a position count, read into dozenal with gates), so
#   lambda = (z - log m) / log q.
# Rows whose error has fewer than 8 reliable digits (z > 91) are not trusted:
# L itself is only within 12^-99 of pi. The run stops there.
# Run from the repository root. Usage: convergents.sh [CF_OUT] > OUT
D=target/release/dz-cli.exe
CF=${1:-Investigations/14-pi-exponent/cf-resume.out}
P=$(sed 's/^([0-9]* s) ~//' Investigations/14-pi-exponent/pi-scale100.out)
L="${P:0:101}"
pm2=0; pm1=1; qm2=1; qm1=0
echo "n a p q z lambda"
grep '^a[0-9]*=' "$CF" | while IFS='=' read -r idx a; do
  n=${idx#a}
  p=$($D eval "$a*$pm1+$pm2"); q=$($D eval "$a*$qm1+$qm2")
  pm2=$pm1; pm1=$p; qm2=$qm1; qm1=$q
  [ "$n" = 0 ] && { echo "$n $a $p $q - -"; continue; }   # q = 1: log q = 0
  d=$($D eval "abs($L-$p/$q)" --mode fixed --scale 100 | tr -d '~' | cut -d' ' -f1)
  frac=${d#0;}; zs=${frac%%[^0]*}; z=${#zs}
  [ "$z" -gt 91 ] && { echo "stop: error below 12^-91 at n=$n, past the 99 digits of L"; break; }
  zdz=$($D eval "$z" --input-base 10)
  lam=$($D eval "($zdz - log(0;${frac:z:20}))/log($q)" --mode fixed --scale 12 | tr -d '~')
  echo "$n $a $p $q $zdz $lam"
done
