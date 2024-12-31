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
  const computed = secrets.map(secret => {
    let s = secret;
    let lastDigit = n => +String(Number(n)).at(-1);
    let prev = [lastDigit(s)];
    const diffs = [];
    for (let i = 0; i < 2_000; i++) {
      s = ((s << 6n) ^ s) % 16777216n;
      s = ((s >> 5n) ^ s) % 16777216n;
      s = ((s << 11n) ^ s) % 16777216n;
      prev.push(lastDigit(s));
      diffs.push(prev[i + 1] - prev[i]);
    }
    return { prev, diffs };
  });

  // Incredibly naive and painfully slow: try every possible set of diffs.
  // See what score we get.
  // Keep the largest one.
  let mostBananas = 0;
  for (let d0 = -9; d0 <= 9; d0++) {
    for (let d1 = -9; d1 <= 9; d1++) {
      for (let d2 = -9; d2 <= 9; d2++) {
        for (let d3 = -9; d3 <= 9; d3++) {
          // The scoring method that we should not be using 19**4 times, but hey if it works...
          const bananas = computed.reduce((a, { prev, diffs }) => {
            for (let i = 4; i < diffs.length; i++) {
              if (diffs[i - 3] === d0 && diffs[i - 2] === d1 && diffs[i - 1] === d2 && diffs[i] === d3) {
                return a + prev[i + 1];
              }
            }
            return a;
          }, 0);
          if (bananas > mostBananas) mostBananas = bananas;
        }
      }
    }
  }
  return mostBananas;
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
