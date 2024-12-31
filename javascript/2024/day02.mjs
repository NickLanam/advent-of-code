import aoc from './aoc.mjs';

/** @typedef {number[][]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 2;

/** @type Part2Solution */
const part2expected = 4;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  return lines.map(l => l.split(' ').map(n => +n));
};

/**
 * 
 * @param {ParsedInput} reports 
 * @returns {Part1Solution}
 */
const part1 = (reports) => {
  let numValid = 0;
  reportLoop: for (const report of reports) {
    const dir = Math.sign(report[1] - report[0]);
    for (let i = 0; i < report.length - 1; i++) {
      const l = report[i];
      const n = report[i + 1];
      if (Math.sign(n - l) !== dir || n === l || Math.abs(n - l) > 3) {
        continue reportLoop;
      }
    }
    numValid++;
  }
  return numValid;
};

/**
 * 
 * @param {ParsedInput} reports 
 * @returns {Part2Solution}
 */
const part2 = (reports) => {
  return reports.map((startLevels) => {
    holeCheck: for (let hole = -1; hole < startLevels.length; hole++) {
      const levels = hole >= 0 ? startLevels.slice(0, hole).concat(startLevels.slice(hole + 1)) : startLevels;
      let dir = Math.sign(levels[1] - levels[0]);
      for (let i = 0; i < levels.length - 1; i++) {
        const l = levels[i];
        let n = levels[i + 1];
        if (Math.sign(n - l) !== dir || Math.abs(n - l) === 0 || Math.abs(n - l) > 3) {
          continue holeCheck;
        }
      }
      return true;
    }
    return false;
  }).reduce((a, c) => a + !!c, 0);
};

aoc({
  year: 2024,
  day: 2,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
