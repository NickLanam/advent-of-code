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
const fromCoordKey = (k) => ({ x: Math.floor(k / 1_000), y: Math.floor(k % 1_000) });
const coordDirKey = (x, y, dir) => x * 1_000 + y + coordDirKeyDirMap[dir];
const fromCoordDirKey = (k) => ({
  x: Math.floor(k / 1_000),
  y: Math.floor(k % 1_000),
  dir: ['N', 'E', 'S', 'W'][Math.round(((k % 1) * 10) - 1)]
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
        default: throw new Error('Bad char: ' + c);
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

  let foundPath = false;
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

    if (next.key < 0) throw new Error('Next did not get set');
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
      foundPath = true;
      break;
    }
  }

  if (!foundPath) {
    throw new Error('Failed to find a path to the goal after visiting all nodes!');
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
  // console.info('Initial queue', queue);
  while (queue.length > 0) {
    const top = queue.pop();
    const { x, y, dir } = fromCoordDirKey(top);
    const n = Math.floor(top);
    if (seen.has(top) && (x != end[0] || y != end[1])) {
      // console.log('ALREADY SAW', { x, y });
      continue;
    }
    seen.add(top);
    seenDirectionless.add(n);
    for (const k of keysAtCoord(x, y)) {
      if (dist.has(k) && prev.has(k)) {
        const p = prev.get(k);
        const mc = moveCost(p.dir, dir);
        const line = `${x},${y} \x1b[35m${dir}\x1b[0m \$${dist.get(k)} \x1b[90m←\x1b[0m ${p.x},${p.y} \x1b[35m${p.dir}\x1b[0m \$${p.cost} (${mc}, ${k}, ${p.key})`;
        if (p.cost + mc === dist.get(k)) {
          if (x === 3 && y < 7) console.log('\x1b[32m✔\x1b[0m', line);
          queue.push(...keysAtCoord(p.x, p.y));
        } else {
          if (x === 3 && y < 7) console.log('\x1b[31m✖\x1b[0m', line);
          //queue.push(coordDirKey(p.x, p.y, p.dir));
        }
      } else {
        // console.log('MISS', { x, y, k });
      }
    }

  }

  // DEBUGGING: Why do I get 73 when there should have been 64?
  for (let y = 0; y < h; y++) {
    let line = '';
    for (let x = 0; x < w; x++) {
      const seenXY = seenDirectionless.has(coordToKey(x, y));
      if (walls.has(coordToKey(x, y))) {
        const connNorth = (walls.has(coordToKey(x, y - 1)) & 1) << 3;
        const connSouth = (walls.has(coordToKey(x, y + 1)) & 1) << 2;
        const connEast = (walls.has(coordToKey(x + 1, y)) & 1) << 1;
        const connWest = (walls.has(coordToKey(x - 1, y)) & 1);
        const conn = connNorth + connSouth + connEast + connWest;
        const boxChar = {
          0b0000: '⚬',
          0b0001: '─', // '╴',
          0b0010: '─', // '╶',
          0b0011: '─',
          0b0100: '│', //'╷',
          0b0101: '┐',
          0b0110: '┌',
          0b0111: '┬',
          0b1000: '│', // '╵',
          0b1001: '┘',
          0b1010: '└',
          0b1011: '┴',
          0b1100: '│',
          0b1101: '┤',
          0b1110: '├',
          0b1111: '┼',
        }[conn];
        line += seenXY ? '\x1b[31m!\x1b[0m' : `\x1b[90m${boxChar}\x1b[0m`;
      } else if (seenXY) {
        line += '\x1b[32m⚬\x1b[0m';
      } else {
        line += '\x1b[90m·\x1b[0m';
      }
    }
    console.log(line);
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
