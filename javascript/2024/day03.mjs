import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Solution */

/** @type Solution */
const part1expected = 161;

/** @type Solution */
const part2expected = 48;

/**
 * 
 * @param {ParsedInput} parsed 
 * @param {boolean} enableDoDont
 * @returns {Solution}
 */
const solve = (parsed, enableDoDont) => {
  let enabled = true;
  return parsed.map((line) => {
    const re = /(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))/g;
    let sum = 0;
    let n;
    while ((n = re.exec(line)) != null) {
      if (n[0] === "don't()") {
        if (enableDoDont) {
          enabled = false;
        }
      } else if (n[0] === "do()") {
        enabled = true;
      } else if (enabled) {
        sum += (+n[2])*(+n[3]);
      }
    }
    return sum;
  }).reduce((a, c) => a+c, 0);
};

aoc({
  year: 2024,
  day: 3,
  part1: (parsed) => solve(parsed, false),
  part1expected,
  part2: (parsed) => solve(parsed, true),
  part2expected,
});
