import aoc from './aoc.mjs';
import { neighbors } from './data-structures/grid2d.mjs';

/** @typedef {string[][]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 18;

/** @type Part2Solution */
const part2expected = 9;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return lines.map(l => l.split(''));
};

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
        const Ms = neighbors(grid, sx, sy).filter(({v}) => v === 'M');
        for (const M of Ms) {
          const As = neighbors(grid, M.x, M.y).filter(({dir, v}) => dir === M.dir && v === 'A');
          for (const A of As) {
            const Ss = neighbors(grid, A.x, A.y).filter(({dir, v}) => dir === A.dir && v === 'S');
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
        const Ms = neighbors(grid, sx, sy).filter(({v, dir}) => v === 'M' && dir.length === 2);
        const Ss = neighbors(grid, sx, sy).filter(({v, dir}) => v === 'S' && dir.length === 2);
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
