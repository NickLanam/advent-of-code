import aoc from './aoc.mjs';

const part1expected = 46;
const part2expected = 51;

const parse = lines => lines.map(l => l.split(''));

const xDir = new Map([['u', 0], ['r', 1], ['d', 0], ['l', -1]]);
const yDir = new Map([['u', -1], ['r', 0], ['d', 1], ['l', 0]]);
const followBeam = ({ x, y, dir }, grid) => {
  const out = [];
  const dx = xDir.get(dir);
  const dy = yDir.get(dir);

  const char = grid[y + dy]?.[x + dx] ?? null;
  switch (char) {
    case null:
      // Beam just dissipates if it hits a wall
      break;
    case '.':
      // Beam passes through if it reaches a dot
      out.push({ x: x + dx, y: y + dy, dir });
      break;
    case '-':
      if (dx === 0) {
        // Spit out of both pointy ends
        out.push({ x: x + dx, y: y + dy, dir: 'l' });
        out.push({ x: x + dx, y: y + dy, dir: 'r' });
      } else {
        // Pass right through
        out.push({ x: x + dx, y: y + dy, dir });
      }
      break;
    case '|':
      if (dy === 0) {
        // Spit out of both pointy ends
        out.push({ x: x + dx, y: y + dy, dir: 'u' });
        out.push({ x: x + dx, y: y + dy, dir: 'd' });
      } else {
        // Pass right through
        out.push({ x: x + dx, y: y + dy, dir });
      }
      break;
    case '/':
      if (dx === -1) {
        out.push({x: x + dx, y: y + dy, dir: 'd' });
      } else if (dx === 1) {
        out.push({x: x + dx, y: y + dy, dir: 'u' });
      } else if (dy === -1) {
        out.push({x: x + dx, y: y + dy, dir: 'r' });
      } else if (dy === 1) {
        out.push({x: x + dx, y: y + dy, dir: 'l' });
      } else {
        throw new Error('Bad fallthrough logic on char /');
      }
      break;
    case '\\':
      if (dx === -1) {
        out.push({x: x + dx, y: y + dy, dir: 'u' });
      } else if (dx === 1) {
        out.push({x: x + dx, y: y + dy, dir: 'd' });
      } else if (dy === -1) {
        out.push({x: x + dx, y: y + dy, dir: 'l' });
      } else if (dy === 1) {
        out.push({x: x + dx, y: y + dy, dir: 'r' });
      } else {
        throw new Error('Bad fallthrough logic on char \\');
      }
      break;
    default:
      throw new Error('Missed case: ' + char);
  }
  return out.filter(({x, y}) => x >= 0 && x < grid[0].length && y >= 0 && y < grid.length);
};

const solve = (charGrid, entryPoint) => {
  const grid = charGrid.map((row, y) => row.map((char, x) => ({ x, y, char, dirs: new Set() })));
  const energizedCoords = new Set(); // To simplify answering later

  let beams = [entryPoint];
  while (true) {
    let hasChanges = false;
    const newBeams = [];
    for (const beam of beams) {
      if (beam.x >= 0 && beam.x < charGrid[0].length && beam.y >= 0 && beam.y < charGrid.length) {
        energizedCoords.add(`${beam.x},${beam.y}`);
      }
      const followed = followBeam(beam, charGrid);
      for (const f of followed) {
        const { x, y, dir } = f;
        const inMap = grid[y]?.[x] ?? null;
        if (inMap) {
          if (!inMap.dirs.has(dir)) {
            hasChanges = true;
            inMap.dirs.add(dir);
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
const part1 = (grid) => solve(grid, { x: -1, y: 0, dir: 'r' });

// TODO: This eats up over a second of runtime, by brute forcing all options.
// There is likely a clever optimization to get under 100ms.
const part2 = (grid) => {
  let best = -Infinity;
  for (let x = 0; x < grid[0].length; x++) {
    best = Math.max(
      best,
      solve(grid, { x, y: -1, dir: 'd' }),
      solve(grid, { x, y: grid.length, dir: 'u' }),
    );
  }
  for (let y = 0; y < grid.length; y++) {
    best = Math.max(
      best,
      solve(grid, { x: -1, y, dir: 'r' }),
      solve(grid, { x: grid[0].length, y, dir: 'l' }),
    );
  }
  return best;
};

aoc(2023, 16, part1, part1expected, part2, part2expected, parse);
