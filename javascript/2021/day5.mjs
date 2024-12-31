function solve(steps, withDiagonals = false) {
  const grid = Array(1000).fill(0).map(_ => Array(1000).fill(0));
  for (const [x1, y1, x2, y2] of steps) {
    if (x1 !== x2 && y1 !== y2 && !withDiagonals) continue;

    let x = x1, y = y1;
    let xd = x1 < x2 ? 1 : -1;
    let yd = y1 < y2 ? 1 : -1;
    grid[y][x]++;
    while (x !== x2 || y !== y2) {
      if (x !== x2) x += xd;
      if (y !== y2) y += yd;
      grid[y][x]++;
    }
  }
  return grid.flat().filter(n => n > 1).length;
}

(await import('./aoc.mjs')).default(
  2021, 5,
  (data) => solve(data, false), 5,
  (data) => solve(data, true), 12,
  data => data.map(
    line => line.match(/^(\d+),(\d+) -> (\d+),(\d+)$/).map(n => +n).slice(1))
);