// Control for PE4: the same run statistic on the first 107 decimal digits of pi
// (107 decimal digits ~ 99 dozenal digits of information). Decimal pi comes
// from Machin's formula in BigInt here: an external oracle, not the instrument.
const N = 130n, S = 10n**N;
const atanInv = x => { let t = S / x, s = t, k = 1n, x2 = x*x; while (t) { t /= x2; k += 2n; s += ((k-1n)/2n % 2n ? -1n : 1n) * (t / k); } return s; };
const pi = (16n*atanInv(5n) - 4n*atanInv(239n)).toString().slice(1, 108);   // drop the leading 3
const out = []; for (let i = 0; i < pi.length; ) { let j = i; while (j < pi.length && pi[j] === pi[i]) j++; if (j-i >= 2) out.push({d:pi[i], k:i, r:j-i}); i = j; }
console.log(`decimal: ${out.length} runs of length >= 2; longest r=${Math.max(...out.map(o=>o.r))}`);
const big = out.filter(o => o.k >= 10);
console.log(`max r/k, k>=10: any digit ${Math.max(...big.map(o=>o.r/o.k)).toFixed(3)}; digit 0 or 9 ${Math.max(...big.filter(o=>'09'.includes(o.d)).map(o=>o.r/o.k)).toFixed(3)}`);
for (const o of out.sort((a,b)=>b.r/b.k-a.r/a.k).slice(0,4)) console.log(`  ${o.d.repeat(o.r)} k=${o.k} r/k=${(o.r/Math.max(o.k,1)).toFixed(3)}`);
