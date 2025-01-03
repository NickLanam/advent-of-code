import aoc from './aoc.mjs';

const part1expected = 405;
const part2expected = 400;

const parse = (data) => (
  data
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
    }))
);

// Strategy: we're looking for a line that's duplicated.
// We're then looking out in both directions until an edge is reached.
// If we don't reach an edge, keep looking (there are trick reflections).
const getAllMirrorLeads = (lines) => {
  const found = [];
  const rejoined = lines.map(l => l.join(''));
  mirrorCheck: for (let i = 0; i < rejoined.length - 1; i++) {
    if (rejoined[i] === rejoined[i + 1]) {
      let j = 1;
      for (; (i + 1 + j) < rejoined.length && i - j >= 0; j++) {
        if (rejoined[i - j] !== rejoined[i + 1 + j]) {
          continue mirrorCheck;
        }
      }
      found.push(i + 1);
    }
  }
  return found;
};

const part1 = (groups) => {
  let lefts = 0;
  let aboves = 0;

  for (const { byRows, byCols } of groups) {
    lefts += getAllMirrorLeads(byCols)[0] ?? 0;
    aboves += getAllMirrorLeads(byRows)[0] ?? 0;
  }

  return lefts + (100 * aboves);
};

const part2 = (groups) => {
  let lefts = 0;
  let aboves = 0;

  for (const { byRows, byCols } of groups) {
    const originalLefts = getAllMirrorLeads(byCols);
    const originalAboves = getAllMirrorLeads(byRows);

    const flip = { '.': '#', '#': '.'};

    // Brute force: just try them all until we get a different valid answer.
    // Luckily, today wasn't another optimization problem.
    differenceSearch: for (let r = 0; r < byRows.length; r++) {
      for (let c = 0; c < byCols.length; c++) {
        const newByRows = byRows
          .map((row, r2) => row.map((cell, c2) => r === r2 && c === c2 ? flip[cell] : cell));
        const newByCols = byCols
          .map((col, c2) => col.map((cell, r2) => r === r2 && c === c2 ? flip[cell] : cell));

        const newLefts = getAllMirrorLeads(newByCols);
        const newAboves = getAllMirrorLeads(newByRows);

        const hasChangedLefts = newLefts.length > 0 &&
          newLefts.some(n => !originalLefts.includes(n));
        const hasChangedAboves = newAboves.length > 0 &&
          newAboves.some(n => !originalAboves.includes(n));

        if (hasChangedLefts || hasChangedAboves) {
          if (hasChangedLefts) {
            lefts += newLefts.find(n => !originalLefts.includes(n));
          } else if (hasChangedAboves) {
            aboves += newAboves.find(n => !originalAboves.includes(n));
          }
          break differenceSearch;
        }
      }
    }
  }

  return lefts + (100 * aboves);
};

aoc(2023, 13, part1, part1expected, part2, part2expected, parse);
