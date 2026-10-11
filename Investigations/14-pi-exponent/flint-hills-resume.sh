#!/usr/bin/env bash
# Resumable version of flint-hills.sh: appends to OUT and, if OUT already has
# rows, continues after its last n with its last partial sum. Same row format.
# Run from the repository root. Usage: flint-hills-resume.sh N OUT
D=target/release/dz-cli.exe
N=${1:-2000}; OUT=${2:-Investigations/14-pi-exponent/flint-hills.out}
S=0; START=1
if [ -s "$OUT" ]; then
  last=$(tail -1 "$OUT")
  START=$(( $(echo "$last" | sed 's/^n=\([0-9]*\) .*/\1/') + 1 ))
  S=$(echo "$last" | sed 's/.* sum=\([^ ]*\) .*/\1/')
fi
for ((n=START;n<=N;n++)); do
  o=$($D eval "1/($n^3*sin($n)^2)" --input-base 10 --mode fixed --scale 12 --gates)
  t=$(echo "$o" | head -1 | tr -d '~'); g=$(echo "$o" | sed -n 's/.*total=\([0-9]*\).*/\1/p')
  S=$($D eval "$S+$t" --mode fixed --scale 12 | tr -d '~')
  echo "n=$n dz=$($D eval "$n" --input-base 10) term=$t sum=$S gates=$g" >> "$OUT"
done
