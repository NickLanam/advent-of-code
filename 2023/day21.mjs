import aoc from './aoc.mjs';

/** @typedef {{ w: number, h: number, grid: ('.'|'#')[], start: [x: number, y: number] }} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number|'N/A'} Part2Solution */

/** @type Part1Solution */
const part1expected = 16; // With 6 steps

/** @type Part2Solution */
const part2expected = 'N/A'; // The sample has a different set of properties than the real input.

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines) => {
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
 * Figure out how many locations can be reached with the given number of steps in one subgrid
 *
 * @param {ParsedInput} parsed 
 * @param {number} requiredSteps
 * @returns {number}
 */
const solveOneGrid = ({ w, h, grid, start}, requiredSteps) => {
  const sk = start[0] * 1_000 + start[1];

  // TODO: Do I actually need to even keep track of parity this way? Probably not?
  // My original part 1 solution didn't need to, but was always an even step count...
  const reachableEven = new Set();
  const reachableOdd = new Set();
  reachableEven.add(sk);
  reachableOdd.add(sk);

  const seen = new Set();

  let exploreStack = [{ x: start[0], y: start[1], stepsTaken: 0 }];

  while (exploreStack.length > 0) {
    const { x, y, stepsTaken } = exploreStack.shift();
    const k = x * 1_000 + y;

    if (seen.has(k)) continue;
    seen.add(k);

    if (stepsTaken <= requiredSteps) {
      const isEven = stepsTaken % 2 === 0;
      if (isEven) {
        reachableEven.add(k);
      } else {
        reachableOdd.add(k);
      }
    }
    if (stepsTaken >= requiredSteps) continue;

    const possibleNeighbors = [
      [x - 1, y],
      [x + 1, y],
      [x, y - 1],
      [x, y + 1],
    ];

    for (const [nx, ny] of possibleNeighbors) {
      if (nx < 0 || nx >= w || ny < 0 || ny >= h || grid[ny][nx] === '#') {
        continue;
      }

      const nk = nx * 1_000 + ny;
      if (!reachableEven.has(nk) && !reachableOdd.has(nk)) {
        exploreStack.push({ x: nx, y: ny, stepsTaken: stepsTaken + 1 });
      }
    }
  }

  return {
    reachableEven: reachableEven.size,
    reachableOdd: reachableOdd.size - 1, // Parity
  };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @param {boolean} isSample
 * @returns {Part1Solution}
 */
const part1 = ({ w, h, grid, start }, isSample) => {
  return solveOneGrid({ w, h, grid, start }, isSample ? 6 : 64).reachableEven;
};

/**
 * 
 * @param {ParsedInput} parsed
 * @param {boolean} isSample
 * @returns {Part2Solution}
 */
const part2 = ({ w, h, grid, start }, isSample) => {
  if (isSample) {
    // The real input has a property that the sample doesn't:
    // The center row and column are clear (no obstacles),
    // which allows for a lot of the simplifications below to work.
    // Without that property, a more convoluted solution would be needed.
    return 'N/A';
  }

  const {
    reachableEven: centerReachableEven,
    reachableOdd: centerReachableOdd,
  } = solveOneGrid({ w, h, grid, start }, Infinity);
  
  const requiredSteps = 26_501_365;

  // Kind of like a radius, but the shape grows in a diamond, not a circle.
  // The logic of the puzzle is the same either way, though.
  const cardinalTileReach = Math.floor(requiredSteps / h);

  const cardinalStepLimit = (requiredSteps - start[0] - 1) % h;
  const lowerDiagStepLimit = (requiredSteps - start[0] - start[1] - h - 2) % (w + h);
  const upperDiagStepLimit = (requiredSteps - start[0] - start[1] - 2) % (w + h);

  let numOddTiles = 1;
  let numEvenTiles = 0;
  for (let tile = 0; tile < cardinalTileReach; tile++) {
    if (tile % 2 === 1) {
      numEvenTiles += tile * 4;
    } else {
      numOddTiles += tile * 4;
    }
  }

  const fullGridSum = (numOddTiles * centerReachableOdd) + (numEvenTiles * centerReachableEven);

  let cardinalSum = 0;
  for (const cardinalStart of [
    [start[0], 0], // North
    [w - 1, start[1]], // East
    [start[0], h - 1], // South
    [0, start[1]], // West
  ]) {
    const { reachableEven, reachableOdd } = solveOneGrid({ w, h, grid, start: cardinalStart }, cardinalStepLimit);
    cardinalSum += (cardinalStepLimit % 2 === 0 ? reachableEven : reachableOdd);
  }

  let diagonalSum = 0;
  for (const diagStart of [
    [0, 0], // Northwest
    [w - 1, 0], // Northeast
    [w - 1, h - 1], // Southeast
    [0, h - 1], // Southwest
  ]) {
    const loLim = solveOneGrid({ w, h, grid, start: diagStart }, lowerDiagStepLimit);
    const hiLim = solveOneGrid({ w, h, grid, start: diagStart }, upperDiagStepLimit);
    if (lowerDiagStepLimit % 2 === 0) {
      diagonalSum += loLim.reachableEven * cardinalTileReach;
      diagonalSum += hiLim.reachableOdd * (cardinalTileReach - 1);
    } else {
      diagonalSum += loLim.reachableOdd * cardinalTileReach;
      diagonalSum += hiLim.reachableEven * (cardinalTileReach - 1);
    }
  }

  return fullGridSum + cardinalSum + diagonalSum;
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
