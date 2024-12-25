import aoc from './aoc.mjs';
import { bold, green, yellow } from './utils/color.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number|'NYI'} Part1Solution */
/** @typedef {string} Part2Solution */

/** @type Part1Solution */
const part1expected = 3;

/** @type Part2Solution */
const part2expected = `${yellow('★')}🎄${bold(green('Happy Holidays'))}🎄${yellow('★')}`;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  if (forPart === 2) return [];
  const blocks = lines.join('\n').split('\n\n').map(b => b.split('\n'));
  const locks = [];
  const keys = [];
  const h = blocks[0].length - 1;
  const w = blocks[0][0].length;
  for (const block of blocks) {
    const isKey = block[0].startsWith('.');
    if (!isKey) block.reverse();
    const code = Array(w).fill(0).map((_, c) => {
      return h - block.findIndex(row => row.substring(c, c + 1) === '#');
    });
    if (isKey) keys.push(code);
    else locks.push(code);
  }
  return { locks, keys, w, h };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const solve = ({ locks, keys, h }) => {
  let matchedPairs = 0;
  for (const key of keys) {
    for (const lock of locks) {
      if (key.every((k, c) => k + lock[c] < h)) {
        matchedPairs++;
      }
    }
  }
  return matchedPairs;
};

aoc({
  year: 2024,
  day: 25,
  part1: solve,
  part1expected,
  part2: () => part2expected,
  part2expected,
  parse,
});
