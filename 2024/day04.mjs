import aoc from './aoc.mjs';

/** @typedef {string[][]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 18;

/** @type Part2Solution */
const part2expected = 9;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  return lines.map(l => l.split(''));
};

/**
 * 
 * @param {string[][]} grid 
 * @param {number} x 
 * @param {number} y 
 */
function vecNeighbors(grid, x, y) {
  const w = grid[0].length;
  const h = grid.length;
  return [
    [x - 1, y - 1, 'NW'], [x, y - 1, 'N'], [x + 1, y - 1, 'NE'],
    [x - 1, y, 'W'],                       [x + 1, y, 'E'],
    [x - 1, y + 1, 'SW'], [x, y + 1, 'S'], [x + 1, y + 1, 'SE']
  ].filter(([x2, y2]) => x2 >= 0 && x2 < w && y2 >= 0 && y2 < h)
  .map(([x3, y3, dir]) => ({ x: x3, y: y3, dir, v: grid[y3][x3]}));
}

/**
 * 
 * @param {ParsedInput} grid 
 * @returns {Part1Solution}
 */
const part1 = (grid) => {
  let found = 0;
  for (let sy = 0; sy < grid.length; sy++) {
    for (let sx = 0; sx < grid[sy].length; sx++) {
      const sv = grid[sy][sx];
      if (sv === 'X') {
        const Ms = vecNeighbors(grid, sx, sy).filter(({v}) => v === 'M');
        for (const M of Ms) {
          const As = vecNeighbors(grid, M.x, M.y).filter(({dir, v}) => dir === M.dir && v === 'A');
          for (const A of As) {
            const Ss = vecNeighbors(grid, A.x, A.y).filter(({dir, v}) => dir === A.dir && v === 'S');
            found += Ss.length;
          }
        }
      }
    }
  }
  return found;
};

/**
 * 
 * @param {ParsedInput} grid 
 * @returns {Part2Solution}
 */
const part2 = (grid) => {
  let found = 0;
  for (let sy = 0; sy < grid.length; sy++) {
    for (let sx = 0; sx < grid[sy].length; sx++) {
      const sv = grid[sy][sx];
      if (sv === 'A') {
        const Ms = vecNeighbors(grid, sx, sy).filter(({v, dir}) => v === 'M' && dir.length === 2);
        const Ss = vecNeighbors(grid, sx, sy).filter(({v, dir}) => v === 'S' && dir.length === 2);
        const possible = Ms.length === 2 && Ss.length === 2;
        if (possible) {
          const MAM = !Ms[0].dir.split('').includes(Ms[1].dir.split('')[0]) && !Ms[0].dir.split('').includes(Ms[1].dir.split('')[1]);
          if (!MAM) found++;
        }
      }
    }
  }
  return found;
};

aoc({
  year: 2024,
  day: 4,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
