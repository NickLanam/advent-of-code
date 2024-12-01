import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number|'NYI'} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 11;

/** @type Part2Solution */
const part2expected = 31;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  return lines;
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = (parsed) => {
  const zipped = parsed.map(l => l.split('   ').map(n => +n));
  const left = zipped.map(([l,r]) => l);
  const right = zipped.map(([l,r]) => r);
  left.sort();
  right.sort();
  const zippedAgain = left.map((l, i) => Math.abs(l - right[i]));
  return zippedAgain.reduce((a, c) => a+c, 0);
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (parsed) => {
  const zipped = parsed.map(l => l.split('   ').map(n => +n));
  const left = zipped.map(([l,r]) => l);
  const right = zipped.map(([l,r]) => r);
  left.sort();
  right.sort();
  const frequencies = left.map(l => right.filter(r => r === l).length);
  return left.reduce((a, c, i) => a+(c * frequencies[i]), 0);
};

aoc({
  year: 2024,
  day: 1,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
