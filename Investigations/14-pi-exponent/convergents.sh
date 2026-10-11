#!/usr/bin/env bash
# P2 of HANDOFF.md: local exponent of every continued-fraction convergent of pi,
#   lambda_n = -log|pi - p_n/q_n| / log q_n,
# all values computed by dz-cli. p_n, q_n come from the certified quotients
# (p_n = a_n p_{n-1} + p_{n-2}, same for q) in exact integer arithmetic.
# The error needs no division: with T = 12^N and LAMBDA = L*T (L = the
# instrument's pi truncated to N dozenal digits), E = |q*LAMBDA - p*T| is an
# exact integer and |L - p/q| = E / (q*T). If E has len digits, E/T = 12^-z * m
# with z = N - len and m = 0;<digits of E> in [1/12, 1), so
#   lambda = 1 + (z - log m) / log q,
# and log() stays inside its 12-digit working scale. z is a position count,
# read into dozenal with gates (--input-base 10).
# L is only within 12^-N of pi, so E/T carries an error up to q*12^-N and its
# relative error is below 12^-R only while z + len(q) <= N-R, with R = 8 reliable
# digits by default. Past that the row is not trusted and the run stops. R is
# far stricter than lambda needs: R digits in E bound the error of lambda by
# about 12^-R / (len(q) ln 12), so a smaller R (env RELIABLE) is enough to
# read one more row near the end of the digits.
# (The first version divided p/q at scale 100; same lambda to 12 digits on the
# 95 rows it covered, but that division does not scale to 400 digits.)
# Run from the repository root. Usage: convergents.sh [CF_OUT] > OUT
# Env: PI_FILE (default pi-scale100.out) and DIGITS (default 99); they must
# match the run that produced CF_OUT. START (default 0): for n < START only the
# p/q recurrence advances (cheap integer evals) and nothing is printed, so a
# rerun with more digits measures just its new rows. RELIABLE (default 8): R
# above.
D=target/release/dz-cli.exe
CF=${1:-Investigations/14-pi-exponent/cf-resume.out}
N=${DIGITS:-99}; START=${START:-0}; R=${RELIABLE:-8}
P=$(sed 's/^([0-9]* s) //; s/^~//' "${PI_FILE:-Investigations/14-pi-exponent/pi-scale100.out}")
LAM="3${P:2:N}"
T="1$(printf '0%.0s' $(seq 1 $N))"
pm2=0; pm1=1; qm2=1; qm1=0
echo "n a p q z lambda"
grep '^a[0-9]*=' "$CF" | while IFS='=' read -r idx a; do
  n=${idx#a}
  p=$($D eval "$a*$pm1+$pm2"); q=$($D eval "$a*$qm1+$qm2")
  pm2=$pm1; pm1=$p; qm2=$qm1; qm1=$q
  [ "$n" -lt "$START" ] && continue                        # recurrence only
  [ "$n" = 0 ] && { echo "$n $a $p $q - -"; continue; }   # q = 1: log q = 0
  E=$($D eval "abs($q*$LAM-$p*$T)")
  z=$((N - ${#E}))
  [ $((z + ${#q})) -gt $((N-R)) ] && { echo "stop: fewer than $R reliable digits at n=$n, past the $N digits of L"; break; }
  zdz=$($D eval "$z" --input-base 10)
  lam=$($D eval "1 + ($zdz - log(0;${E:0:20}))/log($q)" --mode fixed --scale 12 | tr -d '~')
  echo "$n $a $p $q $zdz $lam"
done
