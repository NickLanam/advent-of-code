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
    const bits = inputs.length - 1;
    for (let i = 0; i < numOps**bits; i++) {
      // TODO: There's a massive speedup to be had by memoizing sub-solutions (dynamic programming).
      //  Do that before day 8 opens up, to refresh my memory on the best Advent-of-Code-friendly approach.
      const combo = i.toString(numOps).padStart(bits, '0').split('').map(d => ['+', '*', '.'][+d]);
      let result = inputs[0];
      for (let j = 0; j < combo.length; j++) {
        switch (combo[j]) {
          case '+': result += inputs[j + 1]; break;
          case '*': result *= inputs[j + 1]; break;
          case '.': result = Number(String(result) + String(inputs[j + 1])); break;
          default: throw new Error('How? ' + combo[j]);
        }
      }
      if (result === expected) {
        return expected;
      }
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
