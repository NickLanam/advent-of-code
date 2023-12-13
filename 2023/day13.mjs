import aoc from './aoc.mjs';

const part1expected = 405;
const part2expected = 400;

const parse = (data, part) => {
  return data
    .join('\n')
    .split('\n\n')
    .map(group => group.split('\n').map(line => line.split('')))
    .map(byRows => ({
      byRows,
      byCols: (
        Array(byRows[0].length)
          .fill(0)
          .map((_, i) => byRows.map((r) => r[i]))
      )
    }));
};

const getMirrorLeads = (lines) => {
  // Strategy: we're looking for a line that's duplicated.
  // We're then looking out in both directions until an edge is reached.
  // If we don't reach an edge, keep looking (there are trick reflections).
  const rejoined = lines.map(l => l.join(''));
  mirrorCheck: for (let i = 0; i < rejoined.length - 1; i++) {
    if (rejoined[i] === rejoined[i + 1]) {
      let j = 1;
      for (; (i + 1 + j) < rejoined.length && i - j >= 0; j++) {
        if (rejoined[i - j] !== rejoined[i + 1 + j]) {
          continue mirrorCheck;
        }
      }
      return i + 1;
    }
  }
  return 0;
};

const part1 = (groups) => {
  let lefts = 0;
  let aboves = 0;

  for (const { byRows, byCols } of groups) {
    lefts += getMirrorLeads(byCols);
    aboves += getMirrorLeads(byRows);
  }

  return lefts + (100 * aboves);
};

const part2 = (data) => {
  return 'NYI';
};

aoc(2023, 13, part1, part1expected, part2, part2expected, parse);
