import aoc from './aoc.mjs';

/** @typedef {[l: string, r: string][]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {string} Part2Solution */

/** @type Part1Solution */
const part1expected = 7;

/** @type Part2Solution */
const part2expected = 'co,de,ka,ta';

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  return lines.map(l => l.split('-'));
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = (parsed) => {
  const directConnections = new Map();
  for (const [l, r] of parsed) {
    const el = directConnections.get(l) ?? new Set();
    el.add(r);
    directConnections.set(l, el);
    const er = directConnections.get(r) ?? new Set();
    er.add(l);
    directConnections.set(r, er);
  }

  const allNodes = [...new Set(parsed.flat())];
  const threes = new Set();
  for (let i = 0; i < allNodes.length - 2; i++) {
    for (let j = i + 1; j < allNodes.length - 1; j++) {
      for (let k = j + 1; k < allNodes.length; k++) {
        const d = directConnections.get(allNodes[i]);
        if (d.has(allNodes[j]) && d.has(allNodes[k]) && directConnections.get(allNodes[j]).has(allNodes[k])) {
          let next = [allNodes[i], allNodes[j], allNodes[k]];
          if (next.some(nn => nn.startsWith('t'))) {
            next.sort();
            const nk = next.join(',');
            threes.add(nk);
          }
        }
      }
    }
  }
  return threes.size;
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (parsed) => {
  const directConnections = new Map();
  for (const [l, r] of parsed) {
    const el = directConnections.get(l) ?? new Set();
    el.add(r);
    directConnections.set(l, el);
    const er = directConnections.get(r) ?? new Set();
    er.add(l);
    directConnections.set(r, er);
  }

  const allNodes = [...new Set(parsed.flat())];
  allNodes.sort();

  // Binary search through the possible group sizes.
  // The highest one we end up finding is the one we should return.
  let lo = 0;
  let hi = allNodes.length;
  let best = [];
  // TODO: Have to make my part 1 into a dynamic programming solution that takes the target size.
  //  With that, the binary search here will be easy.
  //  Making that is the hard part though.
  best.sort();
  return best.join(',');
};

aoc({
  year: 2024,
  day: 23,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
