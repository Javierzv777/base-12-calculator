#!/usr/bin/env bash
# Same measurement as mertens.sh, but primality uses the exact bound
# isqrt(n)-1 (dz-cli >= f0a4dd4): exists "isqrt(<n>)-1" "<n> % (i+2) = 0".
# Each row also records the gates of that primality check. RESUMABLE: if
# OUT already has rows, the primes and M are re-read from it and the run
# continues after its last n. Run from the repository root.
# Usage: mertens-isqrt.sh N OUT   (N decimal, loop index only)
D=target/release/dz-cli.exe
N=${1:-10000}; OUT=${2:-Investigations/13-riemann/mertens-isqrt-10000.out}
disp() { sed -n 's/.*"display":"\([^"]*\)".*/\1/p'; }
declare -a PRIMES; declare -A ISPRIME
M=1; START=2
if [ -s "$OUT" ]; then
  while read -r line; do
    nd=$(echo "$line" | sed 's/^n=\([^ ]*\) .*/\1/')
    case "$line" in *kind=prime*) PRIMES+=("$nd"); ISPRIME[$nd]=1;; esac
  done < "$OUT"
  M=$(tail -1 "$OUT" | sed 's/.* M=\([^ ]*\) .*/\1/')
  START=$(( $(wc -l < "$OUT") + 2 ))
fi
for ((i=START;i<=N;i++)); do
  nd=$($D eval "$i" --input-base 10)
  ex=$($D exists "isqrt($nd)-1" "$nd % (i+2) = 0" --gates)
  pg=$(echo "$ex" | sed -n 's/.*total=\([0-9]*\).*/\1/p')
  if echo "$ex" | grep -q '^holds: false'; then
    mu=-1; PRIMES+=("$nd"); ISPRIME[$nd]=1; kind=prime
  else
    kind=comp; cof=$nd; parity=0; mu=
    for p in "${PRIMES[@]}"; do
      o=$($D valuation "$cof" "$p")
      e=$(echo "$o" | sed -n 's/^exponent: //p'); cof=$(echo "$o" | sed -n 's/^cofactor: //p')
      case "$e" in 0) ;; 1) parity=$((1-parity));; *) mu=0; break;; esac
      if [ "$cof" = "1" ]; then break; fi
      if [ -n "${ISPRIME[$cof]}" ]; then parity=$((1-parity)); break; fi
    done
    if [ -z "$mu" ]; then if [ $parity = 1 ]; then mu=-1; else mu=1; fi; fi
  fi
  if [ "$mu" != 0 ]; then M=$($D eval "$M+($mu)"); fi
  ok=$($D eval "sign($nd-($M)*($M))")
  echo "n=$nd kind=$kind mu=$mu M=$M last=${nd: -1} mertens_ok=$ok prime_gates=$pg" >> "$OUT"
done
