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
  const parsed = lines.map(l => l.split('-'));
  let allNodes = new Set();
  const directConnections = new Map();
  for (const [l, r] of parsed) {
    allNodes.add(l);
    allNodes.add(r);
    const el = directConnections.get(l) ?? new Set();
    el.add(r);
    directConnections.set(l, el);
    const er = directConnections.get(r) ?? new Set();
    er.add(l);
    directConnections.set(r, er);
  }

  allNodes = [...allNodes];
  allNodes.sort();
  return { directConnections, allNodes };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ directConnections, allNodes }) => {
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

function findGroupsOfSize(directConnections, allNodes, size, include = new Set(), offset = 0) {
  const out = [];
  if (size <= 0) return [];

  for (let i = offset + 1; i < allNodes.length - size + 1; i++) {
    const el = allNodes[i];
    if ([...include].every(n => directConnections.get(el).has(n))) {
      if (size === 1) out.push([el]);
      else {
        const sub = findGroupsOfSize(directConnections, allNodes, size - 1, new Set([...include, el]), i);
        for (const s of sub) {
          if (s && s.length === size - 1) {
            out.push([el, ...s]);
          }
        }
      }
    }
  }
  return out;
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = ({ directConnections, allNodes }) => {
  // Binary search through the possible group sizes.
  // The highest one we end up finding is the one we should return.
  let lo = 0;
  let hi = allNodes.length;
  let best = [];
  while (hi - lo > 1) {
    const mid = Math.floor((lo + hi) / 2);
    const groups = findGroupsOfSize(directConnections, allNodes, mid);
    if (groups.length > 0) {
      lo = mid;
      best = groups;
    } else {
      hi = mid;
    }
  }

  best = best[0];
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
