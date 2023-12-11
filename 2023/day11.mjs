import aoc from './aoc.mjs';

const part1expected = 374;
const part2expected = 82000210;

const parse = (data) => data.map(l => l.split(''));

const solve = (grid, expansionMultiplier) => {
  // Find the expanding parts of the grid (rows and columns separately)
  const expandedRows = new Set();
  for (let y = grid.length - 1; y >= 0; y--) {
    if (grid[y].every(c => c === '.')) {
      expandedRows.add(y);
    }
  }
  const expandedCols = new Set();
  for (let x = grid[0].length - 1; x >= 0; x--) {
    if (grid.every(row => row[x] === '.')) {
      expandedCols.add(x);
    }
  }
  
  // Find the hashes
  const hashLocs = [];
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[0].length; x++) {
      if (grid[y][x] === '#') {
        hashLocs.push([x, y]);
      }
    }
  }

  // Find the shortest path between each, accounting for expansion when crossing it
  const distances = [];
  for (let i = 0; i < hashLocs.length - 1; i++) {
    const left = hashLocs[i];
    for (let j = i + 1; j < hashLocs.length; j++) {
      const right = hashLocs[j];
      let dx = 0;
      let dy = 0;
      for (let x = Math.min(left[0], right[0]) + 1; x <= Math.max(left[0], right[0]); x++) {
        dx += expandedCols.has(x) ? expansionMultiplier : 1;
      }
      for (let y = Math.min(left[1], right[1]) + 1; y <= Math.max(left[1], right[1]); y++) {
        dy += expandedRows.has(y) ? expansionMultiplier : 1;
      }
      distances.push(dx + dy);
    }
  }

  // Sum 'em up
  return distances.reduce((a, c) => a + c, 0);
}

const part1 = (grid) => solve(grid, 2); 
const part2 = (grid) => solve(grid, 1_000_000);

aoc(2023, 11, part1, part1expected, part2, part2expected, parse);
