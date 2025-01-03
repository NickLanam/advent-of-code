function sim(data, LIM) {
  let buckets = Array(9).fill(0);
  for (const f of data) buckets[f]++;
  for (let d = 0; d < LIM; d++) {
    buckets = [buckets[1], buckets[2], buckets[3], buckets[4], buckets[5], buckets[6], buckets[7] + buckets[0], buckets[8], buckets[0]];
  }
  return buckets.reduce((a, c) => a + c, 0);
}

(await import('./aoc.mjs')).default(
  2021, 6,
  (data) => sim(data, 80), 5934,
  (data) => sim(data, 256), 26984457539, // Aaaaand this breaks the brute force solution.
  data => data[0].split(',').map(n => +n)
);