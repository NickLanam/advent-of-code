import aoc from './aoc.mjs';

/** @typedef {{ w: number, h: number, grid: ('.'|'#')[], start: [x: number, y: number] }} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 16; // With 6 steps

/** @type Part2Solution */
const part2expected = 16_733_044; // With 5_000 steps

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  let start = [-1, -1];
  const h = lines.length;
  const w = lines[0].length;
  const grid = lines.map((line, y) => line.split('').map((char, x) => {
    if (char === 'S') {
      start = [x, y];
      return '.';
    } else {
      return char;
    }
  }));
  return { w, h, grid, start };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @param {boolean} isSample
 * @returns {Part1Solution}
 */
const part1 = ({ w, h, grid, start }, isSample) => {
  const REQUIRED_STEPS = isSample ? 6 : 64;

  const getNeighbors = (x, y) => [
    [x - 1, y],
    [x + 1, y],
    [x, y - 1],
    [x, y + 1],
  ].filter(([x2, y2]) => x2 >= 0 && x2 < w && y2 >= 0 && y2 < h && grid[y2][x2] !== '#');

  const coordKey = ([x, y]) => x * 1_000 + y;

  const reachable = new Set();
  reachable.add(coordKey(start));
  
  const seen = new Set();

  let exploreStack = [{ x: start[0], y: start[1], stepsTaken: 0 }];

  while (exploreStack.length > 0) {
    const { x, y, stepsTaken } = exploreStack.shift();
    const k = coordKey([x, y]);

    if (seen.has(k)) continue;
    seen.add(k);

    if ((REQUIRED_STEPS - stepsTaken) % 2 === 0 && stepsTaken <= REQUIRED_STEPS) {
      reachable.add(k);
    }
    if (stepsTaken >= REQUIRED_STEPS) continue;
    for (const [nx, ny] of getNeighbors(x, y)) {
      if (!reachable.has(coordKey([nx, ny]))) {
        exploreStack.push({ x: nx, y: ny, stepsTaken: stepsTaken + 1 });
      }
    }
  }

  // DEBUGGING ONLY
  // This is how I found a bug in my traversal logic that only happened on the real input
  if (false) {
    console.log({ w, h, start });
    for (let y = 0; y < h; y++) {
      let line = '';
      for (let x = 0; x < w; x++) {
        const char = reachable.has(coordKey([x, y])) ? 'O' : grid[y][x];
        line += (start[0] === x && start[1] === y) ? `\u001b[1;32m${char}\u001b[0m` : char;
      }
      console.log(line);
    }
  }

  return reachable.size;
};

/**
 * 
 * @param {ParsedInput} parsed
 * @param {boolean} isSample
 * @returns {Part2Solution}
 */
const part2 = ({ w, h, grid, start }, isSample) => {
  const REQUIRED_STEPS = isSample ? 5_000 : 26501365;

  const getNeighbors = (x, y) => [
    [x - 1, y],
    [x + 1, y],
    [x, y - 1],
    [x, y + 1],
  ].filter(([x2, y2]) => x2 >= 0 && x2 < w && y2 >= 0 && y2 < h && grid[y2][x2] !== '#');

  const coordKey = ([x, y]) => x * 1_000 + y;

  const reachable = new Set();
  reachable.add(coordKey(start));
  
  const seen = new Set();

  let exploreStack = [{ x: start[0], y: start[1], stepsTaken: 0 }];

  // TODO: The grid is now infinitely tiled on both axes.
  // There will absolutely be a Set max size reached if I do this the same naive way.
  // As with the last few days, the optimization will probably be to find the pattern,
  // and not actually simulate the entire process but instead to multiply a result.

  // My guess: figure out which edges (and with how many remaining steps) can be exited from the center start,
  // and make a copy of the map that enters from there (with that many remaining steps) and does the same.
  // Memoize results, if there's a simplification that allows hash overlap anyway.
  // Like days 5 and 20, that should be enough to simulate only far enough to find the cycle then do lcm or similar.

  while (exploreStack.length > 0) {
    const { x, y, stepsTaken } = exploreStack.shift();
    const k = coordKey([x, y]);

    if (seen.has(k)) continue;
    seen.add(k);

    if ((REQUIRED_STEPS - stepsTaken) % 2 === 0 && stepsTaken <= REQUIRED_STEPS) {
      reachable.add(k);
    }
    if (stepsTaken >= REQUIRED_STEPS) continue;
    for (const [nx, ny] of getNeighbors(x, y)) {
      if (!reachable.has(coordKey([nx, ny]))) {
        exploreStack.push({ x: nx, y: ny, stepsTaken: stepsTaken + 1 });
      }
    }
  }
};

aoc({
  year: 2023,
  day: 21,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
