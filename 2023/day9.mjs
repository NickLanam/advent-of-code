import aoc from './aoc.mjs';
import { sum } from './utils/array.mjs';

const part1expected = 114;
const part2expected = 2;

const parse = (data) => data.map(l => l.split(' ').map(n => +n));

const part1 = (histories) => {
  const makeNextLine = (history) => history.slice(1).map((v, i) => v - history[i]);
  const nextVals = histories.map((history) => {
    const lines = [history];
    while (lines[lines.length - 1].some(v => v !== 0)) {
      lines.push(makeNextLine(lines[lines.length - 1]));
    }
    let innerNexts = new Array(lines.length).fill(0);
    for (let l = lines.length - 2; l >= 0; l--) {
      innerNexts[l] = lines[l][lines[l].length - 1] + innerNexts[l + 1];
    }
    return innerNexts[0];
  });
  return sum(nextVals);
};

const part2 = (histories) => {
  const makeNextLine = (history) => history.slice(1).map((v, i) => v - history[i]);
  const prevVals = histories.map((history) => {
    const lines = [history];
    while (lines[lines.length - 1].some(v => v !== 0)) {
      lines.push(makeNextLine(lines[lines.length - 1]));
    }
    let innerPrevs = new Array(lines.length).fill(0);
    for (let l = lines.length - 2; l >= 0; l--) {
      innerPrevs[l] = lines[l][0] - innerPrevs[l + 1];
    }
    return innerPrevs[0];
  });
  return sum(prevVals);
};

aoc(2023, 9, part1, part1expected, part2, part2expected, parse);
