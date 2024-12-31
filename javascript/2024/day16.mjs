import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 11048;

/** @type Part2Solution */
const part2expected = 64;

const coordDirKeyDirMap = { 'N': 0.1, 'E': 0.2, 'S': 0.3, 'W': 0.4 };
const coordToKey = (x, y) => x * 1_000 + y;
const coordDirKey = (x, y, dir) => x * 1_000 + y + coordDirKeyDirMap[dir];
const fromCoordDirKey = (k) => ({
  x: Math.floor(k / 1_000),
  y: Math.floor(k % 1_000),
  dir: ['N', 'E', 'S', 'W'][Math.round(((k % 1) * 10)) - 1]
});

const parses = {};
/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  const h = lines.length;
  if (parses[h]) return parses[h];
  const w = lines[0].length;
  const walls = new Set();
  let start;
  let end;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const c = lines[+y][+x];
      switch (c) {
        case '.': break;
        case '#': { walls.add(coordToKey(x, y)); break; }
        case 'S': { start = [x, y]; break; }
        case 'E': { end = [x, y]; break; }
        default: break;
      }
    }
  }
  parses[h] = {
    walls,
    start,
    end,
    w,
    h
  };
  return parses[h];
};

const moveCostMap = {
  'N': { 'N': 1, 'E': 1_001, 'S': 2_001, 'W': 1_001 },
  'E': { 'N': 1_001, 'E': 1, 'S': 1_001, 'W': 2_001 },
  'S': { 'N': 2_001, 'E': 1_001, 'S': 1, 'W': 1_001 },
  'W': { 'N': 1_001, 'E': 2_001, 'S': 1_001, 'W': 1 }
};

function moveCost(oldDir, newDir) {
  return moveCostMap[oldDir][newDir];
}

function getMovesWithCost(w, h, x, y, dir, walls) {
  return [
    [x, y - 1, 'N', moveCost(dir, 'N')],
    [x + 1, y, 'E', moveCost(dir, 'E')],
    [x, y + 1, 'S', moveCost(dir, 'S')],
    [x - 1, y, 'W', moveCost(dir, 'W')]
  ]
    .filter((m) => (
      m[0] >= 0 &&
      m[0] < w &&
      m[1] >= 0 &&
      m[1] < h &&
      !walls.has(coordToKey(m[0], m[1])) &&
      !['NS', 'SN', 'EW', 'WE'].includes(dir + m[2])
    ))
    .map(([x, y, dir, cost]) => ({ cost, key: coordDirKey(x, y, dir) }));
}

const saved = {};
function dijkstra(w, h, walls, [startX, startY], [goalX, goalY]) {
  if (saved[w]) return saved[w];
  let numVisited = 0;
  let visits = new Set();

  const dist = new Map();
  const prev = new Map();
  dist.set(coordDirKey(startX, startY, 'E'), 0);

  while (numVisited < w * h * 4) {
    const next = { x: -1, y: -1, cost: Infinity, key: -1, dir: null };

    for (const [k, cost] of dist) {
      if (!visits.has(k) && cost < next.cost) {
        const { x, y, dir } = fromCoordDirKey(k);
        next.x = x;
        next.y = y;
        next.dir = dir;
        next.cost = cost;
        next.key = k;
      }
    }

    numVisited++;
    visits.add(next.key);

    const legalMoves = getMovesWithCost(w, h, next.x, next.y, next.dir, walls);
    for (const neighbor of legalMoves) {
      if (visits.has(neighbor.key)) continue;
      const c = dist.get(next.key) + neighbor.cost;
      if (!dist.has(neighbor.key) || c < dist.get(neighbor.key)) {
        dist.set(neighbor.key, c);
        prev.set(neighbor.key, next);
      }
    }
    if (next.x === goalX && next.y === goalY) {
      break;
    }
  }

  saved[w] = { dist, prev };
  return { dist, prev };
}

function keysAtCoord(x, y) {
  return ['N', 'E', 'S', 'W'].map(dir => coordDirKey(x, y, dir));
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ w, h, walls, start, end }) => {
  const { dist } = dijkstra(w, h, walls, start, end);
  return Math.min(
    ...keysAtCoord(...end).map(k => dist.get(k)).filter(v => v != null));
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = ({ w, h, walls, start, end }) => {
  const { dist, prev } = dijkstra(w, h, walls, start, end);

  let seen = new Set();
  let seenDirectionless = new Set();
  const queue = keysAtCoord(...end);

  while(queue.length > 0) {
    const k = queue.pop();
    if (seen.has(k)) {
      continue;
    } else {
      seen.add(k);
      seenDirectionless.add(Math.floor(k));
    }
    if (!(dist.has(k) && prev.has(k))) continue;
    
    const d = dist.get(k);
    const p = prev.get(k);
    for (const pk of keysAtCoord(p.x, p.y)) {
      if ((dist.get(pk) ?? Infinity) + moveCost(fromCoordDirKey(k).dir, fromCoordDirKey(pk).dir) === d) {
        queue.push(pk);
      }
    }
  }

  return seenDirectionless.size;
};

aoc({
  year: 2024,
  day: 16,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
