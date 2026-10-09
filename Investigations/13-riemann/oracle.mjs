// EXTERNAL ORACLE (not the instrument): exact Bernoulli numbers with BigInt,
// zeta(1-2k) = -B_2k/(2k), its reduced denominator factored, and whether it
// terminates in base 10 / base 12 / base 6. Used only to cross-check dz-cli.
const gcd = (a, b) => { a = a < 0n ? -a : a; b = b < 0n ? -b : b; while (b) [a, b] = [b, a % b]; return a; };
const fr = (n, d) => { if (d < 0n) { n = -n; d = -d; } const g = gcd(n, d) || 1n; return [n / g, d / g]; };
const sub = ([a, b], [c, d]) => fr(a * d - c * b, b * d);
const mulI = ([a, b], k) => fr(a * k, b);
const N = Number(process.argv[2] || 120);
const B = [];
const A = [];
for (let m = 0; m <= N; m++) {
  A[m] = fr(1n, BigInt(m + 1));
  for (let j = m; j >= 1; j--) A[j - 1] = mulI(sub(A[j - 1], A[j]), BigInt(j));
  B[m] = A[0];
}
const factor = (n) => { const f = []; for (let p = 2n; p * p <= n; p++) { let e = 0; while (n % p === 0n) { n /= p; e++; } if (e) f.push(e > 1 ? `${p}^${e}` : `${p}`); } if (n > 1n) f.push(`${n}`); return f.join('*'); };
const smooth = (d, ps) => { for (const p of ps) while (d % p === 0n) d /= p; return d === 1n; };
for (let m = 2; m <= N; m += 2) {
  const z = fr(-B[m][0], B[m][1] * BigInt(m));
  const d = z[1];
  console.log(`2k=${m} zeta(1-2k)=${z[0]}/${d} den=${factor(d)} term10=${smooth(d,[2n,5n])} term12=${smooth(d,[2n,3n])} term6=${smooth(d,[2n,3n])}`);
}
