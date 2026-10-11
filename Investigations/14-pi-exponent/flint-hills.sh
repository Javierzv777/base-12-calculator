#!/usr/bin/env bash
# Partial sums of the Flint-Hills series sum 1/(n^3 sin^2 n), n = 1..N, every
# term and every sum by dz-cli (fixed mode, scale 12; sin is approximate, so
# every value carries ~). n is a decimal loop index read with --input-base 10.
# Output: n (decimal), n (dozenal), term, partial sum, gates of the term.
# Run from the repository root. Usage: flint-hills.sh N
D=target/release/dz-cli.exe
N=${1:-400}; S=0
for ((n=1;n<=N;n++)); do
  o=$($D eval "1/($n^3*sin($n)^2)" --input-base 10 --mode fixed --scale 12 --gates)
  t=$(echo "$o" | head -1 | tr -d '~'); g=$(echo "$o" | sed -n 's/.*total=\([0-9]*\).*/\1/p')
  S=$($D eval "$S+$t" --mode fixed --scale 12 | tr -d '~')
  echo "n=$n dz=$($D eval "$n" --input-base 10) term=$t sum=$S gates=$g"
done
