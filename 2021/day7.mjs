const solve = (d, f) => Math.min(...Array(d[d.length - 1]).fill(0).map((_, i) => d.reduce((a, c) => a + f(Math.abs(c - i)), 0)));

(await import('./aoc.mjs')).default(
  2021, 7,
  data => solve(data, x => x), 37,
  data => solve(data, x => (x * (x + 1)) / 2), 168,
  data => data[0].split(',').map(n => +n).sort()
);