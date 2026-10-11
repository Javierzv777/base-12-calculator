// External oracle (node, double precision with BigInt argument reduction) for
// the partial sums of flint-hills-*.out. Usage: node flint-hills-oracle.mjs FILE
import fs from 'fs';
const PI = 314159265358979323846264338327950288419716939937510n, SC = 10n**50n;
const dz = s => { const neg = s.startsWith('-'); s = s.replace(/[~-]/g,''); const [i,f=''] = s.split(';');
  const v = c => 'XE'.includes(c) ? 10+'XE'.indexOf(c) : +c; let x = 0; for (const c of i) x = x*12+v(c);
  let w = 1/12; for (const c of f) { x += v(c)*w; w /= 12; } return neg ? -x : x; };
let S = 0, worst = 0, at = 0;
for (const line of fs.readFileSync(process.argv[2],'utf8').trim().split('\n')) {
  const n = BigInt(line.match(/^n=(\d+)/)[1]);
  const k = (n*SC*2n + PI) / (2n*PI); const r = Number(n*SC - k*PI) / 1e50;
  S += 1/(Number(n)**3 * Math.sin(r)**2);
  const e = Math.abs(dz(line.match(/sum=(\S+)/)[1]) - S) / S; if (e > worst) { worst = e; at = n; }
  if ([100n,355n,400n,1000n,2000n].includes(n)) console.log(`n=${n} instr=${dz(line.match(/sum=(\S+)/)[1]).toPrecision(12)} oracle=${S.toPrecision(12)}`);
}
console.log(`worst relative error ${worst.toExponential(2)} at n=${at}`);
