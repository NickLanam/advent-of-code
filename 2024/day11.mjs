import aoc from './aoc.mjs';

/** @typedef {number[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 55312;

/** @type Part2Solution */
const part2expected = 65601038650482;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return lines[0].split(' ').map(n => +n);
};

/**
 * The rules of the challenge
 * @param {number} rock 
 * @returns {number[]}
 */
const step = (rock) {
  if (rock === 0) return [1];
  const s = String(rock);
  if (s.length % 2 === 0) {
    return [
      +s.substring(0, s.length / 2),
      +s.substring(s.length / 2),
    ];
  } else {
    return [2024 * rock];
  }
}

/**
 * 
 * @param {ParsedInput} rocks
 * @param {number} steps
 */
function solve(rocks, steps) {
  const memo = {};
  
  function solveInner(subset, steps) {
    if (steps === 0) return subset.length;
    if (!memo[steps]) memo[steps] = {};
    if (subset.length != 1) {
      return subset.map(rock => solveInner([rock], steps)).reduce((a, c) => a + c, 0);
    }
    const rock = subset[0];
    if (memo[steps][rock] == null) {
      memo[steps][rock] = step(rock)
        .map(n => solveInner([n], steps - 1))
        .reduce((a, c) => a + c, 0);
    }
    return memo[steps][rock];
  }

  return solveInner(rocks, steps);
}

aoc({
  year: 2024,
  day: 11,
  part1: (rocks) => solve(rocks, 25),
  part1expected,
  part2: (rocks) => solve(rocks, 75),
  part2expected,
  parse,
});
