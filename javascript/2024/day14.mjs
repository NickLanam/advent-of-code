import aoc from './aoc.mjs';

/** @typedef {number[][]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 12;

/** @type Part2Solution */
const part2expected = 'SKIP';

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return lines.map(l => [...l.matchAll(/[-\d]+/g)].map(([d]) => +d));
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @param {boolean} isSample 
 * @returns {Part1Solution}
 */
const part1 = (parsed, isSample) => {
  const w = isSample ? 11 : 101;
  const h = isSample ?  7 : 103;
  const numSteps = 100;
  const bots = [];
  for (const [px, py, vx, vy] of parsed) {
    bots.push([
      (((w + px + vx * numSteps) % w) + w) % w,
      (((h + py + vy * numSteps) % h) + h) % h,
    ]);
  }
  let q0 = bots.filter(([x, y]) => x < Math.floor(w / 2) && y < Math.floor(h / 2)).length;
  let q1 = bots.filter(([x, y]) => x > Math.floor(w / 2) && y < Math.floor(h / 2)).length;
  let q2 = bots.filter(([x, y]) => x < Math.floor(w / 2) && y > Math.floor(h / 2)).length;
  let q3 = bots.filter(([x, y]) => x > Math.floor(w / 2) && y > Math.floor(h / 2)).length;
  return q0 * q1 * q2 * q3;
};

/**
 * 
 * @param {ParsedInput} parsed 
 * * @param {boolean} isSample 
 * @returns {Part2Solution}
 */
const part2 = (parsed, isSample) => {
  if (isSample) return 'SKIP';
  const w = isSample ? 11 : 101;
  const h = isSample ?  7 : 103;
  // TODO: There's a neat trick here: it's one of the only states where there are ZERO overlapping bots,
  //  AND there's a rectangle drawn around the tree (dotted top/bottom, solid sides),
  //  AND we can assume that the generator probably started with the solution and randomly jettisoned bots
  //      in different directions for ($solution) steps
  //  Putting those together, there's quite likely a math trick to do this in one step instead of a big loop.
  const maxSteps = 10_000;
  for (let step = 0; step < maxSteps; step++) {
    const bots = parsed.map(([px, py, vx, vy]) => [
      (((w + px + vx * step) % w) + w) % w,
      (((h + py + vy * step) % h) + h) % h,
    ]);
    for (let y = 0; y < h; y++) {
      const onLine = bots.filter(([,by]) => by === y);
      let seq = 0;
      for (let x = 0; x < w; x++) {
        const yes = onLine.some(([bx]) => bx === x);
        if (yes) seq++;
        if (seq >= 10) {
          return step;
        }
        if (!yes) seq = 0;
      }
    }
  }
  return '\x1b[1;31mFAIL\x1b[0m';
};

aoc({
  year: 2024,
  day: 14,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
