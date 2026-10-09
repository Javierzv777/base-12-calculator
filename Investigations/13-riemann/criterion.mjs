// EXTERNAL ORACLE: checks, for 2k <= N, that
//   zeta(1-2k) terminates in base 12  <=>  no prime p >= 5 has (p-1) | 2k
// and that in that case the reduced denominator is exactly 12; also that
// 3 always divides the denominator (so base 10 never terminates).
const gcd=(a,b)=>{a=a<0n?-a:a;b=b<0n?-b:b;while(b)[a,b]=[b,a%b];return a;};
const fr=(n,d)=>{if(d<0n){n=-n;d=-d;}const g=gcd(n,d)||1n;return[n/g,d/g];};
const N=Number(process.argv[2]||400); const A=[]; const B=[];
for(let m=0;m<=N;m++){A[m]=fr(1n,BigInt(m+1));for(let j=m;j>=1;j--){const[a,b]=A[j-1],[c,d]=A[j];A[j-1]=fr((a*d-c*b)*BigInt(j),b*d);}B[m]=A[0];}
const isPrime=n=>{if(n<2)return false;for(let p=2;p*p<=n;p++)if(n%p===0)return false;return true;};
let bad=0,count=0,always3=true; const ks=[];
for(let m=2;m<=N;m+=2){const[z0,d]=fr(-B[m][0],B[m][1]*BigInt(m)); let dd=d; for(const p of[2n,3n])while(dd%p===0n)dd/=p;
 const term=dd===1n; let crit=true; for(let q=1;q<=m;q++) if(m%q===0&&isPrime(q+1)&&q+1>=5){crit=false;break;}
 if(d%3n!==0n) always3=false; if(term!==crit||(term&&d!==12n))bad++; if(term){count++;ks.push(m);} }
console.log(`N=${N} even indices=${N/2} terminating=${count} mismatches=${bad} 3|den always=${always3}`);
console.log(ks.join(','));
