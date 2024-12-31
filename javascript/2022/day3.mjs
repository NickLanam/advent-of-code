const score = (c) => (c.charCodeAt(0) <= 'Z'.charCodeAt(0))
  ? (c.charCodeAt(0) - 'A'.charCodeAt(0) + 27)
  : (c.charCodeAt(0) - 'a'.charCodeAt(0) + 1);

(await import('./aoc.mjs')).default(
  2022, 3,
  (data) => {
    let sum = 0;
    for (const line of data) {
      const left = line.slice(0, line.length / 2);
      const right = line.slice(line.length / 2);
      const found = left.split('').filter(c => right.includes(c))[0];
      sum += score(found);
    }
    return sum;
  }, 157,
  (data) => {
    let sum = 0;
    while (data.length) {
      const [a, b, c] = [data.shift(), data.shift(), data.shift()];
      const found = a.split('').filter(ch => b.includes(ch) && c.includes(ch))[0];
      sum += score(found);
    }
    return sum;
  }, 70,
  data => data
);