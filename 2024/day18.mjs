import aoc from './aoc.mjs';

/** @typedef {[x: number, y: number][]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {string} Part2Solution */

/** @type Part1Solution */
const part1expected = 22;

/** @type Part2Solution */
const part2expected = '6,1';

const toKey = (x, y) => x * 100 + y;
const fromKey = (k) => ({ x: Math.floor(k / 100), y: k % 100 });

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return lines.map(l => l.split(',').map(n => +n));
};

function dijkstra(dim, walls) {
  // start is 0,0; goal is dim-1,dim-1, always
  let numVisited = 0;
  let visits = new Set();

  const dist = new Map();
  const prev = new Map();
  dist.set(toKey(0, 0), 0);

  while (numVisited < dim * dim) {
    const next = { x: -1, y: -1, cost: Infinity, key: -1 };
    for (const [k, cost] of dist) {
      if (!visits.has(k) && cost < next.cost) {
        const { x, y } = fromKey(k);
        next.x = x;
        next.y = y;
        next.cost = cost;
        next.key = k;
      }
    }

    numVisited++;
    visits.add(next.key);

    const neighbors = [
      [next.x - 1, next.y],
      [next.x, next.y - 1],
      [next.x + 1, next.y],
      [next.x, next.y + 1]
    ].filter(([x2, y2]) => (
      x2 >= 0 && x2 < dim &&
      y2 >= 0 && y2 < dim &&
      !walls.has(toKey(x2, y2))));
    
    for (const [nx, ny] of neighbors) {
      const nk = toKey(nx, ny);
      if (visits.has(nk)) continue;
      const c = dist.get(next.key) + 1;
      if (!dist.has(nk) || c < dist.get(nk)) {
        dist.set(nk, c);
        prev.set(nk, next);
      }
    }
    if (next.x === dim - 1 && next.y === dim - 1) {
      break;
    }
  }

  return { dist, prev };
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @param {boolean} isSample
 * @returns {Part1Solution}
 */
const part1 = (parsed, isSample) => {
  const dim = isSample ? 7 : 71;
  const steps = isSample ? 12 : 1024;
  const walls = new Set();

  for (let i = 0; i < steps; i++) {
    walls.add(toKey(...parsed[i]));
  }

  const { dist } = dijkstra(dim, walls);
  return dist.get(toKey(dim - 1, dim - 1));
};

/**
 * 
 * @param {ParsedInput} parsed 
 * * @param {boolean} isSample
 * @returns {Part2Solution}
 */
const part2 = (parsed, isSample) => {
  const dim = isSample ? 7 : 71;
  const goalKey = toKey(dim - 1, dim - 1);
  
  let lo = 0;
  let hi = parsed.length - 1;
  while (true) {
    const steps = Math.floor((lo + hi) / 2);
    const walls = new Set();

    for (let i = 0; i < steps; i++) {
      walls.add(toKey(...parsed[i]));
    }

    const { dist } = dijkstra(dim, walls);
    if (dist.get(goalKey) < Infinity) {
      lo = steps;
    } else {
      hi = steps;
    }

    if (Math.abs(hi - lo) < 2) {
      return parsed[lo].join(',');
    }
  }
};

aoc({
  year: 2024,
  day: 18,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
