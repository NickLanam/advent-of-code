(await import('./aoc.mjs')).default(
  2022, 4,
  (data) => {
    let dupes = 0;
    for (const [[ll, lr], [rl, rr]] of data) {
      if ((ll >= rl && lr <= rr) || (rl >= ll && rr <= lr)) dupes++;
    }
    return dupes;
  }, 2,
  (data) => {
    let dupes = 0;
    for (const [[ll, lr], [rl, rr]] of data) {
      for (let x = ll; x <= lr; x++) {
        if (x >= rl && x <= rr) {
          dupes++;
          break;
        }
      }
    }
    return dupes;
  }, 4,
  (data) => {
    let pairs = [];
    for (const line of data) {
      const [l, r] = line.split(',');
      const [ll, lr] = l.split('-').map(n => +n);
      const [rl, rr] = r.split('-').map(n => +n);
      pairs.push([[ll, lr], [rl, rr]]);
    }
    return pairs;
  }
);