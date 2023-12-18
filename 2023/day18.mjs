import aoc from './aoc.mjs';
import { bold, grey } from './utils/color.mjs';

const part1expected = 62;
const part2expected = 952_408_144_115;

const DIR = { 'U': [0, -1], 'R': [1, 0], 'D': [0, 1], 'L': [-1, 0] };
const parse = (data) => {
  return data.map(line => {
    let [, dir, count, color] = line.match(/([RDLU]) (\d+) \(#([0-9abcdef]{6})\)/);
    return { dir: DIR[dir], count: +count, color };
  });
};

const part1 = (instructions) => {
  const dug = new Set(); // (`${x},${y}`) -> means it was dug
  dug.add(0);
  let loc = [0, 0];
  let minX = 0;
  let minY = 0;
  let maxX = 0;
  let maxY = 0;
  // TODO:
  // Part 2 numbers are huge - like, "this approach will run out of memory" huge.
  for (const { dir: [dx, dy], count } of instructions) {
    for (let c = 1; c <= count; c++) {
      const x = loc[0] + (c * dx);
      const y = loc[1] + (c * dy);
      dug.add(`${x},${y}`);
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
    }
    loc[0] += dx * count;
    loc[1] += dy * count;
  }

  // DEBUGGING: Print the grid as if we bothered to convert it (we didn't lol)
  // for (let y = minY - 1; y <= maxY + 1; y++) {
  //   let line = [];
  //   for (let x = minX - 1; x <= maxX + 1; x++) {
  //     const item = dug.has(`${x},${y}`);
  //     line.push(item ? '#' : '.');
  //   }
  //   console.log(line.join(''));
  // }

  // Flood fill like day 10, counting how many things we DON'T reach
  const w = maxX - minX + 1;
  const h = maxY - minY + 1;
  const explored = new Set();
  const unexplored = [[minX - 1, minY - 1]];
  while (unexplored.length) {
    const [ux, uy] = unexplored.shift();
    const uk = `${ux},${uy}`;
    if (explored.has(uk)) continue;
    explored.add(uk);
    for (const [nx, ny] of [
      [ux - 1, uy],
      [ux + 1, uy],
      [ux, uy - 1],
      [ux, uy + 1],
    ]) {
      // Don't look past the outer ring
      const nk = `${nx},${ny}`;
      const outOfBounds = (nx < minX - 1 || nx > maxX + 1 || ny < minY - 1 || ny > maxY + 1);
      const seen = explored.has(nk);
      const blocked = dug.has(nk);
      if (!outOfBounds && !seen && !blocked) {
        unexplored.push([nx, ny]);
      }
    }
  }

  return (w + 2) * (h + 2) - explored.size;
};

const part2 = (data) => {
  return 'NYI';
};

aoc(2023, 18, part1, part1expected, part2, part2expected, parse);
