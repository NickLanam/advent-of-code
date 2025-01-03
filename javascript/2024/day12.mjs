import aoc from './aoc.mjs';
import { neighbors } from './data-structures/grid2d.mjs';

/** @typedef {string[][]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 1930;

/** @type Part2Solution */
const part2expected = 1206;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return lines.map(l => l.split(''));
};

const toKey = (x, y, d) => (x << 8) + y + (d != null ? '_' + d : 0);

const buildRegions = (grid) => {
  /** @type Set<number>[] */
  const regions = [];
  const seen = new Set();
  const explore = (x, y) => {
    const v = grid[y][x];
    const k = toKey(x, y);
    if (seen.has(k)) return;
    seen.add(k);
    let match = regions.find(r => r.has(k));
    if (!match) {
      match = new Set();
      match.add(k);
      regions.push(match);
    }
    const nexts = neighbors(grid, x, y, false).filter(({x: nx, y: ny, v: nv}) => v === nv && !match.has(toKey(nx, ny)));
    for (const next of nexts) {
      match.add(toKey(next.x, next.y));
    }
    for (const next of nexts) {
      explore(next.x, next.y);
    }
  };
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
      explore(x, y);
    }
  }
  return regions;
};

/**
 * 
 * @param {ParsedInput} grid 
 * @returns {Part1Solution}
 */
const part1 = (grid) => {
  const regions = buildRegions(grid);
  
  const w = grid[0].length;
  const h = grid.length;
  let out = 0;
  for (const region of regions) {
    let perimeter = 0;
    let area = region.size;
    for (const key of region) {
      const y = key % 256;
      const x = (key - y) >> 8;
      if (x === 0) perimeter++;
      if (x === w - 1) perimeter++;
      if (y === 0) perimeter++;
      if (y === h - 1) perimeter++;
      const mismatchedNeighbors = neighbors(grid, x, y, false).filter(({v}) => v !== grid[y][x]);
      perimeter += mismatchedNeighbors.length;
    }
    out += area * perimeter;
  }
  return out;
};

/**
 * 
 * @param {ParsedInput} grid 
 * @returns {Part2Solution}
 */
const part2 = (grid) => {
  const regions = buildRegions(grid);
  const w = grid[0].length;
  const h = grid.length;

  let out = 0;
  for (const region of regions) {
    const touchedPerimeter = new Set();
    let perimeter = 0;
    let area = region.size;

    const updatePerimeter = (px, py, pd, rv) => {
      const pk = toKey(px, py, pd);
      if (touchedPerimeter.has(pk)) return;
      touchedPerimeter.add(pk);
      switch (pd) {
        case 'N':
        case 'S': {
          const e = touchedPerimeter.has(toKey(px + 1, py, pd));
          const w = touchedPerimeter.has(toKey(px - 1, py, pd));
          if (!e && !w) perimeter++;
          if (e && w) perimeter--; // Fix for meeting in middle
          break;
        }
        case 'E':
        case 'W': {
          const n = touchedPerimeter.has(toKey(px, py - 1, pd));
          const s = touchedPerimeter.has(toKey(px, py + 1, pd));
          if (!n && !s) perimeter++;
          if (n && s) perimeter--; // Fix for meeting in middle
          break;
        }
      }
    };

    for (const key of region) {
      const y = key % 256;
      const x = (key - y) >> 8;
      const v = grid[y][x];

      if (x === 0) updatePerimeter(x - 1, y, 'W', v);
      if (x === w - 1) updatePerimeter(x + 1, y, 'E', v);
      if (y === 0) updatePerimeter(x, y - 1, 'N', v);
      if (y === h - 1) updatePerimeter(x, y + 1, 'S', v);
      for (const n of neighbors(grid, x, y, false).filter(({v}) => v !== grid[y][x])) {
        updatePerimeter(n.x, n.y, n.dir, v);
      }
    }

    out += area * perimeter;
  }
  return out;
};

aoc({
  year: 2024,
  day: 12,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
