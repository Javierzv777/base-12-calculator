// Control for PE4: the same run statistic on the first decimal digits of pi,
// as many as carry the same information as the dozenal digits used
// (DIGITS * log 12 / log 10: 99 -> 107, 399 -> 431). Decimal pi comes from
// Machin's formula in BigInt here: an external oracle, not the instrument.
// Usage: node runs-base10.mjs [DOZENAL_DIGITS]   (default 99)
const D = Math.round(+(process.argv[2] ?? 99) * Math.log(12) / Math.LN10);
const N = BigInt(D + 30), S = 10n**N;
const atanInv = x => { let t = S / x, s = t, k = 1n, x2 = x*x; while (t) { t /= x2; k += 2n; s += ((k-1n)/2n % 2n ? -1n : 1n) * (t / k); } return s; };
const pi = (16n*atanInv(5n) - 4n*atanInv(239n)).toString().slice(1, D + 1);   // drop the leading 3
const out = []; for (let i = 0; i < pi.length; ) { let j = i; while (j < pi.length && pi[j] === pi[i]) j++; if (j-i >= 2) out.push({d:pi[i], k:i, r:j-i}); i = j; }
console.log(`decimal, ${pi.length} digits: ${out.length} runs of length >= 2 (expected ~${((pi.length-1)/10).toFixed(1)}); longest r=${Math.max(...out.map(o=>o.r))}`);
const big = out.filter(o => o.k >= 10);
console.log(`max r/k, k>=10: any digit ${Math.max(...big.map(o=>o.r/o.k)).toFixed(3)}; digit 0 or 9 ${Math.max(...big.filter(o=>'09'.includes(o.d)).map(o=>o.r/o.k)).toFixed(3)}`);
for (const o of out.sort((a,b)=>b.r/b.k-a.r/a.k).slice(0,4)) console.log(`  ${o.d.repeat(o.r)} k=${o.k} r/k=${(o.r/Math.max(o.k,1)).toFixed(3)}`);
