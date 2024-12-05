import aoc from './aoc.mjs';

/** @typedef {{rules: Record<number, number[]>, updates: number[][], meetsRules: (pages: number[]) => boolean}} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 143;

/** @type Part2Solution */
const part2expected = 123;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  const splitAt = lines.indexOf('');
  const splitters = lines.slice(0, splitAt).map(l => l.split('|').map(n => +n));
  const updates = lines.slice(splitAt + 1).map(l => l.split(',').map(n => +n));

  const rules = splitters.reduce((acc, [a, b]) => ({
    ...acc, [a]: [...(acc[a] ?? []), b]
  }), {});

  const meetsRules = (pages) => pages.every((p, i) => {
    const before = pages.slice(0, i);
    return before.every(bb => !rules[p]?.includes(bb));
  });
  
  return { rules, updates, meetsRules };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ updates, meetsRules }) => {
  return updates
    .filter(u => meetsRules(u))
    .map(u => u[Math.floor(u.length / 2)])
    .reduce((a, c) => a + c, 0);
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = ({ rules, updates, meetsRules }) => {
  return updates
    .filter(u => !meetsRules(u))
    .map(u => {
      const out = [...u];
      out.sort((a, b) => {
        if (rules[a]?.includes(b)) return -1;
        else if (rules[b]?.includes(a)) return 1;
        return 0;
      });
      return out;
    })
    .map(u => u[Math.floor(u.length / 2)])
    .reduce((a, c) => a + c, 0);
};

aoc({
  year: 2024,
  day: 5,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
