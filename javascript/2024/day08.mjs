import aoc from './aoc.mjs';

/** @typedef {{ w: number, h: number, freqs: Map<string, [x: number, y: number][] }} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 14;

/** @type Part2Solution */
const part2expected = 34;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  /** @type Map<string, [x: number, y: number][]> */
  const freqs = new Map();
  const h = lines.length;
  const w = lines[0].length;
  for (let y = 0; y < h; y++) {
    const line = lines[y];
    for (let x = 0; x < w; x++) {
      const c = line[x];
      if (c !== '.') {
        const f = freqs.get(c) ?? [];
        f.push([x, y]);
        freqs.set(c, f);
      }
    }
  }
  return { w, h, freqs };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ w, h, freqs }) => {
  let antinodes = new Set();
  for (const nodes of freqs.values()) {
    for (let i = 0; i < nodes.length - 1; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const left = nodes[i], right = nodes[j];
        const dx = right[0] - left[0], dy = right[1] - left[1];
        const an1 = [left[0] - dx, left[1] - dy];
        const an2 = [right[0] + dx, right[1] + dy];
        if (an1[0] >= 0 && an1[0] < w && an1[1] >= 0 && an1[1] < h) {
          antinodes.add(an1[0] * 100 + an1[1]);
        }
        if (an2[0] >= 0 && an2[0] < w && an2[1] >= 0 && an2[1] < h) {
          antinodes.add(an2[0] * 100 + an2[1]);
        }
      }
    }
  }
  return antinodes.size;
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = ({ w, h, freqs }) => {
  let antinodes = new Set();
  for (const nodes of freqs.values()) {
    for (let i = 0; i < nodes.length - 1; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const left = nodes[i], right = nodes[j];
        const dx = right[0] - left[0], dy = right[1] - left[1];

        let tx = left[0], ty = left[1];
        while (tx >= 0 && tx < w && ty >= 0 && ty < h) {
          antinodes.add(tx * 100 + ty);
          tx -= dx;
          ty -= dy;
        }
        tx = right[0];
        ty = right[1];
        while (tx >= 0 && tx < w && ty >= 0 && ty < h) {
          antinodes.add(tx * 100 + ty);
          tx += dx;
          ty += dy;
        }
      }
    }
  }
  return antinodes.size;
};

aoc({
  year: 2024,
  day: 8,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
