import aoc from './aoc.mjs';

/** @typedef {number[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 37990510;

/** @type Part2Solution */
const part2expected = 23;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  return lines.map(n => BigInt(+n));
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = (secrets) => {
  return Number(secrets.map(secret => {
    let s = secret;
    for (let i = 0; i < 2_000; i++) {
      s = ((s << 6n) ^ s) % 16777216n;
      s = ((s >> 5n) ^ s) % 16777216n;
      s = ((s << 11n) ^ s) % 16777216n;
    }
    return s;
  }).reduce((a, c) => a + c, 0n));
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (secrets) => {
  return Number(secrets.map(secret => {
    let s = secret;
    let lastDigit = n => +String(Number(n)).at(-1);
    let prev = [lastDigit(s)];
    for (let i = 0; i < 2_000; i++) {
      s = ((s << 6n) ^ s) % 16777216n;
      s = ((s >> 5n) ^ s) % 16777216n;
      s = ((s << 11n) ^ s) % 16777216n;
      prev.push(lastDigit(s));
    }
    return s;
  }).reduce((a, c) => a + c, 0n));
};

aoc({
  year: 2024,
  day: 22,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
