// Recursion may be lazy and problematic, but the input is small enough that it works.
// My first attempt was iterative and worked on the sample, but was wrong for the real input.
// I didn't feel like tracking down why when the recursive solution was easier anyway.
const flood = (grid, x, y, basin) => {
  const v = grid[y]?.[x] ?? NaN;
  const id = `${x};${y};${v}`;
  if (Number.isNaN(v) || v === 9 || basin.has(id)) return basin;
  basin.add(id);
  for (const [x2, y2] of [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]]) {
    flood(grid, x2, y2, basin);
  }
  return basin;
}

(await import('./aoc.mjs')).default(
  2021, 9,
  (grid) => (
    grid.reduce((out, row, y) => {
      out.push(...row.filter((v, x) => (
        v < 9 && [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]].every(([x2, y2]) => ((grid[y2]?.[x2] ?? Infinity) > v))
      )));
      return out;
    }, []).reduce((a, c) => a + c + 1, 0)
  ), 15,
  (grid) => {
    const basins = [];
    for (let y = 0; y < grid.length; y++) {
      for (let x = 0; x < grid[y].length; x++) {
        const id = `${x};${y};${grid[y][x]}`;
        if (grid[y][x] === 9 || basins.some(b => b.has(id))) continue;
        basins.push(flood(grid, x, y, new Set()));
      }
    }
    return basins.map(b => b.size).sort((a, b) => a-b).slice(-3).reduce((a, b) => a * b, 1);
  }, 1134,
  lines => lines.map(l => l.split('').map(n => +n))
);