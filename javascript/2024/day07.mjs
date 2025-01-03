import aoc from './aoc.mjs';

/** @typedef {{ expected: number, inputs: number[]}[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 3749;

/** @type Part2Solution */
const part2expected = 11387;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return lines.map(line => {
    const [l, r] = line.split(': ');
    return { expected: +l, inputs: r.split(' ').map(n => +n) };
  });
};

function solve(parsed, numOps) {
  return parsed.map(({expected, inputs}) => {
    let stack = [inputs[0]];
    for (let layer = 1; layer <= inputs.length; layer++) {
      const next = [];
      for (const v of stack) {
        if (v === expected) return expected;
        if (v > expected) continue;
        if (layer < inputs.length) {
          next.push(v + inputs[layer]);
          next.push(v * inputs[layer]);
          if (numOps === 3) next.push(v * 10**String(inputs[layer]).length + inputs[layer]);
        }
      }
      stack = next;
    }
    return 0;
  }).reduce((a, c) => a + c, 0);
}

aoc({
  year: 2024,
  day: 7,
  part1: parsed => solve(parsed, 2),
  part1expected,
  part2: parsed => solve(parsed, 3),
  part2expected,
  parse,
});
