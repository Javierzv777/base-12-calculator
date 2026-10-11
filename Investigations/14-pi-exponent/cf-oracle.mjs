// External oracle for P1: continued fraction of pi from Machin's formula in
// BigInt (600 decimal digits, well past 399 dozenal ~ 431 decimal), compared
// quotient by quotient with the instrument's certified CF. Usage: node cf-oracle.mjs FILE
import fs from 'fs';
const N = 600n, S = 10n**N;
const atanInv = x => { let t = S / x, s = t, k = 1n, x2 = x*x; while (t) { t /= x2; k += 2n; s += ((k-1n)/2n % 2n ? -1n : 1n) * (t / k); } return s; };
let num = 16n*atanInv(5n) - 4n*atanInv(239n), den = S;
const ref = []; for (let i = 0; i < 600; i++) { const a = num / den; ref.push(a); [num, den] = [den, num - a*den]; }
const dz = s => [...s].reduce((x, c) => x*12n + BigInt('X↊'.includes(c) ? 10 : 'E↋'.includes(c) ? 11 : +c), 0n);
const got = fs.readFileSync(process.argv[2], 'utf8').split(/\r?\n/).filter(l => /^a\d+=/.test(l)).map(l => dz(l.split('=')[1]));
const bad = got.map((a, i) => a === ref[i] ? null : i).filter(i => i !== null);
console.log(`${got.length} certified quotients, ${bad.length} mismatches${bad.length ? ' at ' + bad : ''}`);
console.log(`next reference quotients: ${ref.slice(got.length, got.length + 3).join(', ')}`);
console.log(`large ones (decimal): ${got.map((a,i)=>[i,a]).filter(([,a])=>a>=20n).map(([i,a])=>`a${i}=${a}`).join(' ')}`);
