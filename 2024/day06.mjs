import aoc from './aoc.mjs';
import { neighbors } from './data-structures/grid2d.mjs';

/** @typedef {{ w: number, h: number, gx: number, gy: number, obstacles: Set<string> }} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 41;

/** @type Part2Solution */
const part2expected = 6;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  const h = lines.length;
  const w = lines[0].length;
  let gx, gy;
  let obstacles = new Set();
  for (let y = 0; y < lines.length; y++) {
    const line = lines[y];
    for (let x = 0; x < line.length; x++) {
      if (line[x] === '^') {
        gx = x;
        gy = y;
      } else if (line[x] === '#') {
        obstacles.add(`${x},${y}`);
      }
    }
  }
  return { w, h, obstacles, gx, gy };
};

const rotMap = { 'N': 'E', 'E': 'S', 'S': 'W', 'W': 'N' };

function move(x, y, dir) {
  switch(dir) {
    case 'N': return [x, y - 1];
    case 'E': return [x + 1, y];
    case 'S': return [x, y + 1];
    case 'W': return [x - 1, y];
  }
}

function runSim({ w, h, obstacles, gx, gy }, ox = -2, oy = -2) {
  const unique = new Set();
  const visited = new Set();
  let ggx = gx;
  let ggy = gy;
  let gd = 'N';
  while (ggx >= 0 && ggx < w && ggy >= 0 && ggy < h) {
    const visitKey = `${ggx},${ggy}`;
    const loopKey = `${visitKey}@${gd}`;
    visited.add(visitKey);
    if (unique.has(loopKey)) {
      return { reason: 'loop', nodes: NaN };
    } else {
      unique.add(loopKey);
    }
    const [nx, ny] = move(ggx, ggy, gd);
    if ((nx === ox && ny === oy) || obstacles.has(`${nx},${ny}`)) {
      gd = rotMap[gd];
    } else {
      ggx = nx;
      ggy = ny;
    }
  }
  return { reason: 'exit', nodes: visited };
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = (parsed) => {
  return runSim(parsed).nodes.size;
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (parsed, isSample) => {
  const { gx, gy } = parsed;
  let validSpots = new Set();
  for (const key of runSim(parsed).nodes) {
    const [ox, oy] = key.split(',').map(n => +n);
    if (ox === gx && oy === gy) {
      continue;
    }
  
    if (runSim(parsed, ox, oy).reason === 'loop') {
      validSpots.add(key);
    }
  }
  return validSpots.size;
};

aoc({
  year: 2024,
  day: 6,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
