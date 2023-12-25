import aoc from './aoc.mjs';
import { bold, green, yellow, white } from './utils/color.mjs';

/** @typedef {Map<string, Set<string>>} ParsedInput */
/** @typedef {number} Solution */

/** @type Solution */
const part1expected = 54;
/** @type string */
const part2expected = `${yellow('★')}🎄${bold(green('Happy Holidays'))}🎄${yellow('★')}`;

/**
 * @param {string[][]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  if (forPart === 2) return null; // See below
  const parsed = lines.map(line => line.split(': ').map(half => half.split(' ')));
  const allConnections = new Map();
  for (const [[left], right] of parsed) {
    for (const r of right) {
      /** @type Set<string> */
      const set = allConnections.get(left) ?? new Set();
      set.add(r);
      allConnections.set(left, set);
      const rset = allConnections.get(r) ?? new Set();
      rset.add(left);
      allConnections.set(r, rset);
    }
  }
  return allConnections;
};

/**
 * @param {[string, string][]} edges
 * @param {number[]} excludeIndices
 * @returns {Set<string>[]}
 */
const getNetworks = (edges, excludeIndices = []) => {
  const networks = [];
  for (let ei = 0; ei < edges.length; ei++) {
    if (excludeIndices.includes(ei)) continue;
    const [a, b] = edges[ei];
    const neighbors = edges
      .filter((e, ej) => !excludeIndices.includes(ej) && (e.includes(a) || e.includes(b)));
    const local = [a, b, ...new Set(neighbors.flat())];
    const groupIndex = networks.findIndex(n => local.some(l => n.has(l)));
    const group = networks[groupIndex] ?? new Set();
    for (const l of local) {
      group.add(l);
    }
    if (groupIndex < 0) networks.push(group);
  }
  const combinedNetworks = [];
  for (let n = 0; n < networks.length; n++) {
    const net = networks[n];
    const existing = combinedNetworks.find(s => [...net].some(k => s.has(k)));
    if (existing) {
      for (const k of net) {
        existing.add(k);
      }
    } else {
      combinedNetworks.push(networks[n]);
    }
  }
  return combinedNetworks;
};

/**
 * Note: Correct answer takes on the order of hours with this approach.
 * I should rewrite this to use Karger's algorithm instead of brute force.
 * After I proved this works on smaller samples, I miiight have used a Python NetworkX solution
 * that solved it in a few seconds using Karger's Algorithm (internally).
 * TODO: Make this do that. Without any external dependencies!
 *
 * @param {ParsedInput} allConnections 
 * @returns {Solution}
 */
const solve = (connections) => {
  const allEdges = [];
  for (const [k, v] of connections) {
    for (const r of v) {
      if (!allEdges.some((edge) => edge.includes(k) && edge.includes(r))) {
        allEdges.push([k, r]);
      }
    }
  }

  // Karger's algorithm would do this WAY faster, but hey, if it works it works
  for (let x = 0; x < allEdges.length - 2; x++) {
    console.log(x);
    for (let y = x + 1; y < allEdges.length - 1; y++) {
      console.log(' ', y);
      for (let z = y + 1; z < allEdges.length; z++) {
        console.log('  ', z);
        const networks = getNetworks(allEdges, [x, y, z]);
        // if (networks.length !== 1) console.log(networks.map(n => n.size));
        if (networks.length === 2) {
          // console.log({ x: allEdges[x], y: allEdges[y], z: allEdges[z], networks });
          return networks[0].size * networks[1].size;
        }
      }
    }
  }
};

aoc({
  year: 2023,
  day: 25,
  part1: solve,
  part1expected,
  // Day 25 does not have a part 2. Instead, the 50th star
  // is a reward for earning the other 49.
  part2: () => part2expected,
  part2expected,
  parse,
});
