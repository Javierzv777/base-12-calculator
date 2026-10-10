// EXTERNAL ORACLE check of mertens-1728.out: every row's n, kind, mu and M
// against a plain sieve; plus race and mu=0 statistics read from the
// instrument's own rows (the class is the last dozenal digit it printed).
import fs from "fs";
const L = fs.readFileSync(process.argv[2] || "mertens-1728.out", "utf8").trim().split("\n");
const v = c => "0123456789XE".indexOf(c.replace("↊", "X").replace("↋", "E"));
const dz = s => { const neg = s[0] == "-"; if (neg) s = s.slice(1); let x = 0; for (const c of s) x = x * 12 + v(c); return neg ? -x : x; };
const N = Number(process.argv[3] || 1728), mu = new Array(N + 1).fill(1), isP = new Array(N + 1).fill(true); isP[0] = isP[1] = false;
for (let p = 2; p <= N; p++) if (isP[p]) { for (let m = 2 * p; m <= N; m += p) isP[m] = false; for (let m = p; m <= N; m += p) mu[m] = -mu[m]; for (let m = p * p; m <= N; m += p * p) mu[m] = 0; }
let M = 1, bad = 0, maxR = 0, maxRat = "", maxAbs = 0, maxAt = "", zero = 0, z1 = 0, z2 = 0;
const cls = { 1: 0, 5: 0, 7: 0, E: 0 }, ahead = { 5: 0, 7: 0, E: 0 }, tie = { 5: 0, 7: 0, E: 0 };
for (let i = 0; i < L.length; i++) {
  const f = Object.fromEntries(L[i].split(" ").map(t => t.split("=")));
  const n = dz(f.n); M += mu[n]; const Mi = dz(f.M);
  if (n !== i + 2 || +f.mu !== mu[n] || Mi !== M || (f.kind == "prime") !== isP[n] || f.mertens_ok !== "1") bad++;
  if (f.kind == "prime" && n > 3) cls[f.last]++;
  if (n >= 5) for (const r of ["5", "7", "E"]) { if (cls[1] > cls[r]) ahead[r]++; else if (cls[1] == cls[r]) tie[r]++; }
  if (Math.abs(Mi) > maxAbs) { maxAbs = Math.abs(Mi); maxAt = f.n; }
  if (Mi * Mi / n > maxR) { maxR = Mi * Mi / n; maxRat = f.n; }
  if (f.mu == "0") { zero++; if ("048".includes(f.last)) z1++; if ("048".includes(f.last) || dz(f.n.slice(-2)) % 9 == 0) z2++; }
}
console.log({ rows: L.length, mismatches: bad, last: L.at(-1), cls, ahead, tie, maxAbs, maxAt, maxM2overN: maxR.toFixed(4), at: maxRat, mu0: zero, mu0_lastDigit: z1, mu0_last2Digits: z2 });
