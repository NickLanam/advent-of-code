import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number|'SKIP'} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 'SKIP';

/** @type Part2Solution */
const part2expected = 'NYI';

const toKey = (x, y) => x * 1_000 + y;
const fromKey = (k) => ({ x: Math.floor(k / 1_000), y: k % 1_000 });

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
      const v = lines[y][x];
      switch (v) {
        case '#': walls.add(toKey(x, y)); break;
        case 'S': start = { x, y }; break;
        case 'E': end = { x, y }; break;
        default: break;
      }
    }
  }
  return { w, h, walls, start, end };
};

function dijkstra(w, h, start, end, walls, ignoreWall) {
  let numVisited = 0;
  let visits = new Set();

  const dist = new Map();
  const prev = new Map();
  dist.set(toKey(start.x, start.y), 0);

  while (numVisited < w * h) {
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
      x2 >= 0 && x2 < w &&
      y2 >= 0 && y2 < h &&
      ((x2 === ignoreWall.x && y2 === ignoreWall.y) || !walls.has(toKey(x2, y2)))
    ));
    
    for (const [nx, ny] of neighbors) {
      const nk = toKey(nx, ny);
      if (visits.has(nk)) continue;
      const c = dist.get(next.key) + 1;
      if (!dist.has(nk) || c < dist.get(nk)) {
        dist.set(nk, c);
        prev.set(nk, toKey(next.x, next.y));
      }
    }
    if (next.x === end.x && next.y === end.y) {
      break;
    }
  }

  const path = [];
  let u = toKey(end.x, end.y);
  if (prev.has(u) || u === toKey(start.x, start.y)) {
    while (u) {
      path.unshift(u);
      u = prev.get(u);
    }
  }
  return path;
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ w, h, walls, start, end }, isSample) => {
  const noCheatScore = (w * h) - walls.size;
  const savingsCounts = { 0: 0 };

  const noCheatPath = dijkstra(w, h, start, end, walls, { x: -1, y: -1 });
  if (noCheatScore !== noCheatPath.length) {
    throw new Error(`noCheatScore=${noCheatScore}, noCheatPath.length = ${noCheatPath.length}`);
  }
  for (let i = 0; i < noCheatPath.length; i++) {
    const { x: px, y: py } = fromKey(noCheatPath[i]);
    const nearWalls = [[px - 1, py], [px, py - 1], [px + 1, py], [px, py + 1]]
      .filter(([x2, y2]) => x2 >= 0 && x2 < w && y2 >= 0 && y2 < h && walls.has(toKey(x2, y2)));
    
    for (const [wx, wy] of nearWalls) {
      const nonWallNeighbors = [[wx - 1, wy], [wx, wy - 1], [wx + 1, wy], [wx, wy + 1]]
      .filter(([x2, y2]) => x2 >= 0 && x2 < w && y2 >= 0 && y2 < h && !walls.has(toKey(x2, y2)));
      for (const nwn of nonWallNeighbors) {
        const j = noCheatPath.indexOf(toKey(...nwn));
        const savings = j - i - 2;
        if (savings > 0) {
          savingsCounts[savings] = (savingsCounts[savings] ?? 0) + 1;
        }
      }
    }
  }
  if (isSample) return 'SKIP';
  return [...Object.entries(savingsCounts)].reduce((a, [s, c]) => a + (+s >= 100 ? c : 0), 0);
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
  day: 20,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
