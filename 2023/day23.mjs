import aoc from './aoc.mjs';

/** @typedef {number} CoordKey (x * 1_000 + y) */
/** @typedef {Map<CoordKey,{ to: CoordKey, cost: number }[]} Graph */
/** @typedef {{ graph: Graph, w: number, h: number }} ParsedInput */

/**
 * @param {number} x 
 * @param {number} y 
 * @returns {CoordKey}
 */
const coordKey = (x, y) => x * 1_000 + y;

/**
 * @param {string[][]} grid
 * @param {number} x
 * @param {number} y
 * @param {number} w
 * @param {number} h
 * @returns {CoordKey[]}
 */
const getExits = (grid, x, y, w, h) => {
  const c = grid[y][x];
  let possible = [];
  switch (c) {
    case '#': possible = []; break;
    case '.': possible = [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]]; break;
    case '>': possible = [[x + 1, y]]; break;
    case '<': possible = [[x - 1, y]]; break;
    case '^': possible = [[x, y - 1]]; break;
    case 'v': possible = [[x, y + 1]]; break;
    default: throw new Error(`Unknown char: ${c}`);
  }
  return possible.filter(([x2, y2]) => (
    // Stay in bounds
    x2 >= 0 && x2 < w && y2 >= 0 && y2 < h &&
    // Don't path into walls
    grid[y2][x2] !== '#' &&
    // Don't path into a cell that would immediately bounce back,
    // which the rules forbid (no visting the same cell twice).
    // Technically, the solution already ignores these, but it's
    // slightly faster to prune the grid ahead of time.
    !(x2 > x && grid[y2][x2] === '<') &&
    !(x2 < x && grid[y2][x2] === '>') &&
    !(y2 > y && grid[y2][x2] === '^') &&
    !(y2 < y && grid[y2][x2] === 'v')
  )).map(([x3, y3]) => coordKey(x3, y3));
};

/**
 * When there's a run of `.` nodes that only connect to each other with no branches,
 * combine them and sum their costs as a single node.
 *
 * This is just to make the graph smaller so that DFS has less work to do.
 * It doesn't change the answer, but it DOES make the solution run 20x faster!
 *
 * @param {Graph} graph 
 */
const compactCorridors = (graph) => {
  while (true) {
    let didSimplify = false;
    simplifier: for (const [from, exits] of graph) {
      for (let i = 0; i < exits.length; i++) {
        const { to, cost } = exits[i];
        if (!graph.has(to)) continue;
        const nextExits = graph.get(to);
        if (nextExits.length !== 2) continue;
        if (!nextExits.some(({ to: nextTo }) => nextTo === from)) continue;

        // Mark the process to be continued
        didSimplify = true;

        // Find where the node to be removed leads
        const { to: target, cost: targetCost } = nextExits.find(
          ({ to: nextTo }) => nextTo !== from
        );

        // Remove the node in the middle
        graph.delete(to);

        // Edit the source node to jump past the removed node, adding the costs together
        exits.splice(i, 1, { to: target, cost: cost + targetCost });
        graph.set(from, exits);

        // Make sure to do the same for the target, if it pointed back to the removed node
        const targetExits = graph.get(target);
        for (let j = 0; j < targetExits.length; j++) {
          if (targetExits[j].to === to) {
            // Costs are the same in both directions, so this works.
            targetExits.splice(j, 1, { to: from, cost: cost + targetCost});
            graph.set(target, targetExits);
          }
        }

        // Then restart the process until we stop making changes
        break simplifier;
      }
    }
    if (!didSimplify) break;
  }
};

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  const grid = lines.map(line => line.split(''));
  const w = grid[0].length;
  const h = grid.length;

  // In part 2, we can walk up slopes.
  // The rest of the challenge is unchanged.
  // Mutating the grid to turn slopes to paths is enough to handle this.
  if (forPart === 2) {
    for (let x = 0; x < w; x++) {
      for (let y = 0; y < h; y++) {
        if (grid[y][x] !== '#') grid[y][x] = '.';
      }
    }
  }

  /** @type Graph */
  const graph = new Map();
  for (let x = 0; x < w; x++) {
    for (let y = 0; y < h; y++) {
      const k = coordKey(x, y);
      const exits = getExits(grid, x, y, w, h);
      // No point holding onto nodes that have no exit
      if (exits.length > 0) {
        graph.set(k, exits.map(to => ({ to, cost: 1 })));
      }
    }
  }

  // If we have a chain of paths with no branches, merge them as if they were
  // only the first and last of those paths with a cost equal to the removals.
  // If we do this, solution takes 1.3ms for part 1 and 3.5sec for part 2.
  // If we don't do this, 23.7ms and (minutes). It's quite the difference!
  compactCorridors(graph);

  return { graph, w, h };
};

/**
 * A pathfinding puzzle, not unlike day 10. Rules:
 * - We want the length of the LONGEST legal path from [1, 0] to [w - 2, h - 1]
 * - Cannot use the same node twice
 * - The only exit neighbor from a ><^v character is the one it points to
 *   Combined with rule above, means three entrances, one exit for these
 *   This rule only matters for part 1; in part 2 we transform the slopes to paths
 * - # is unpathable
 * - . is fully pathable (four entrances, four exits, but no backtracking still)
 *
 * @param {ParsedInput} parsed Already has the part 2 change in it if needed
 * @returns {number}
 */
const solve = ({ graph, w, h }) => {
  const start = coordKey(1, 0);
  const goal = coordKey(w - 2, h - 1);

  function longestPath(current, totalCost, seen) {
    if (current === goal) return totalCost;

    let longest = 0;
    seen.add(current);
    for (const { to, cost } of graph.get(current)) {
      if (seen.has(to)) continue;
      longest = Math.max(longest, longestPath(to, totalCost + cost, seen));
    }
    seen.delete(current);
    return longest;
  }

  return longestPath(start, 0, new Set());
};

aoc({
  year: 2023,
  day: 23,
  part1: (parsedWithSlopes) => solve(parsedWithSlopes),
  part1expected: 94,
  part2: (parsedWithoutSlopes) => solve(parsedWithoutSlopes),
  part2expected: 154,
  parse,
});
