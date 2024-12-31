function neighborCoords(grid, x, y) {
  const possible = [
    [x - 1, y - 1], [x, y - 1], [x + 1, y - 1], [x - 1, y], [x + 1, y], [x - 1, y + 1], [x, y + 1], [x + 1, y + 1]
  ];
  return possible.filter(([x2, y2]) => grid[y2]?.[x2] != null);
}

function flash(grid, x, y, alreadyFlashed) {
  if (alreadyFlashed.has(`${x},${y}`)) {
    return;
  }
  alreadyFlashed.add(`${x},${y}`);
  const neighbors = neighborCoords(grid, x, y);
  for (const [x2, y2] of neighbors) {
    grid[y2][x2]++;
  }
}

function step(grid) {
  let numFlashed = 0;
  // First, increase every value.
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
      grid[y][x]++;
    }
  }
  // As long as there are flashes pending, perform them and remember how many flashes there were.
  let flashedThisStep = new Set();
  while (true) {
    const ready = grid
      .map((_, y) => grid[y]
        .map((v, x) => v >= 10 ? [x, y] : null))
      .flat()
      .filter(c => c)
      .filter(([rx, ry]) => !flashedThisStep.has(`${rx},${ry}`));
    if (!ready.length) break;
    for (const [x, y] of ready) {
      flash(grid, x, y, flashedThisStep);
    }
  }
  numFlashed += flashedThisStep.size;
  flashedThisStep.forEach((coord) => {
    const [x, y] = coord.split(',');
    grid[y][x] = 0;
  })
  return numFlashed;
}

(await import('./aoc.mjs')).default(
  2021, 11,
  (grid) => (Array(100).fill(0).reduce(a => a + step(grid), 0)), 1656,
  (grid) => {
    for (let stepsTaken = 1; stepsTaken < 10_000 /* Safety limit */; stepsTaken++) {
      const numFlashed = step(grid);
      if (numFlashed === grid.length * grid[0].length) {
        return stepsTaken;
      }
    }
    return 'ERROR: Loop ran 10,000 iterations without finding synchronization';
  }, 195,
  data => data.map(l => l.split('').map(n => +n))
);