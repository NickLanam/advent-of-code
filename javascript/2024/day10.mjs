import aoc from './aoc.mjs';
import { neighbors } from './data-structures/grid2d.mjs';

/** @typedef {{ route: [x: number, y: number][], seen: Set<number>, done: boolean, nextVal: number }[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 36;

/** @type Part2Solution */
const part2expected = 81;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return makePaths(lines.map(l => l.split('').map(n => +n)));
};

const toKey = (x, y) => x*100 + y;

function makePaths(grid) {
  let paths = [];
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
      if (grid[y][x] === 0) {
        paths.push({ route: [[x, y]], seen: new Set([toKey(x, y)]), done: false, nextVal: 1 });
      }
    }
  }
  do {
    paths = paths.filter(p => !p.done || p.nextVal > 9);
    const unfinished = paths.filter(p => !p.done);
    if (!unfinished.length) break;
    for (const path of unfinished) {
      const log = path.seen.has(6) && path.seen.has(106) ? console.log : () => {};
      // We'll clone it if it has neighbors that keep going
      path.done = true;
      if (path.nextVal > 9) continue;
      const [sx, sy] = path.route[path.route.length - 1];
      let next = neighbors(grid, sx, sy, false);
      next = next.filter(n => n.v === path.nextVal);
      if (!next.length) {
        continue;
      }
      for (const n of next) {
        const nk = toKey(n.x, n.y);
        if (!path.seen.has(nk)) {
          path.seen.add(nk);
          paths.push({ route: [...path.route, [n.x, n.y]], seen: new Set(path.seen), done: n.v >= 9, nextVal: path.nextVal + 1 });
        }
      }
    }
  } while (true);
  
  paths = paths.filter(p => p.done && p.nextVal > 9);
  return paths;
}

/**
 * 
 * @param {ParsedInput} grid 
 * @returns {Part1Solution}
 */
const part1 = (paths) => {
  const grouped = paths.reduce((a, c) => {
    const k = toKey(...c.route[0]);
    a[k] ??= new Set();
    a[k].add(toKey(...c.route[c.route.length - 1]));
    return a;
  }, {});
  return (Object.values(grouped).map(s => s.size)).reduce((a, c) => a + c, 0);
};

/**
 * 
 * @param {ParsedInput} grid 
 * @returns {Part2Solution}
 */
const part2 = (paths) => {
  const grouped = paths.reduce((a, c) => {
    const k = toKey(...c.route[0]);
    a[k] ??= new Set();
    a[k].add(JSON.stringify(c.route));
    return a;
  }, {});
  return (Object.values(grouped).map(s => s.size)).reduce((a, c) => a + c, 0);
};

aoc({
  year: 2024,
  day: 10,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
