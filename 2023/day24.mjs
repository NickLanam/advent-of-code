import aoc from './aoc.mjs';

/** @typedef {{px: number, py: number, pz: number, vx: number, vy: number, vz: number}[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 2;

/** @type Part2Solution */
const part2expected = 'NYI';

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  return lines.map(l => l.split(' @ ').map(h => h.split(', ').map(n => +n))).map(([[px, py, pz], [vx, vy, vz]]) => ({ px, py, pz, vx, vy, vz }));
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @param {boolean} isSample
 * @returns {Part1Solution}
 */
const part1 = (lines, isSample) => {
  const minRange = isSample ? 7 : 200_000_000_000_000;
  const maxRange = isSample ? 27 : 400_000_000_000_000;

  const equations = lines.map(({ px, py, vx, vy }) => {
    // No positions OR velocities equal to zero, so don't have to worry about NaN.
    const m = vy / vx;
    const b = py - m * px;
    return { px, py, vx, vy, m, b };
  });

  // console.log(equations);
  
  let collisionsInBand = 0;
  for (let i = 0; i < equations.length - 1; i++) {
    for (let j = i + 1; j < equations.length; j++) {
      const ea = equations[i];
      const eb = equations[j];
      const x = (eb.b - ea.b) / (ea.m - eb.m);
      const y = ea.m * (eb.b - ea.b) / (ea.m - eb.m) + ea.b;
      const inBounds = x >= minRange && x <= maxRange && y >= minRange && y <= maxRange;
      const isFutureA = Math.sign(x - ea.px) === Math.sign(ea.vx) && Math.sign(y - ea.py) === Math.sign(ea.vy);
      const isFutureB = Math.sign(x - eb.px) === Math.sign(eb.vx) && Math.sign(y - eb.py) === Math.sign(eb.vy);
      // console.log({ inBounds, isFutureA, isFutureB, ea, eb, x, y });
      if (inBounds && isFutureA && isFutureB) {
        collisionsInBand++;
      }
    }
  }
  return collisionsInBand;
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
  year: 2023,
  day: 24,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
