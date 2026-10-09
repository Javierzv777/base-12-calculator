// EXTERNAL ORACLE (not the instrument): mu(n), M(n), prime counts by class
// mod 12, for n <= N, in plain JS numbers. Prints decimal checkpoints and the
// race statistics, to cross-check mertens.sh.
const N = Number(process.argv[2] || 1728);
const mu = new Array(N + 1).fill(1), isP = new Array(N + 1).fill(true);
isP[0] = isP[1] = false;
for (let p = 2; p <= N; p++) if (isP[p]) {
  for (let m = 2 * p; m <= N; m += p) isP[m] = false;
  for (let m = p; m <= N; m += p) mu[m] = -mu[m];
  for (let m = p * p; m <= N; m += p * p) mu[m] = 0;
}
let M = 0, maxRatio = 0, at = 0; const cls = {1:0,5:0,7:0,11:0}; const lead = {5:0,7:0,11:0};
let sqfreeZero = 0, by4 = 0, by4or9 = 0, by4or25 = 0;
for (let n = 1; n <= N; n++) {
  M += mu[n];
  if (n >= 2 && M * M / n > maxRatio) { maxRatio = M * M / n; at = n; }
  if (isP[n] && n > 3) cls[n % 12]++;
  if (n >= 5) for (const r of [5,7,11]) if (cls[1] >= cls[r]) lead[r]++;
  if (mu[n] === 0) { sqfreeZero++; if (n % 4 === 0) by4++; if (n % 4 === 0 || n % 9 === 0) by4or9++; if (n % 4 === 0 || n % 25 === 0) by4or25++; }
  if ([10,100,144,1000,1728].includes(n)) console.log(`M(${n})=${M}`);
}
console.log(`max M^2/n for 2<=n<=${N}: ${maxRatio.toFixed(4)} at n=${at}`);
console.log(`primes>3 by class mod 12:`, cls);
console.log(`n in 5..${N} where class 1 is level or ahead of class r:`, lead);
console.log(`mu=0 count=${sqfreeZero}; decided by 4|n=${by4}; by 4|n or 9|n=${by4or9}; by 4|n or 25|n=${by4or25}`);
