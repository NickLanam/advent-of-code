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

const followBeam = ([px, py, odir], grid) => {
  const x = px + xDir.get(odir);
  const y = py + yDir.get(odir);
  
  const char = grid[y]?.[x];
  if (!char) return [];

  const outDirs = FOLLOW_MAP.get(odir).get(char);
  return outDirs.map((dir) => ([x, y, dir]));
};

const solve = (charGrid, entryPoint) => {
  const grid = charGrid.map((row) => row.map(() => new Set()));

  // Tracking this separately cost less than counting non-empty sets at the end
  const energizedCoords = new Set();

  let beams = [entryPoint];
  while (true) {
    let hasChanges = false;
    const newBeams = [];
    for (const beam of beams) {
      const [x, y] = beam;
      if (x >= 0 && x < charGrid[0].length && y >= 0 && y < charGrid.length) {
        // Storing a number is 300ms faster on part2 than doing `${x},${y}`!
        energizedCoords.add(x * 10_000 + y);
      }
      const followed = followBeam(beam, charGrid);
      for (const f of followed) {
        // const [x, y, dir] = f; // Destructuring costs 100ms on part2!
        const inMap = grid[f[1]]?.[f[0]] ?? null;
        if (inMap) {
          if (!inMap.has(f[2])) {
            hasChanges = true;
            inMap.add(f[2]);
            newBeams.push(f);
          }
        }
      }
    }
    beams = newBeams;
    if (!hasChanges) break;
  }

  return energizedCoords.size;
};

// Start off-screen because 0,0 is a reflector in the real input
const part1 = (charGrid) => solve(charGrid, [-1, 0, 'r']);

// TODO: This still takes nearly a second on my input.
// I can make solve() even faster, but can I avoid trying every entry point?
// Could memoize the energized set of every (coordinate, beam) pair
// to avoid recalculating across or within solves, but that may be slower?
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
