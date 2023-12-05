import aoc from './aoc.mjs';

const part1expected = 35;
const part2expected = 46;

const parse = (data) => ({
  seeds: data[0].split(': ')[1].split(' ').map(id => +id),
  rangeBlocks: data.slice(2).join('\n').split('\n\n').map(block => block.split('\n').slice(1).map(l => l.split(' ').map(c => +c))),
});

const solve = ({ seeds, rangeBlocks }, isSample, isPart1) => {
  const seedBlocks = isPart1
    ? seeds.reduce((a, c, i) => [...a, [c, 1]], [])
    : seeds.reduce((a, c, i) => (i % 2 === 0 ? [...a, [c, seeds[i + 1]]] : a), []);

  for (const block of rangeBlocks) {
    const toAdd = [];
    for (const [dest, source, blockLen] of block) {
      let i = 0;
      while (i < seedBlocks.length) {
        const [scanStart, scanLen] = seedBlocks[i];
        const scanEnd = scanStart + scanLen;
        const sourceEnd = source + blockLen;
        

        if (source <= scanStart && scanStart < sourceEnd && sourceEnd <= scanEnd) {
          toAdd.push([scanStart - source + dest, sourceEnd - scanStart]);
          seedBlocks[i] = [sourceEnd, scanEnd - source - blockLen];
        } else if (scanStart <= source && source < scanEnd && scanEnd <= sourceEnd) {
          toAdd.push([dest, scanEnd - source]);
          seedBlocks[i] = [scanStart, source - scanStart];
        } else if (scanStart <= source && source < sourceEnd && sourceEnd <= scanEnd) {
          toAdd.push([dest, blockLen]);
          seedBlocks[i] = [scanStart, source - scanStart];
          seedBlocks.push([sourceEnd, scanEnd - source - blockLen]);
        }

        if (source <= scanStart && scanStart < scanEnd && scanEnd <= sourceEnd) {
          toAdd.push([scanStart - source + dest, scanLen]);
          seedBlocks[i] = seedBlocks[seedBlocks.length - 1];
          seedBlocks.splice(i, 1);
        } else {
          i += 1;
        }
      }
    }
    seedBlocks.push(...toAdd);
  }
  return Math.min(...seedBlocks.map(([vbeg]) => vbeg));
};

const part1 = (parsed, isSample) => solve(parsed, isSample, true);
const part2 = (parsed, isSample) => solve(parsed, isSample, false);

aoc(2023, 5, part1, part1expected, part2, part2expected, parse);
