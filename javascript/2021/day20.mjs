// Longer code to be as tight as possible at run time, since this has to be invoked hundreds of thousands of times.
function pointToBin(grid, x, y, outer = 0) {
  let out = 0;
  let off = 8;
  for (let y2 = y - 1; y2 < y + 2; y2++) {
    const row = grid[y2];
    for (let x2 = x - 1; x2 < x + 2; x2++) {
      out += (row?.[x2] ?? outer) << off;
      off--;
    }
  }
  return out;
}

function printGrid(grid) {
  for (let r = 0; r < grid.length; r++) {
    console.log(String(r).padStart(3, ' ') + ' ' + grid[r].map(c => c ? '#' : '.').join(''));
  }
}

// Outer: the sample has alg[0] = 0 and alg[-1] = 1. But real input can do the opposite, so we need to know what the rest of the infinite grid is up to.
function enhance(grid, alg, outer = 0) {
  const out = Array.from({ length: grid.length }, () => Array.from({ length: grid[0].length }, () => 0));
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[0].length; x++) {
      out[y][x] = alg[pointToBin(grid, x, y, outer)];
    }
  }
  return out;
}

function enhanceLoop(times, gridIn, alg) {
  let grid = gridIn;
  for (let i = 0; i < times; i++) {
    let outer;
    if (alg[0] === 1 && alg[alg.length - 1] === 0) {
      outer = i % 2;
    } else if (alg[0] === alg[alg.length- 1]) {
      outer = alg[0];
    } else {
      outer = 0;
    }
    grid = enhance(grid, alg, outer);
  }
  const numLights = grid.reduce((a, row) => a + row.reduce((b, c) => b + c, 0), 0);
  return numLights;
}

(await import('./aoc.mjs')).default(
  2021, 20,
  ({ grid, alg }) => enhanceLoop(2, grid, alg), 35,
  ({ grid, alg }) => enhanceLoop(50, grid, alg), 3351,
  data => {
    const alg = data[0].split('').map(c => c === '#' ? 1 : 0);
    const lines = data.slice(2);

    const grid = Array.from({ length: lines.length + 150 }, (_, r) => (
      Array.from({ length: lines[0].length + 150 }, (_, c) => lines[r - 75]?.[c - 75] === '#' ? 1 : 0)
    ));

    return { grid, alg };
  }
);