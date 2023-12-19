import aoc from './aoc.mjs';

const part1expected = 288;
const part2expected = 71503;

const parse = ([timeLine, distanceLine]) => ({
  times: timeLine.split(':')[1].trim().split(/\s+/g).map(c => +c),
  distances: distanceLine.split(':')[1].trim().split(/\s+/g).map(c => +c),
});

const part1 = ({ times, distances }) => {
  const allWaysToWin = [];
  for (let race = 0; race < times.length; race++) {
    const len = times[race];
    const toBeat = distances[race];
    let waysToWin = 0;
    for (let t = 1; t < len; t++) {
      if (t * (len - t) > toBeat) waysToWin++;
    }
    allWaysToWin.push(waysToWin);
  }
  return allWaysToWin.reduce((a, c) => a * Math.max(c, 1), 1);
};

const part2 = (original) => {
  const len = +original.times.reduce((a, c) => `${a}${c}`, '');
  const toBeat = +original.distances.reduce((a, c) => `${a}${c}`, '');
  let waysToWin = 0;
  for (let t = 1; t < len; t++) {
    if (t * (len - t) > toBeat) waysToWin++;
  }
  return waysToWin;
};

aoc(2023, 6, part1, part1expected, part2, part2expected, parse);
