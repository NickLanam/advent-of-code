import aoc from './aoc.mjs';

/** @typedef {{p0: {x: number, y: number}, p1: {x: number, y: number}, goal: {x: number, y: number}}[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 480;

/** @type Part2Solution */
const part2expected = 875318608908;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  const partMod = forPart === 1 ? 0 : 10_000_000_000_000;
  const grouped = lines.join('\n').split('\n\n').map(group => group.split('\n'));
  return grouped.map(([a, b, p]) => {
    const axy = a.match(/^Button A: X\+(\d+), Y\+(\d+)$/);
    const bxy = b.match(/^Button B: X\+(\d+), Y\+(\d+)$/);
    const pxy = p.match(/^Prize: X=(\d+), Y=(\d+)$/);
    return {
      p0: { x: +axy[1], y: +axy[2] },
      p1: { x: +bxy[1], y: +bxy[2] },
      goal: { x: +pxy[1] + partMod, y: +pxy[2] + partMod },
    };
  });
};

/**
 * Linear algebra... we meet again.
 * Specifically, this is an invertible matrix situation.
 * Originally I tried to solve it the way a 9th grader might,
 * with slope/intercept matching, but that was more complex
 * and dealt with floating point error. This doesn't fail that way.
 * 
 * @param {ParsedInput} groups
 * @returns {number}
 */
function solve(groups) {
  return groups.map(({ p0, p1, goal }) => {
    // Basic matrix inversion that I haven't done since college...
    // Yes, I had to look up how to do so.
    const bPresses = (p0.x * goal.y - p0.y * goal.x)/(p0.x * p1.y - p0.y * p1.x);
    const aPresses = (goal.x - p1.x * bPresses) / p0.x;
    if (!Number.isNaN(aPresses) && !Number.isNaN(bPresses) && aPresses % 1 === 0 && bPresses % 1 === 0) {
      return aPresses * 3 + bPresses;
    }
    return 0;
  }).reduce((a, c) => a + c, 0);
}

aoc({
  year: 2024,
  day: 13,
  part1: solve,
  part1expected,
  part2: solve,
  part2expected,
  parse,
});
