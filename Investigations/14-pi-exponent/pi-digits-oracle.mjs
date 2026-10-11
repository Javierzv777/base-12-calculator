// External oracle: how many leading dozenal digits of an instrument pi file
// (piscale*.out) are correct, against Machin's formula in BigInt.
// Usage: node pi-digits-oracle.mjs FILE
import fs from 'fs';
const got = fs.readFileSync(process.argv[2], 'utf8').replace(/^(\(\d+ s\) )?~3;/, '').trim().split(/\s/)[0];
const N = BigInt(Math.ceil(got.length * 1.08 + 20)), S = 10n**N;
const atanInv = x => { let t = S / x, s = t, k = 1n, x2 = x*x; while (t) { t /= x2; k += 2n; s += ((k-1n)/2n % 2n ? -1n : 1n) * (t / k); } return s; };
let f = 16n*atanInv(5n) - 4n*atanInv(239n) - 3n*S, ref = '';
for (let i = 0; i < got.length; i++) { f *= 12n; const d = f / S; f -= d*S; ref += 'XE'[Number(d) - 10] ?? String(d); }
let ok = 0; while (ok < got.length && got[ok] === ref[ok]) ok++;
console.log(`${got.length} digits printed, ${ok} correct${ok < got.length ? `; digit ${ok+1}: got ${got[ok]}, reference ${ref[ok]}` : ''}`);
