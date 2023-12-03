import { sum } from './utils/array.mjs';

(await import('./aoc.mjs')).default(
  2023, 3,
  (data) => {
    const grid = data.map(l => l.split(''));
    let foundPartNumbers = [];
    let digitY = 0;
    let digitXs= [];
    for (let y = 0; y < grid.length; y++) {
      for (let x = 0; x < grid[y].length; x++) {
        if (!Number.isNaN(+grid[y][x])) {
          digitY = y;
          digitXs.push(x);
        } else if (digitXs.length) {
          const left = digitXs[0];
          const right = digitXs[digitXs.length - 1];
          const num = +grid[digitY].slice(left, right + 1).join('');
          if (Number.isNaN(num)) {
            console.error('Missed a spot', {y,left,right,num});
            throw new Error();
          }
          let good = false;
          neighborCheck: for (let ly = digitY - 1; ly <= digitY + 1; ly++) {
            for (let lx = left - 1; lx <= right + 1; lx++) {
              if (grid[ly]?.[lx] == null) continue;
              let c = grid[ly][lx];
              if (Number.isNaN(+c) && c !== '.') {
                good = true;
                break neighborCheck;
              }
            }
          }
          if (good) {
            foundPartNumbers.push(num);
          }
          digitXs = [];
        }
      }
    }
    return sum(foundPartNumbers);
  }, 4361,
  (data) => {
    const grid = data.map(l => l.split(''));
    const gears = {};

    // First pass: find the gears.
    for (let gy = 0; gy < grid.length; gy++) {
      for (let gx = 0; gx < grid[gy].length; gx++) {
        if (grid[gy][gx] === '*') gears[`${gx},${gy}`] = [];
      }
    }

    // Second pass: find numbers, and if they are adjacent to a gear, add them to that gear's memory
    // Very similar to part 1, but the innermost loop is different enough to write it out again.
    let digitY = 0;
    let digitXs= [];
    for (let y = 0; y < grid.length; y++) {
      for (let x = 0; x < grid[y].length; x++) {
        if (!Number.isNaN(+grid[y][x])) {
          digitY = y;
          digitXs.push(x);
        } else if (digitXs.length) {
          const left = digitXs[0];
          const right = digitXs[digitXs.length - 1];
          const num = +grid[digitY].slice(left, right + 1).join('');
          if (Number.isNaN(num)) {
            console.error('Missed a spot', {y,left,right,num});
            throw new Error();
          }
          for (let ly = digitY - 1; ly <= digitY + 1; ly++) {
            for (let lx = left - 1; lx <= right + 1; lx++) {
              if (grid[ly]?.[lx] == null) continue;
              let c = grid[ly][lx];
              if (c === '*') {
                gears[`${lx},${ly}`].push(num);
              }
            }
          }
          digitXs = [];
        }
      }
    }

    // Third pass: for each gear, if it is adjacent to two numbers, multiply them together
    let out = 0;
    for (const block of Object.values(gears)) {
      if (block.length === 2) out += block[0] * block[1];
    }
    
    return out;
  }, 467835,
  data => data
);
