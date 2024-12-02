import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 11;

/** @type Part2Solution */
const part2expected = 31;

/**
 * Sum the pairwise differences of the least, next-least, etc inputs in each column.
 *
 * @param {ParsedInput} lines
 * @returns {Part1Solution}
 */
const part1 = (lines) => {
  const length = lines.length;
  const left = Array(length).fill(0);
  const right = Array(length).fill(0);
  for (const i in lines) {
    const [l, r] = lines[i].split('   ');
    left[i] = Number(l);
    right[i] = Number(r);
  }
  left.sort();
  right.sort();

  let sum = 0;
  for (let i = 0; i < length; i++) {
    sum += Math.abs(left[i] - right[i]);
  }
  return sum;
};

/**
 * For each left value, multiply by how often it shows up in the right column, then sum those.
 *
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (lines) => {
  const lefts = Array(lines.length);
  let freqMap = new Map();
  for (const i in lines) {
    const [l, r] = lines[i].split('   ');
    lefts[i] = Number(l);
    const right = Number(r);
    const val = freqMap.get(right) ?? 0;
    freqMap.set(right, val + 1);
  }
  
  let sum = 0;
  for (const l of lefts) {
    sum += l * (freqMap.get(l) ?? 0);
  }
  return sum;
};

aoc({
  year: 2024,
  day: 1,
  part1,
  part1expected,
  part2,
  part2expected,
});
