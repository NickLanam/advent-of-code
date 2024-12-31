(await import('./aoc.mjs')).default(
  2022, 4,
  (data) => data.filter(([[ll, lr], [rl, rr]]) => (
    (ll >= rl && lr <= rr)
    || (rl >= ll && rr <= lr)
  )).length, 2,
  (data) => data.filter(([[ll, lr], [rl, rr]]) => (
    (ll >= rl && ll <= rr)
    || (lr >= rl && lr <= rr)
    || (rl >= ll && rl <= lr)
    || (rr >= ll && rr <= lr)
  )).length, 4,
  data => data.map(line => line.split(',').map(half => half.split('-').map(n => +n)))
);