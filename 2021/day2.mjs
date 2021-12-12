(await import('./aoc.mjs')).default(
  2021, 2,
  (data) => {
    let h = 0, d = 0;
    for (const { dir, amt } of data) {
      switch (dir) {
        case 'forward': h += amt; break;
        case 'down': d += amt; break;
        case 'up': d -= amt; break;
      }
    }
    return h * d;
  }, 150,
  (data) => {
    let h = 0, d = 0, a = 0;
    for (const { dir, amt } of data) {
      switch (dir) {
        case 'forward':
          h += amt;
          d += a * amt;
          break;
        case 'down': a += amt; break;
        case 'up': a -= amt; break;
      }
    }
    return h * d;
  }, 900,
  data => data.map(inst => {
    let [dir, amt] = inst.split(' ');
    amt = parseInt(amt);
    return { dir, amt };
  })
);