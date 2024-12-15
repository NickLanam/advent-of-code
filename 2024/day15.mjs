import aoc from './aoc.mjs';
import { lineOfSight } from './data-structures/grid2d.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 10092;

/** @type Part2Solution */
const part2expected = 9021;

const toKey = (x, y) => x * 1_000 + y;
const fromKey = (k) => [Math.floor(k / 1_00) / 10, k % 500];

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  const splitPoint = lines.indexOf('');
  const grid = lines.slice(0, splitPoint).map(l => l.split(''));
  const path = lines.slice(splitPoint + 1).map(l => l.split('').map(arrow => ({'<': 'W', '>': 'E', '^': 'N', 'v': 'S'}[arrow]))).flat();

  if (forPart === 1) {
    const botY = grid.findIndex(row => row.includes('@'));
    const botX = grid[botY].indexOf('@');
    grid[botY][botX] = '.';
    return {
      grid,
      path,
      bot: { x: botX, y: botY },
      w: grid[0].length,
      h: grid.length,
    };
  } else {
    const boxes = new Set();
    const walls = new Set();
    let botX = -1;
    let botY = -1;
    for (let y = 0; y < grid.length; y++) {
      for (let x = 0; x < grid.length; x++) {
        switch (grid[y][x]) {
          case '.': break;
          case '@':
            botX = x;
            botY = y;
            break;
          case '#':
            walls.add(toKey(x, y));
            break;
          case 'O':
            boxes.add(toKey(x, y));
            break;
          default:
            throw new Error('How did you do that? ' + grid[y][x]);
        }
      }
    }
    return {
      path,
      boxes,
      walls,
      bot: { x: botX, y: botY },
      w: grid[0].length,
      h: grid.length,
    };
  }
};

function scorePart1(grid, w, h) {
  let boxPoints = 0;
  for (let py = 0; py < h; py++) {
    for (let px = 0; px < w; px++) {
      if (grid[py][px] === 'O') {
        boxPoints += 100 * py + px;
      }
    }
  }
  return boxPoints;
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ grid, path, bot, w, h }) => {
  for (const move of path) {
    let line = lineOfSight(grid, bot.x, bot.y, move);
    const firstWall = line.findIndex(el => el.v === '#');
    line = line.slice(0, firstWall + 1);
    if (line[0].v === '.') {
      bot.x = line[0].x;
      bot.y = line[0].y;
    } else if (line[0].v === 'O') {
      const firstSpace = line.findIndex(el => el.v === '.');
      if (firstSpace >= 1) {
        grid[line[0].y][line[0].x] = '.';
        grid[line[firstSpace].y][line[firstSpace].x] = 'O';
        bot.x = line[0].x;
        bot.y = line[0].y;
      }
    }
  }
  return scorePart1(grid, w, h);
};

