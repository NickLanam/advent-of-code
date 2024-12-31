const { getInput, fromRaw } = require('./utils');
const input = getInput(12).map(inst => [inst[0], Number(inst.substr(1))]);
const sample = `F10
N3
F7
R90
F11`.split('\n').map(inst => [inst[0], Number(inst.substr(1))]);

const which = input;

function stepStar1(x, y, dir, inst, arg) {
  switch (inst) {
    case 'N':
      return [x, y - arg, dir];
    case 'S':
      return [x, y + arg, dir];
    case 'E':
      return [x + arg, y, dir];
    case 'W':
      return [x - arg, y, dir];
    case 'L':
      return [x, y, (dir + arg + 360) % 360];
    case 'R':
      return [x, y, (dir - arg + 360) % 360];
    case 'F':
      switch (dir) {
        case 0:
          return stepStar1(x, y, dir, 'E', arg);
        case 90:
          return stepStar1(x, y, dir, 'N', arg);
        case 180:
          return stepStar1(x, y, dir, 'W', arg);
        case 270:
          return stepStar1(x, y, dir, 'S', arg);
        default:
          throw new Error('Non-cardinal direction: ' + dir);
      }
  }
}

function rotate(dx, dy, dr) {
  switch (dr) {
    case 0:
      return [dx, dy];
    case 90:
      return [dy, 0 - dx];
    case 180:
      return [0 - dx, 0 - dy];
    case 270:
      return [0 - dy, dx];
    default:
      throw new Error('Non-cardinal direction: ' + dir);
  }
}

function stepStar2(x, y, dx, dy, inst, arg) {
  switch (inst) {
    case 'N':
      return [x, y, dx, dy - arg];
    case 'S':
      return [x, y, dx, dy + arg];
    case 'E':
      return [x, y, dx + arg, dy];
    case 'W':
      return [x, y, dx - arg, dy];
    case 'L':
      return [x, y, ...rotate(dx, dy, arg)];
    case 'R':
      return [x, y, ...rotate(dx, dy, 360 - arg)];
    case 'F':
      for (let f = 0; f < arg; f++) {
        x += dx;
        y += dy;
      }
      return [x, y, dx, dy];
  }
}

const day12star1 = (() =>  {
  let [x, y, dir] = [0, 0, 0];
  for (const [inst, arg] of which) {
    // console.log(`stepStar1 from ${x}, ${y}, ${dir} by doing ${inst}${arg}`);
    [x, y, dir] = stepStar1(x, y, dir, inst, arg);
  }
  return Math.abs(x) + Math.abs(y);
})();

const day12star2 = (() => {
  let [dx, dy] = [10, -1];
  let [x, y] = [0, 0];
  for (const [inst, arg] of which) {
    console.log(`stepStar2 from ${x}, ${y}, ${dx}, ${dy} by doing ${inst}${arg}`);
    [x, y, dx, dy] = stepStar2(x, y, dx, dy, inst, arg);
  }
  return Math.abs(x) + Math.abs(y);
})();

console.log('Star 1: ', day12star1);
console.log('Star 2: ', day12star2)