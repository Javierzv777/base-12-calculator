#!/usr/bin/env bash
# Re-measure the prime-power decode of 12-hallazgo-godel-incompletitud.md
# with the new `valuation` subcommand: one call per prime, the cofactor
# fed to the next prime. Run from the repository root after
# `cargo build -p dz-cli --release`. The loop and the sums live here; the
# gate counts are the binary's.
D=target/release/dz-cli.exe
tot() { echo "$1" | grep -o 'total=[0-9]*' | cut -d= -f2; }
dep() { echo "$1" | grep -o 'depth=[0-9]*' | cut -d= -f2; }
decode() {
  N0=$1; shift; N=$N0; T=0; DS=0; calls=0; exps=""
  for p in "$@"; do
    o=$($D valuation "$N" "$p" --gates)
    e=$(echo "$o" | grep '^exponent:' | cut -d' ' -f2)
    N=$(echo "$o" | grep '^cofactor:' | cut -d' ' -f2)
    T=$((T + $(tot "$o"))); DS=$((DS + $(dep "$o"))); calls=$((calls + 1))
    exps="$exps $e"
  done
  echo "N=$N0 exps=[$exps ] remainder=$N calls=$calls total_gates=$T serial_depth_sum=$DS"
}
decode 8 2 3
decode 20 2 3
decode 8820 2 3 5
decode 50920 2 3 5 7
decode 333E2E10X0 2 3 5 7 E
decode 689E28E40XE177286X0 2 3 5 7 E 11
