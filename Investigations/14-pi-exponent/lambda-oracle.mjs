// External oracle for P2: lambda_n = -ln|pi - p_n/q_n| / ln q_n with pi from
// Machin in BigInt (300 decimal digits) and the reference CF, compared with
// the instrument's convergents.out (dozenal lambda). Usage: node lambda-oracle.mjs FILE
import fs from 'fs';
const N = 300n, S = 10n**N;
const atanInv = x => { let t = S / x, s = t, k = 1n, x2 = x*x; while (t) { t /= x2; k += 2n; s += ((k-1n)/2n % 2n ? -1n : 1n) * (t / k); } return s; };
const PI = 16n*atanInv(5n) - 4n*atanInv(239n);
const lnBig = v => { const s = v.toString(); const h = Math.min(s.length, 17); return Math.log(+s.slice(0, h)) + (s.length - h) * Math.LN10; };
const v = c => 'X↊'.includes(c) ? 10 : 'E↋'.includes(c) ? 11 : +c;
const dzI = s => [...s].reduce((x, c) => x*12n + BigInt(v(c)), 0n);
const dzF = s => { const [i, f = ''] = s.split(';'); let x = Number(dzI(i)), w = 1/12; for (const c of f) { x += v(c)*w; w /= 12; } return x; };
let worst = 0, max = [0, 0];
for (const l of fs.readFileSync(process.argv[2], 'utf8').split(/\r?\n/)) {
  const m = l.match(/^(\d+) (\S+) (\S+) (\S+) (\S+) (\S+)$/); if (!m || m[6] === '-') continue;
  const [p, q] = [dzI(m[3]), dzI(m[4])];
  let d = PI*q - p*S; if (d < 0n) d = -d;                     // |pi q - p| * 10^300
  const lam = -(lnBig(d) - Number(N)*Math.LN10 - lnBig(q)) / lnBig(q);
  const got = dzF(m[6]); worst = Math.max(worst, Math.abs(got - lam));
  if (got > max[1]) max = [m[1], got];
  if (+m[1] < 6 || got > 2.3) console.log(`n=${m[1]} q~10^${(lnBig(q)/Math.LN10).toFixed(1)} instr=${got.toFixed(6)} oracle=${lam.toFixed(6)}`);
}
console.log(`max |instr - oracle| = ${worst.toExponential(2)}; max lambda at n=${max[0]}: ${max[1].toFixed(4)}`);
