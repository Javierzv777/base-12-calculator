#!/usr/bin/env bash
# mertens-isqrt.sh with a faster factorization: the primality check
#   exists "isqrt(<c>)-1" "<c> % (i+2) = 0"
# returns, when it holds, the witness i of the SMALLEST divisor i+2 of c,
# which is therefore prime. So mu(n) peels off smallest prime factors:
# witness -> eval "w+2" -> valuation; a cofactor already known prime (all
# primes below n are known, in order) ends the peel, otherwise one more
# exists. 1-3 calls per factor instead of a sweep over the prime list.
# Same row format as mertens-isqrt.sh; RESUMABLE from OUT; stops by itself
# after BUDGET seconds (default 6600) so each run fits a 2 h window.
# Run from the repository root. Usage: mertens-witness.sh N OUT [BUDGET]
D=target/release/dz-cli.exe
N=${1:-10000}; OUT=${2:-Investigations/13-riemann/mertens-isqrt-10000.out}; BUDGET=${3:-6600}
declare -A ISPRIME
M=1; START=2
if [ -s "$OUT" ]; then
  while read -r line; do
    case "$line" in *kind=prime*) nd=${line#n=}; nd=${nd%% *}; ISPRIME[$nd]=1;; esac
  done < "$OUT"
  M=$(tail -1 "$OUT" | sed 's/.* M=\([^ ]*\) .*/\1/')
  START=$(( $(wc -l < "$OUT") + 2 ))
fi
SECONDS=0
for ((i=START;i<=N;i++)); do
  (( SECONDS > BUDGET )) && { echo "budget reached before n=$i (decimal index)"; exit 0; }
  nd=$($D eval "$i" --input-base 10)
  ex=$($D exists "isqrt($nd)-1" "$nd % (i+2) = 0" --gates)
  pg=$(echo "$ex" | sed -n 's/.*total=\([0-9]*\).*/\1/p')
  if echo "$ex" | grep -q '^holds: false'; then
    mu=-1; ISPRIME[$nd]=1; kind=prime
  else
    kind=comp; cof=$nd; parity=0; mu=
    while :; do
      w=$(echo "$ex" | sed -n 's/^witness: //p')
      p=$($D eval "$w+2")
      o=$($D valuation "$cof" "$p")
      e=$(echo "$o" | sed -n 's/^exponent: //p'); cof=$(echo "$o" | sed -n 's/^cofactor: //p')
      case "$e" in 1) parity=$((1-parity));; *) mu=0; break;; esac
      [ "$cof" = "1" ] && break
      if [ -n "${ISPRIME[$cof]}" ]; then parity=$((1-parity)); break; fi
      ex=$($D exists "isqrt($cof)-1" "$cof % (i+2) = 0")
    done
    if [ -z "$mu" ]; then if [ $parity = 1 ]; then mu=-1; else mu=1; fi; fi
  fi
  if [ "$mu" != 0 ]; then M=$($D eval "$M+($mu)"); fi
  ok=$($D eval "sign($nd-($M)*($M))")
  echo "n=$nd kind=$kind mu=$mu M=$M last=${nd: -1} mertens_ok=$ok prime_gates=$pg" >> "$OUT"
done
echo "done up to n=$N"
