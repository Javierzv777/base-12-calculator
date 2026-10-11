// PE4: runs of a repeated digit in the instrument's dozenal pi. A run of
// length r starting right after fractional digit k gives a rational with
// denominator 12^k (digit 0 or E) or 11*12^k (any digit) within ~12^-(k+r),
// i.e. local exponent 1 + r/k. Reports max r/k for k >= 10.
// Usage: node runs.mjs [FILE] [DIGITS]   (default pi-scale100.out, 99 digits)
import fs from 'fs';
const s = fs.readFileSync(process.argv[2] ?? 'pi-scale100.out','utf8').replace(/^(\(\d+ s\) )?~3;/,'').trim().slice(0, +(process.argv[3] ?? 99));
const out = [];
for (let i = 0; i < s.length; ) { let j = i; while (j < s.length && s[j] === s[i]) j++; if (j-i >= 2) out.push({d:s[i], k:i, r:j-i}); i = j; }
out.sort((a,b) => b.r - a.r || a.k - b.k);
console.log(`dozenal, ${s.length} digits: ${out.length} runs of length >= 2 (expected ~${((s.length-1)/12).toFixed(1)}); longest r=${out[0]?.r}`);
for (const o of out.slice(0,6)) console.log(`  ${o.d.repeat(o.r)} k=${o.k} r=${o.r} r/k=${(o.r/Math.max(o.k,1)).toFixed(3)}`);
const big = out.filter(o => o.k >= 10);
const m = (f) => Math.max(...big.filter(f).map(o => o.r/o.k));
console.log(`max r/k, k>=10: any digit ${m(()=>true).toFixed(3)}; digit 0 or E ${m(o=>'0E'.includes(o.d)).toFixed(3)}`);
