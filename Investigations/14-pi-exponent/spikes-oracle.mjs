// External oracle (node, BigInt-reduced argument) for the spike terms.
import fs from 'fs';
const PI = 314159265358979323846264338327950288419716939937510n, SC = 10n**50n;
const dz = s => { s = s.replace('~',''); const [i,f=''] = s.split(';'); const v = c => 'XE'.includes(c) ? 10+'XE'.indexOf(c) : +c;
  let x = 0; for (const c of i) x = x*12 + v(c); let w = 1/12; for (const c of f) { x += v(c)*w; w /= 12; } return x; };
for (const line of fs.readFileSync(process.argv[2],'utf8').trim().split('\n')) {
  const p = BigInt(line.match(/^p=(\d+)/)[1]); const t = dz(line.match(/term=(\S+)/)[1]);
  let k = (p*SC*2n + PI) / (2n*PI); const r = Number(p*SC - k*PI) / 1e50;   // p - k*pi
  const o = 1/(Number(p)**3 * Math.sin(r)**2);
  console.log(`p=${p} instr=${t.toPrecision(10)} oracle=${o.toPrecision(10)} relerr=${Math.abs(t-o)/o}`);
}
