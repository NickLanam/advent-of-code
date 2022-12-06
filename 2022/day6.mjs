function solve(data, length) {
  for (let i = length; i < data.length; i++) {
    if (new Set(data.slice(i - length, i)).size === length) {
      return i;
    }
  }
}

(await import('./aoc.mjs')).default(
  2022, 6,
  (data) => solve(data, 4), 7,
  (data) => solve(data, 14), 19,
  data => data[0].split('')
);