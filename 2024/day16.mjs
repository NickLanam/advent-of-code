import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 11048;

/** @type Part2Solution */
const part2expected = 'NYI';

const coordDirKeyDirMap = { 'N': 0.1, 'E': 0.2, 'S': 0.3, 'W': 0.4 };
const coordToKey = (x, y) => x * 1_000 + y;
const fromCoordKey = (k) => [Math.floor(k / 1_000), k % 1_000];
const coordDirKey = (x, y, dir) => x * 1_000 + y + coordDirKeyDirMap[dir];
const fromCoordDirKey = (k) => ({
  x: Math.floor(k / 1_000),
  y: Math.floor(k % 1_000),
  dir: ['N', 'E', 'S', 'W'][Math.round(((k % 1) * 10) - 1)]
});

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  const h = lines.length;
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
  return {
    walls,
    start,
    end,
    w,
    h
  };
};

const rotLeftMap = { 'N': 'W', 'E': 'N', 'S': 'E', 'W': 'S' };
const rotRightMap = { 'N': 'E', 'E': 'S', 'S': 'W', 'W': 'N' };
function getMovesWithCost(w, h, x, y, dir, walls) {
  let forward;
  switch (dir) {
    case 'N': forward = { x, y: y - 1, dir, cost: 1 }; break;
    case 'E': forward = { x: x + 1, y, dir, cost: 1 }; break;
    case 'S': forward = { x, y: y + 1, dir, cost: 1 }; break;
    case 'W': forward = { x: x - 1, y, dir, cost: 1 }; break;
    default: throw new Error('What direction is this? ' + dir);
  }

  const rotLeft = {
    x, y, cost: 1_000,
    dir: rotLeftMap[dir]
  };

  const rotRight = {
    x, y, cost: 1_000,
    dir: rotRightMap[dir]
  };

  return [forward, rotLeft, rotRight]
    .filter(({ x, y }) => x >= 0 && x < w && y >= 0 && y < h && !walls.has(coordToKey(x, y)))
    .map(move => ({ ...move, key: coordDirKey(move.x, move.y, move.dir) }));
}

function dijkstra(w, h, walls, [startX, startY], [goalX, goalY]) {
  let numVisited = 0;
  let visits = new Set();

  const dist = new Map();
  const prev = new Map();
  dist.set(coordDirKey(startX, startY, 'E'), 0);

  let foundPath = false;
  // const startTime = performance.now();
  while (numVisited < w * h * 4) {
    if (numVisited % 100 === 0) {
      // console.log(numVisited, performance.now() - startTime);
    }
    const next = { x: -1, y: -1, cost: Infinity, key: -1, dir: null };
    for (let y = 0; y < h; y++) {
      for (let x = 0; x < w; x++) {
        for (const dir of ['N', 'E', 'S', 'W']) {
          const k = coordDirKey(x, y, dir);
          if (visits.has(k)) continue;
          if (dist.has(k) && dist.get(k) < next.cost) {
            next.x = x;
            next.y = y;
            next.cost = dist.get(k);
            next.dir = dir;
            next.key = k;
          }
        }
      }
    }
    if (next.key < 0) throw new Error('Next did not get set');
    numVisited++;
    visits.add(next.key);
    // This one seems to be about 0.6ms each run, which isn't great but isn't the problem either - the rest is waaaay more expensive
    // if (numVisited % 100 === 0) {
    //   console.log(numVisited, performance.now() - startTime);
    // }

    const legalMoves = getMovesWithCost(w, h, next.x, next.y, next.dir, walls);
    if (!legalMoves.length) {
      throw new Error(`No legal moves from ${next.x},${next.y} ${next.dir}, should be impossible`);
    }
    for (const neighbor of legalMoves) {
      if (visits.has(neighbor.key)) continue;
      const c = dist.get(next.key) + neighbor.cost;
      if (!dist.has(neighbor.key) || c < dist.get(neighbor.key)) {
        dist.set(neighbor.key, c);
        prev.set(neighbor.key, [next.x, next.y, next.dir]);
      }
    }
    if (next.x === goalX && next.y === goalY) {
      foundPath = true;
      break;
    }
  }
  // const endTime = performance.now();
  // console.info('It took', (endTime - startTime) / numVisited, 'on average');

  if (!foundPath) {
    throw new Error('Failed to find a path to the goal after visiting all nodes!');
  }

  // console.log(visits);
  // console.log([...visits].map(v => fromCoordDirKey(v)));
  // console.log(visits.has(coordDirKey(goalX, goalY, 'N'))); // False, uh-oh
  // TODO: Getting Infinity back from the sample, need to add a ton of logs
  const rets = ['N', 'E', 'S', 'W'].map(dir => dist.get(coordDirKey(goalX, goalY, dir))).filter(v => v != null);
  return Math.min(...rets);
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ w, h, walls, start, end }) => {
  return dijkstra(w, h, walls, start, end);
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (parsed) => {
  return 'NYI';
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