function neighborTree(walls, boxes, xStart, yStart, dir) {
  let connectedKeys = new Set();
  let hitsWall = false;
  let dx = {'N': 0, 'S' : 0, 'W': -1, 'E': 1}[dir];
  let dy = {'N': -1, 'S' : 1, 'W': 0, 'E': 0}[dir];

  let toCheck = [];
  // To avoid special casing all over the place below, start by figuring out what the bot is
  // touching in its target direction
  const isBotOnInteger = xStart % 1 === 0;
  switch (dir) {
    case 'N':
    case 'S': {
      const VKey = toKey(xStart, yStart + dy);
      const VWKey = toKey(xStart - 0.5, yStart + dy);
      const VEKey = toKey(xStart + 0.5, yStart + dy);
      if (isBotOnInteger ? walls.has(VKey) : walls.has(VWKey)) {
        return { connectedKeys, hitsWall: true };
      } else {
        if (boxes.has(VKey)) {
          toCheck.push(VKey);
        } else {
          if (boxes.has(VWKey)) toCheck.push(VWKey);
        }
      }
      break;
    }
    case 'E': {
      const EKey = toKey(xStart + dx / 2, yStart);
      const EFarKey = toKey(xStart + dx, yStart);
      if (walls.has(EKey)) {
        return { connectedKeys, hitsWall: true };
      } else {
        if (boxes.has(EKey)) toCheck.push(EKey);
      }
      break;
    }
    case 'W': {
      const WKey = toKey(xStart + dx / 2, yStart);
      const WFarKey = toKey(xStart + dx, yStart);
      if (walls.has(WFarKey)) {
        return { connectedKeys, hitsWall: true };
      } else {
        if (boxes.has(WFarKey)) toCheck.push(WFarKey);
      }
      break;
    }
  }

  for (const tc of toCheck) {
    // Initial conditions.
    connectedKeys.add(tc);
  }

  // Now, we're only looking at boxes, not at the bot
  while (toCheck.length > 0) {
    const k = toCheck.pop();
    const [x, y] = fromKey(k);

    if (!boxes.has(k)) {
      throw new Error(`Tried to check on a box that is not present at ${x}, ${y}`);
    }

    // Complexity:
    // - If looking left/right, just look at one neighbor for wall/box.
    //   - If box, add to connections and to toCheck.
    //   - If wall, hitsWall = true, continue
    // - If looking up/down, varies by even or half:
    //  - If even, look up/down as well as same 0.5 to the RIGHT for box, just plain up/down for wall
    //  - If half, look up/down but 0.5 to the LEFT for wall
    //  - Both cases: look for box in up/down spot. If found, add it to connectedKeys and to toCheck
    //   - If not found, check 0.5 left and 0.5 right
    //   - Repeat this process offset 0.5 to the right (for the other half of the box we're looking at)
    const isBoxOnInteger = x % 1 === 0;
    let dx = {'N': 0, 'S' : 0, 'W': -1, 'E': 1}[dir];
    let dy = {'N': -1, 'S' : 1, 'W': 0, 'E': 0}[dir];
    switch (dir) {
      case 'N':
      case 'S': {
        const VKey = toKey(x, y + dy);
        const VWKey = toKey(x - 0.5, y + dy);
        const VEKey = toKey(x + 0.5, y + dy);
        if (isBoxOnInteger ? walls.has(VKey) : (walls.has(VWKey) || walls.has(VEKey))) {
          hitsWall = true;
        } else {
          if (boxes.has(VKey)) {
            toCheck.push(VKey);
            connectedKeys.add(VKey);
          } else {
            if (boxes.has(VWKey)) {
              toCheck.push(VWKey);
              connectedKeys.add(VWKey);
            }
            if (boxes.has(VEKey)) {
              toCheck.push(VEKey);
              connectedKeys.add(VEKey);
            }
          }
        }
        break;
      }
      case 'E':
      case 'W': {
        const HCloseKey = toKey(x + dx / 2, y);
        const HFarKey = toKey(x + dx, y);
        if (walls.has(HFarKey)) {
          hitsWall = true;
        } else {
          if (boxes.has(HCloseKey)) {
            toCheck.push(HCloseKey);
            connectedKeys.add(HCloseKey);
          }
          if (boxes.has(HFarKey)) {
            toCheck.push(HFarKey);
            connectedKeys.add(HFarKey);
          }
        }
      }
    }
  }
  return { connectedKeys, hitsWall };
}

function scorePart2(boxes, w, h) {
  let score = 0;
  for (const k of boxes) {
    const [x, y] = fromKey(k);
    score += y * 100 + x * 2;
  }
  return score;
}

function printPart2Grid({ w, h, bot, walls, boxes }) {
  for (let y = 0; y < h; y++) {
    let line = '';
    for (let x = 0; x < w; x+= 0.5) {
      if (walls.has(toKey(Math.floor(x), y))) {
        line += '#';
      } else if (bot.x === x && bot.y === y) {
        line += '@';
      } else if (boxes.has(toKey(x, y))) {
        line += '[';
      } else if (boxes.has(toKey(x - 0.5, y))) {
        line += ']';
      } else {
        line += '.';
      }
    }
    console.log(line);
  }
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = ({ w, h, walls, boxes, path, bot }) => {
  for (const move of path) {
    const { connectedKeys, hitsWall } = neighborTree(walls, boxes, bot.x, bot.y, move);
    

    let dx = {'N': 0, 'S' : 0, 'W': -0.5, 'E': 0.5}[move];
    let dy = {'N': -1, 'S' : 1, 'W': 0, 'E': 0}[move];

    if (!hitsWall) {
      // Move the bot first, then all 
      bot.x += dx;
      bot.y += dy;
      const newBoxes = [...connectedKeys].map(k => {
        const [x, y] = fromKey(k);
        return toKey(x + dx, y + dy);
      });
      // Two separate steps since we may have overlapping lines (don't clobber!)
      for (const oldKey of connectedKeys) {
        boxes.delete(oldKey);
      }
      for (const newKey of newBoxes) {
        boxes.add(newKey);
      }
    }
  }
  // printPart2Grid({w, h, bot, walls, boxes});
  return scorePart2(boxes, w, h);
};

aoc({
  year: 2024,
  day: 15,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
