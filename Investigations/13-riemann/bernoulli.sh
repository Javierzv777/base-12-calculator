#!/usr/bin/env bash
# Bernoulli numbers B_0..B_N computed INSIDE dz-cli (rational mode) with the
# Akiyama-Tanigawa recurrence: A[m] = 1/(m+1); A[j-1] = j*(A[j-1]-A[j]).
# The loop lives here; every value and every gate is the binary's.
# Then zeta(1-2k) = -B_2k/(2k), also evaluated by dz-cli.
# Run from the repository root. Usage: bernoulli.sh N  (N in decimal)
D=target/release/dz-cli.exe
N=${1:-40}
disp() { sed -n 's/.*"display":"\([^"]*\)".*/\1/p'; }
tot()  { sed -n 's/.*"total":\([0-9]*\).*/\1/p'; }
# Dozenal names of the loop counters, read by the instrument from base 10.
declare -a DZ
for ((i=0;i<=N+1;i++)); do DZ[$i]=$($D eval "$i" --input-base 10 --json | disp); done
declare -a A
GATES=0
for ((m=0;m<=N;m++)); do
  A[$m]="1/${DZ[$((m+1))]}"
  for ((j=m;j>=1;j--)); do
    o=$($D eval "${DZ[$j]}*((${A[$((j-1))]})-(${A[$j]}))" --json)
    A[$((j-1))]=$(echo "$o" | disp); GATES=$((GATES + $(echo "$o" | tot)))
  done
  if (( m % 2 == 0 && m >= 2 )); then
    k2=${DZ[$m]}
    z=$($D eval "-(${A[0]})/${k2}" --json)
    zd=$(echo "$z" | disp); per=$(echo "$z" | sed -n 's/.*"periodic_note":\([a-z]*\).*/\1/p')
    echo "2k=$m (dz $k2)  B=${A[0]}  zeta(1-2k)=$zd  periodic=$per  cum_gates=$GATES"
  fi
done
