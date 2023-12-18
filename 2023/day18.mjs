import aoc from './aoc.mjs';

const part1expected = 62;
const part2expected = 952_408_144_115;

const DIR = { 'U': [0, -1], 'R': [1, 0], 'D': [0, 1], 'L': [-1, 0] };
const parse = (data, part) => {
  return data.map(line => {
    let [, dir, count, color] = line.match(/([RDLU]) (\d+) \(#([0-9abcdef]{6})\)/);
    if (part === 1) {
      return { dir: DIR[dir], count: +count };
    } else {
      count = parseInt(color.substring(0, 5), 16);
      dir = ['R', 'D', 'L', 'U'][+color.substring(5)];
      return { dir: DIR[dir], count: +count };
    }
  });
};

/**
 * An implementation of the Shoelace algorithm.
 * See: https://www.101computing.net/the-shoelace-algorithm/
 *
 * @param {{ dir: [dx: number, dy: number], count: number }[]} instructions 
 * @returns 
 */
const solve = (instructions) => {
  let prevRow;
  let prevCol;
  let row = 0;
  let col = 0;
  let colDiff = 0;
  let rowDiff = 0;
  for (const { dir: [dx, dy], count } of instructions) {
    prevRow = row;
    prevCol = col;
    row += dx * count;
    col += dy * count;
    colDiff += prevCol * row - col * prevRow;
    rowDiff += count;
  }
  return Math.abs(colDiff / 2) + (rowDiff / 2) + 1;
};

aoc(2023, 18, solve, part1expected, solve, part2expected, parse);
