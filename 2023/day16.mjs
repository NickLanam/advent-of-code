import aoc from './aoc.mjs';

const part1expected = 46;
const part2expected = 51;

// Const ints instead of one-character strings saves ~50ms on part 2.
const UP = 0;
const RIGHT = 1;
const DOWN = 2;
const LEFT = 3;

const DOT = 0;
const DASH = 1;
const PIPE = 2;
const FSLASH = 3;
const BSLASH = 4;

const CHARS = new Map([['.', DOT], ['-', DASH], ['|', PIPE], ['/', FSLASH], ['\\', BSLASH]]);
const parse = lines => lines.map(l => l.split('').map(c => CHARS.get(c)));

// These were maps, but as arrays they save ~100ms on part 2.
const X_DIR = [0, 1, 0, -1];
const Y_DIR = [-1, 0, 1, 0];
const FOLLOW_RES = [
  [[UP], [LEFT, RIGHT], [UP], [RIGHT], [LEFT]],
  [[RIGHT], [RIGHT], [UP, DOWN], [UP], [DOWN]],
  [[DOWN], [LEFT, RIGHT], [DOWN], [LEFT], [RIGHT]],
  [[LEFT], [LEFT], [UP, DOWN], [DOWN], [UP]],
];

const solve = (charGrid, entryPoint) => {
  // Each cell is an array of seen directions out of that cell.
  // It was a Set(), but that's slower than an array with so few elements.
  const grid = charGrid.map((row) => row.map(() => []));

  // Tracking this separately is faster than counting non-empty sets at the end
  // Since this can have thousands of elements, it IS faster than an array.
  const energizedCoords = new Set();

  const beams = [entryPoint];
  while (beams.length) {
    const [x, y, dir] = beams.pop();
    if (x >= 0 && x < charGrid[0].length && y >= 0 && y < charGrid.length) {
      // Storing a number is 300ms faster on part2 than doing `${x},${y}`!
      energizedCoords.add(x * 1_000 + y);
    }
    const fx = x + X_DIR[dir];
    const fy = y + Y_DIR[dir];
    const fc = charGrid[fy]?.[fx];
    if (fc != null) {
      for (const ndir of FOLLOW_RES[dir][fc]) {
        const stored = grid[fy]?.[fx];
        if (stored) {
          if (!stored.includes(ndir)) {
            stored.push(ndir);
            beams.push([fx, fy, ndir]);
          }
        }
      }
    }
  }

  return energizedCoords.size;
};

// Start off-screen because 0,0 is a reflector in the real input
const part1 = (charGrid) => solve(charGrid, [-1, 0, RIGHT]);

// TODO: This still takes on average 350 milliseconds on my input.
// Would memoizing the energized set per (coord, dir) pair be faster?
const part2 = (charGrid) => {
  const w = charGrid[0].length;
  const h = charGrid.length;
  let best = -Infinity;
  for (let x = 0; x < w; x++) {
    best = Math.max(
      best,
      solve(charGrid, [x, -1, DOWN]),
      solve(charGrid, [x, h, UP]),
    );
  }
  for (let y = 0; y < h; y++) {
    best = Math.max(
      best,
      solve(charGrid, [-1, y, RIGHT]),
      solve(charGrid, [w, y, LEFT]),
    );
  }
  return best;
};

aoc(2023, 16, part1, part1expected, part2, part2expected, parse);
