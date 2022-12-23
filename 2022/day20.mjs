const DECRYPTION_KEY = 811589153;

function solve(orig, numMixes) {
  const data = [...orig];
  const l = data.length;
  for (let m = 0; m < numMixes; m++) {
    for (let id = 0; id < l; id++) {
      const i = data.findIndex(({ id: d }) => d === id);
      const n = data[i].n;
      const j = (i + n < 0) ? 0 - ((0 - i - n) % (l - 1)) : (i + n) % (l - 1);
      data.splice(i, 1);
      data.splice(j, 0, { id, n });
    }
  }
  const io0 = data.findIndex(({ n }) => n === 0);
  const c = (n) => data[(io0 + n) % data.length].n;
  return c(1000) + c(2000) + c(3000);
}

(await import('./aoc.mjs')).default(
  2022, 20,
  (orig) => solve(orig, 1), 92,
  (orig) => solve(orig.map(({ id, n }) => ({ id, n: n * DECRYPTION_KEY })), 10), 8927480683,
  data => data.map((n, id) => ({ id: +id, n: +n }))
);