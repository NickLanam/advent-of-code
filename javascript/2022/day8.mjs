(await import('./aoc.mjs')).default(
  2022, 8,
  (grid) => {
    let visible = 0;
    for (let r = 0; r < grid.length; r++) {
      const row = grid[r];
      cellLoop: for (let c = 0; c < row.length; c++) {
        if (r === 0 || r === grid.length - 1 || c === 0 || c === row.length - 1) {
          visible++;
          continue cellLoop;
        }
        const cell = row[c];
        const col = grid.map(row2 => row2[c]);
        const left = row.slice(0, c);
        const right = row.slice(c + 1);
        const up = col.slice(0, r);
        const down = col.slice(r + 1);
        if (
          left.every(e => e < cell)
          || right.every(e => e < cell)
          || up.every(e => e < cell)
          || down.every(e => e < cell)
        ) { visible++; }
      }
    }
    return visible;
  }, 21,
  (grid) => {
    let bestScore = -1;
    for (let r = 0; r < grid.length; r++) {
      const row = grid[r];
      for (let c = 0; c < row.length; c++) {
        const cell = row[c];
        const col = grid.map(row2 => row2[c]);
        const left = row.slice(0, c); left.reverse();
        const right = row.slice(c + 1);
        const up = col.slice(0, r); up.reverse();
        const down = col.slice(r + 1);

        let score = 1;
        for (const dir of [left, right, up, down]) {
          let local = 0;
          for (const c2 of dir) {
            local++;
            if (c2 >= cell) break;
          }
          score *= local;
        }
        if (score > bestScore) bestScore = score;
      }
    }
    return bestScore;
  }, 8,
  (data) => data.map(line => line.split('').map(n => +n)),
);