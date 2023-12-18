import aoc from './aoc.mjs';
import { PriorityQueue } from './utils/priorityqueue.mjs';
import { range } from './utils/array.mjs';

const part1expected = 102;
const part2expected = 94;

const parse = lines => lines.map(line => line.split('').map(n => +n));

/** @typedef {{ x: number; y: number; dir: number; heatLoss: number }} Path */

const solve = (grid, part) => {
  const [minStraight, maxStraight] = part === 1 ? [1, 3] : [4, 10];
  const height = grid.length;
  const width = grid[0].length;

  /** @type PriorityQueue<Path> */
  const paths = new PriorityQueue(
    [2, 3].map(dir => ({ x: 0, y: 0, dir, heatLoss: 0 })),
    ({ heatLoss: a }, { heatLoss: b }) => a - b,
  );

  /** @type Set<number> */
  const visited = new Set();

  const findPath = () => {
    for (;;) {
      const { x, y, dir, heatLoss } = paths.remove();
      if (!visited.has(2 * (y * width + x) + (dir % 2))) {
        if (x === width - 1 && y === height - 1) {
          return heatLoss;
        } else {
          visited.add(2 * (y * width + x) + (dir % 2));
          addBlocks(x, y, (dir + 1) % 4, heatLoss);
          addBlocks(x, y, (dir + 3) % 4, heatLoss);
        }
      }
    }
  };
  
  /**
   * 
   * @param  {...[x: number, y: number, dir: number, heatLoss: number]} blocks
   */
  const addBlocks = (...[x, y, dir, heatLoss]) => {
    nextBlocks(x, y, dir).forEach(([x, y], i) => {
      heatLoss += grid[y][x];
      i + 1 >= minStraight && paths.add({ x, y, dir, heatLoss });
    });
  };
  
  /**
   * 
   * @param  {...[x: number, y: number, dir: number]} param0 
   * @returns 
   */
  const nextBlocks = (...[x, y, dir]) =>
    [...range(1, maxStraight + 1)]
      .map(b => {
        switch (dir) {
          case 0:
            return [x + b, y];
          case 1:
            return [x, y + b];
          case 2:
            return [x - b, y];
          case 3:
            return [x, y - b];
        }
      })
      .filter(([x, y]) => grid[y]?.[x]);
  
  return findPath();
};

const part1 = (grid) => solve(grid, 1);
const part2 = (grid) => solve(grid, 2);

aoc(2023, 17, part1, part1expected, part2, part2expected, parse);
