#!/usr/bin/env bash
# For n = 2..N (N decimal, loop index only): the instrument names n in
# dozenal, decides primality with a bounded `exists`, factors composites
# with `valuation`, and accumulates the Mertens function M(n) with `eval`.
# It also checks the (false in general, true here) Mertens inequality
# M(n)^2 < n exactly. The prime's residue class mod 12 is its last dozenal
# digit, read from the string, no gates. Output: one line per n.
# Run from the repository root.
D=target/release/dz-cli.exe
N=${1:-1728}
disp() { sed -n 's/.*"display":"\([^"]*\)".*/\1/p'; }
declare -a PRIMES; declare -A ISPRIME
M=1
for ((i=2;i<=N;i++)); do
  nd=$($D eval "$i" --input-base 10)   # ASCII aliases X/E: one byte per digit
  ex=$($D exists "$nd-2" "$nd % (i+2) = 0")
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
  if [ "$mu" != 0 ]; then M=$($D eval "$M+($mu)" --json | disp); fi
  ok=$($D eval "sign($nd-($M)*($M))" --json | disp)
  echo "n=$nd kind=$kind mu=$mu M=$M last=${nd: -1} mertens_ok=$ok"
done
