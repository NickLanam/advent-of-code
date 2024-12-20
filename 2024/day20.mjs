import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 44;

/** @type Part2Solution */
const part2expected = 285;

const toKey = (x, y) => x * 1_000 + y;
const fromKey = (k) => [Math.floor(k / 1_000), k % 1_000];

const savedParses = {};

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  const h = lines.length;
  const w = lines[0].length;
  if (savedParses[h]) return savedParses[h];

  const points = new Set();
  let start;
  let end;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const v = lines[y][x];
      switch (v) {
        case '.': points.add(toKey(x, y)); break;
        case 'S': start = { x, y }; break;
        case 'E': end = { x, y }; break;
        default: break;
      }
    }
  }
  let path = [toKey(start.x, start.y)];
  let px = start.x;
  let py = start.y;
  while (px !== end.x || py !== end.y) {
    for (const [nx, ny] of [[px - 1, py], [px, py - 1], [px + 1, py], [px, py + 1]]) {
      const nk = toKey(nx, ny);
      if ((!points.has(nk) && (nx !== end.x || ny !== end.y)) || path.includes(nk)) continue;
      path.push(nk);
      px = nx;
      py = ny;
      break;
    }
  }
  savedParses[h] = path;
  return savedParses[h];
};

const CLOSE_DISTANCE = 2;
const FAR_DISTANCE = 20;
const savedSavings = {};
function findSavings(path, minSavings) {
  if (savedSavings[minSavings]) return savedSavings[minSavings];
  const closeSavings = new Map();
  const farSavings = new Map();
  
  // Next, for every point on the way, look for points further along that can be reached via a wall near the starting point.
  // Cheats are uniquely keyed (per problem description) by where they were activated (on the path) and where they ended (also on the path).
  // Notably, it's impossible for a cheat to jump less than 4 steps (moving and and out of a wall costs at least 2 steps itself).
  // The savings can be as low as 2 in some scenarios though.
  for (let i = 0; i < path.length - minSavings - 2; i++) {
    const pk = path[i];
    const [px, py] = fromKey(pk);

    for (let j = i + minSavings + 2; j < path.length; j++) {
      const jk = path[j];
      const expectedCost = j - i;
      const [jx, jy] = fromKey(jk);
      const cost = Math.abs(jx - px) + Math.abs(jy - py);
      const savings = expectedCost - cost;
      if (savings > 0) {
        if (cost <= CLOSE_DISTANCE) {
          closeSavings.set(savings, (closeSavings.get(savings) ?? 0) + 1);
        }
        if (cost <= FAR_DISTANCE) {
          farSavings.set(savings, (farSavings.get(savings) ?? 0) + 1);
        }
      }
    }
  }
  savedSavings[minSavings] = { closeSavings, farSavings };
  return savedSavings[minSavings];
}

function solve(path, forPart, isSample) {
  let minSavings = isSample ? [, 2, 50][forPart] : 100;
  const { closeSavings, farSavings } = findSavings(path, minSavings);
  const which = forPart === 1 ? closeSavings : farSavings;
  
  let out = 0;
  for (const [k, v] of which) {
    if (k >= minSavings) {
      out += v;
    }
  }
  return out;
}

aoc({
  year: 2024,
  day: 20,
  part1: (parsed, isSample) => solve(parsed, 1, isSample),
  part1expected,
  part2: (parsed, isSample) => solve(parsed, 2, isSample),
  part2expected,
  parse,
});
