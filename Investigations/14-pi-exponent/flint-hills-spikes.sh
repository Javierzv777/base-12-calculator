#!/usr/bin/env bash
# The large Flint-Hills terms come from n = p, numerators of the continued-
# fraction convergents p/q of pi, where |sin p| = |sin(p - q pi)| ~ 1/q.
# This evaluates ONLY those terms (one dz-cli call each, cheap), at a scale
# large enough that sin(p) keeps its significant digits. The numerators are
# the classical ones (decimal); they can be re-derived from cf.out.
# Run from the repository root. Usage: flint-hills-spikes.sh [SCALE]
D=target/release/dz-cli.exe
SC=${1:-24}
for p in 3 22 333 355 103993 104348 208341 312689 833719 1146408 4272943 5419351; do
  o=$($D eval "1/($p^3*sin($p)^2)" --input-base 10 --mode fixed --scale $SC --gates)
  echo "p=$p dz=$($D eval "$p" --input-base 10) term=$(echo "$o" | head -1) $(echo "$o" | tail -1)"
done
