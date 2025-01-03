import aoc from './aoc.mjs';

/** @typedef {{ towels: string[], patterns: string[] }} ParsedInput */
/** @typedef {number} Solution */

/** @type Solution */
const part1expected = 6;

/** @type Solution */
const part2expected = 16;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  const towels = lines[0].split(', ');
  const patterns = lines.slice(2);
  return { towels, patterns };
};

/**
 * @param {ParsedInput} parsed 
 * @returns 
 */
function solve({ towels, patterns }) {
  const subSolves = new Map();
  subSolves.set('', 1);

  function search(target) {
    if (subSolves.has(target)) return subSolves.get(target);
    let foundSolutions = 0;
    for (const towel of towels) {
      if (target.startsWith(towel)) {
        foundSolutions += search(target.substring(towel.length));
      }
    }
    subSolves.set(target, foundSolutions);
    return foundSolutions;
  }

  return patterns.map(pattern => search(pattern));
}

aoc({
  year: 2024,
  day: 19,
  part1: parsed => solve(parsed).reduce((a, c) => a + !!c, 0),
  part1expected,
  part2: parsed => solve(parsed).reduce((a, c) => a + c, 0),
  part2expected,
  parse,
});
