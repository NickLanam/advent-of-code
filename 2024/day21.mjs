import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Solution */

/** @type Solution */
const part1expected = 126384;

/** @type Solution */
const part2expected = 154115708116294;

const toKey = (x, y) => x * 100 + y;
const fromKey = (k) => [Math.round(k / 100), k % 100];

const DPAD_MOVE_TO_BUTTON = new Map([
  [toKey( 0, -1), '^'],
  [toKey( 1,  0), '>'],
  [toKey( 0,  1), 'v'],
  [toKey(-1,  0), '<']
]);

/*
 * +---+---+---+
 * | 7 | 8 | 9 |
 * +---+---+---+
 * | 4 | 5 | 6 |
 * +---+---+---+
 * | 1 | 2 | 3 |
 * +---+---+---+
 *     | 0 | A |
 *     +---+---+
 */
const numPad = new Map([
  [toKey(0, 0), '7'],
  [toKey(1, 0), '8'],
  [toKey(2, 0), '9'],
  [toKey(0, 1), '4'],
  [toKey(1, 1), '5'],
  [toKey(2, 1), '6'],
  [toKey(0, 2), '1'],
  [toKey(1, 2), '2'],
  [toKey(2, 2), '3'],
  [toKey(1, 3), '0'],
  [toKey(2, 3), 'A'],
]);
const numPadA = toKey(2, 3);

/*
 *     +---+---+
 *     | ^ | A |
 * +---+---+---+
 * | < | v | > |
 * +---+---+---+
 */
const dPad = new Map([
  [toKey(1, 0), '^'],
  [toKey(2, 0), 'A'],
  [toKey(0, 1), '<'],
  [toKey(1, 1), 'v'],
  [toKey(2, 1), '>'],
]);
const dPadA = toKey(2, 0);


/*
 * Key details from the problem description:
 *
 * - All bots start on `A` (for their keypad)
 * - You have the d-pad controlling bot 0
 * - Bot (N - 2) has a d-pad controlling bot (N - 1)
 * - Bot N has the numpad
 * - After inputting a code correctly, it HAS to be true that
 *   all bots are on `A` again (becaue each one had to tell the next
 *   to activate, and the final one had to input something).
 *   This means that the state resets itself between codes,
 *   so we don't need to track anything between them.
 */

const buttonPaths = (pos, pad, b) => {
  // We could use Dijkstra's for this, but that's overcomplicating it even though I already have
  // code for it from several previous days this year. A simple BFS works too!
  const queue = [[[], [pos]]];
  const paths = [];
  let end;
  while (queue.length) {
    const [path, visited] = queue.shift();
    const ck = visited[visited.length - 1];
    if (b === pad.get(ck)) {
      end = ck;
      paths.push(path);
      continue;
    }
    if (paths.some(([p]) => p.length < path.length)) {
      // Don't explore paths that we've already beaten
      continue;
    }
    const [cx, cy] = fromKey(ck);
    for (const [dk, db] of DPAD_MOVE_TO_BUTTON) {
      const [dx, dy] = fromKey(dk);
      const nk = toKey(cx + dx, cy + dy);
      if (pad.has(nk) && !visited.includes(nk)) {
        queue.push([
          [...path, db],
          [...visited, nk],
        ]);
      }
    }
  }
  return [paths, end];
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Solution}
 */
const solve = (codes, numBots) => {
  const cache = new Map();

  return codes.map((expect) => {
    const bots = [
      { pos: numPadA, pad: numPad },
      ...Array(numBots).fill({ pos: dPadA, pad: dPad })
    ];

    const recurse = (code, bot) => {
      const cacheKey = code.join('') + '_' + bot;
      if (cache.has(cacheKey)) return cache.get(cacheKey);
      cache.set(cacheKey, code.reduce(
        ([pos, len], b) => {
          const [paths, end] = buttonPaths(pos, bots[bot].pad, b);
          return [
            end,
            len + (bot < numBots
              ? Math.min(...paths.map(p => recurse([...p, 'A'], bot + 1)[1]))
              : paths[0].length + 1
            )
          ];
        },
        [bots[bot].pos, 0]
      ));
      return cache.get(cacheKey)
    }
    return recurse([...expect], 0);
  }).reduce((a, c, i) => a + (c[1] * parseInt(codes[i], 10)), 0);
};

aoc({
  year: 2024,
  day: 21,
  part1: (parsed) => solve(parsed, 2),
  part1expected,
  part2: (parsed) => solve(parsed, 25),
  part2expected
});
