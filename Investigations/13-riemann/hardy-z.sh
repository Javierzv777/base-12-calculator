#!/usr/bin/env bash
# Hardy's Z(t) = e^{i theta(t)} zeta(1/2 + i t), real for real t, by the
# Riemann-Siegel formula truncated at the C0 correction:
#   Z(t) ~ 2 sum_{n<=N} n^{-1/2} cos(theta - t ln n)
#          + (-1)^{N-1} (t/2pi)^{-1/4} cos(2pi(p^2-p-1/16)) / cos(2pi p)
#   theta(t) ~ t/2 ln(t/2pi) - t/2 - pi/8 + 1/(48t) + 7/(5760 t^3)
#   N = floor(sqrt(t/2pi)), p = frac(sqrt(t/2pi)).
# Everything is DOZENAL text (constants converted by dz-cli: 16->14,
# 48->40, 5760->3400). N is passed in (the caller knows which interval it
# brackets). A sign change of the real function Z between a and b proves a
# zero of zeta ON the critical line in (a,b) -- up to the formula's error.
# Bisection midpoints are computed by the instrument.
D=target/release/dz-cli.exe
OPT="--mode fixed --scale 10"
z_expr() { # $1 = t (dozenal), $2 = N
  local t="($1)" N=$2
  local u="($t/(2*pi))" th="($t/2*ln($t/(2*pi)) - $t/2 - pi/8 + 1/(40*$t) + 7/(3400*$t^3))"
  local p="(sqrt($u) - $N)" s="" sg="" n
  for ((n=1;n<=N;n++)); do s="${s:+$s + }2*cos($th - $t*ln($n))/sqrt($n)"; done
  if (( N % 2 == 1 )); then sg="+"; else sg="-"; fi
  echo "$s $sg cos(2*pi*($p*$p - $p - 1/14))/cos(2*pi*$p)/sqrt(sqrt($u))"
}
sgn() { $D eval "sign($(z_expr "$1" "$2"))" $OPT | tr -d '~'; }
bisect() { # a b N iterations
  local a=$1 b=$2 N=$3 it=$4 sa sb sm m i
  sa=$(sgn "$a" "$N"); sb=$(sgn "$b" "$N")
  [ "$sa" = "$sb" ] && { echo "no sign change on [$a,$b]: $sa $sb"; return; }
  for ((i=0;i<it;i++)); do
    m=$($D eval "($a+$b)*0;6" $OPT | tr -d '~')
    sm=$(sgn "$m" "$N")
    if [ "$sm" = "$sa" ]; then a=$m; else b=$m; fi
  done
  echo "zero in [$a, $b]  (N=$N, Z(a) sign $sa)"
}
if [ "$1" = "eval" ]; then $D eval "$(z_expr "$2" "$3")" $OPT --gates; exit; fi
bisect 12 "12;3" 1 ${IT:-24}
bisect "1X" "19;0" 1 ${IT:-24}
bisect 21 "21;1" 1 ${IT:-24}
bisect "26;4" "26;6" 2 ${IT:-24}
bisect "28;E" "29;0" 2 ${IT:-24}
