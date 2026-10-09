#!/usr/bin/env bash
# Robin's criterion (1984): RH <=> sigma(n) < e^gamma * n * ln ln n for all
# n > 5040; if it ever fails, it fails at a colossally abundant number.
# For each CA number (list taken from the literature, OEIS A004490 -- not
# verified here; the inequality check does not depend on the CA property),
# the instrument computes, from the factorization: n in dozenal, sigma(n)
# exactly, and the Robin ratio  sigma(n) / (n ln ln n)  against e^gamma,
# with gamma by Euler-Maclaurin at 12 (see the doc). Input is base 10.
D=target/release/dz-cli.exe
G="(1+1/2+1/3+1/4+1/5+1/6+1/7+1/8+1/9+1/10+1/11+1/12) - ln(12) - 1/24 + 1/1728 - 1/(120*12^4) + 1/(252*12^6) - 1/(240*12^8)"
EG=$($D eval "exp($G)" --input-base 10 --mode fixed --scale 12)
echo "e^gamma = $EG"
while read -r fac; do
  [ -z "$fac" ] && continue
  n=$(echo "$fac" | sed 's/\^/^/g')
  sig=""
  for t in $(echo "$fac" | tr '*' ' '); do
    p=${t%%^*}; a=1; [[ "$t" == *^* ]] && a=${t##*^}
    sig="${sig:+$sig*}(($p^($a+1)-1)/($p-1))"
  done
  nd=$($D eval "$n" --input-base 10)
  sd=$($D eval "$sig" --input-base 10)
  r=$($D eval "($sig)/(($n)*ln(ln($n)))" --input-base 10 --mode fixed --scale 12)
  d=$($D eval "exp($G) - ($sig)/(($n)*ln(ln($n)))" --input-base 10 --mode fixed --scale 12 --gates | tr '\n' ' ')
  echo "n=$fac | dozenal=$nd | sigma=$sd | ratio=$r | e^g-ratio=$d"
done <<'LIST'
2^3*3^2*5*7
2^4*3^2*5*7
2^4*3^2*5*7*11
2^4*3^2*5*7*11*13
2^5*3^2*5*7*11*13
2^5*3^3*5*7*11*13
2^5*3^3*5^2*7*11*13
2^5*3^3*5^2*7*11*13*17
2^5*3^3*5^2*7*11*13*17*19
2^5*3^3*5^2*7*11*13*17*19*23
2^6*3^3*5^2*7*11*13*17*19*23
2^6*3^3*5^2*7*11*13*17*19*23*29
2^6*3^3*5^2*7*11*13*17*19*23*29*31
2^6*3^3*5^2*7^2*11*13*17*19*23*29*31
LIST
