// External oracle, planning only: how many quotients the interval method of
// cf-int.sh certifies from N dozenal digits of pi (L = pi truncated to N digits,
// U = L + 12^-N), for a range of N. Usage: node cf-reach.mjs FROM TO
const NN = 700n, S = 10n**NN;
const atanInv = x => { let t = S / x, s = t, k = 1n, x2 = x*x; while (t) { t /= x2; k += 2n; s += ((k-1n)/2n % 2n ? -1n : 1n) * (t / k); } return s; };
const PI = 16n*atanInv(5n) - 4n*atanInv(239n);
const cf = (num, den, max) => { const r = []; while (den && r.length < max) { const a = num / den; r.push(a); [num, den] = [den, num - a*den]; } return r; };
for (let N = +process.argv[2]; N <= +process.argv[3]; N++) {
  const T = 12n ** BigInt(N), LAM = PI * T / S;          // floor(pi * 12^N)
  const a = cf(LAM, T, 600), b = cf(LAM + 1n, T, 600);
  let k = 0; while (k < a.length && a[k] === b[k]) k++;
  console.log(`N=${N} certified=${k}${k > 431 ? ' (includes a431=' + a[431] + ')' : ''}`);
}
