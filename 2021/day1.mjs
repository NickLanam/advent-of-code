const f = (d, n) => d.slice(n).reduce((a, c, i) => a + (+c > +d[i]), 0);
(await import('./aoc.mjs')).default(2021, 1, d => f(d, 1), 7, d => f(d, 3), 5);