import aoc from './aoc.mjs';

const part1expected = 46;
const part2expected = 51;

const parse = lines => lines.map(l => l.split(''));

const xDir = new Map([['u', 0], ['r', 1], ['d', 0], ['l', -1]]);
const yDir = new Map([['u', -1], ['r', 0], ['d', 1], ['l', 0]]);

const FOLLOW_MAP = new Map([
  ['u', new Map([
    ['.', ['u']],
    ['-', ['l', 'r']],
    ['|', ['u']],
    ['/', ['r']],
    ['\\', ['l']],
  ])],
  ['r', new Map([
    ['.', ['r']],
    ['-', ['r']],
    ['|', ['u', 'd']],
    ['/', ['u']],
    ['\\', ['d']],
  ])],
  ['d', new Map([
    ['.', ['d']],
    ['-', ['l', 'r']],
    ['|', ['d']],
    ['/', ['l']],
    ['\\', ['r']],
  ])],
  ['l', new Map([
    ['.', ['l']],
    ['-', ['l']],
    ['|', ['u', 'd']],
    ['/', ['d']],
    ['\\', ['u']],
  ])],
]);

const solve = (charGrid, entryPoint) => {
  const grid = charGrid.map((row) => row.map(() => new Set()));

  // Tracking this separately is faster than counting non-empty sets at the end
  const energizedCoords = new Set();

  const beams = [entryPoint];
  while (beams.length) {
    const [x, y, dir] = beams.pop();
    if (x >= 0 && x < charGrid[0].length && y >= 0 && y < charGrid.length) {
      // Storing a number is 300ms faster on part2 than doing `${x},${y}`!
      energizedCoords.add(x * 1_000 + y);
    }
    const fx = x + xDir.get(dir);
    const fy = y + yDir.get(dir);
    const fc = charGrid[fy]?.[fx];
    if (fc) {
      for (const ndir of FOLLOW_MAP.get(dir).get(fc)) {
        const stored = grid[fy]?.[fx] ?? null;
        if (stored) {
          if (!stored.has(ndir)) {
            stored.add(ndir);
            beams.push([fx, fy, ndir]);
          }
        }
      }
    }
  }

  return energizedCoords.size;
};

// Start off-screen because 0,0 is a reflector in the real input
const part1 = (charGrid) => solve(charGrid, [-1, 0, 'r']);

// TODO: This still takes about 550 milliseconds on my input.
// Would memoizing the energized set per (coord, dir) pair be faster?
const part2 = (charGrid) => {
  let best = -Infinity;
  for (let x = 0; x < charGrid[0].length; x++) {
    best = Math.max(
      best,
      solve(charGrid, [x, -1, 'd']),
      solve(charGrid, [x, charGrid.length, 'u']),
    );
  }
  for (let y = 0; y < charGrid.length; y++) {
    best = Math.max(
      best,
      solve(charGrid, [-1, y, 'r']),
      solve(charGrid, [charGrid[0].length, y, 'l']),
    );
  }
  return best;
};

aoc(2023, 16, part1, part1expected, part2, part2expected, parse);
