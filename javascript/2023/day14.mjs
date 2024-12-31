import aoc from './aoc.mjs';

const part1expected = 136;
const part2expected = 64;

const parse = (data) => {
  const rows = data.map(l => l.split(''));
  const columns = Array(rows[0].length).fill(0).map((_, c) => rows.map(r => r[c]));
  return columns;
};

const part1 = (columns) => {
  const rolled = columns.map((col) => {
    const newCol = [...col];
    rollNorth: while (true) {
      let didRoll = false;
      for (let c = 0; c < col.length - 1; c++) {
        if (newCol[c] === '.' && newCol[c + 1] === 'O') {
          didRoll = true;
          newCol[c] = 'O';
          newCol[c + 1] = '.';
        }
      }
      if (!didRoll) break rollNorth;
    }
    return newCol
      .map((v, r) => v === 'O' ? col.length - r : 0)
      .reduce((a, c) => a + c, 0);
  }).reduce((a, c) => a + c, 0);
  return rolled;
};

const part2 = (columns) => {
  const spinCycle = () => {
    rollNorth: while (true) {
      let didRoll = false;
      for (let c = 0; c < columns[0].length; c++) {
        for (const col of columns) {
          if (col[c] === '.' && col[c + 1] === 'O') {
            didRoll = true;
            col[c] = 'O';
            col[c + 1] = '.';
          }
        }
      }
      if (!didRoll) break rollNorth;
    }
    rollWest: while (true) {
      let didRoll = false;
      for (let r = 0; r < columns[0].length; r++) {
        for (let c = 0; c < columns.length - 1; c++) {
          if (columns[c][r] === '.' && columns[c + 1][r] === 'O') {
            didRoll = true;
            columns[c][r] = 'O';
            columns[c + 1][r] = '.';
          }
        }
      }
      if (!didRoll) break rollWest;
    }
    rollSouth: while (true) {
      let didRoll = false;
      for (let c = columns[0].length - 1; c > 0; c--) {
        for (const col of columns) {
          if (col[c] === '.' && col[c - 1] === 'O') {
            didRoll = true;
            col[c] = 'O';
            col[c - 1] = '.';
          }
        }
      }
      if (!didRoll) break rollSouth;
    }
    rollEast: while (true) {
      let didRoll = false;
      for (let r = 0; r < columns[0].length; r++) {
        for (let c = columns.length - 1; c > 0; c--) {
          if (columns[c][r] === '.' && columns[c - 1][r] === 'O') {
            didRoll = true;
            columns[c][r] = 'O';
            columns[c - 1][r] = '.';
          }
        }
      }
      if (!didRoll) break rollEast;
    }
  };

  const score = (columns) => columns
    .map((col) => (
      col
        .map((v, r) => (
          v === 'O' ? col.length - r : 0))
        .reduce((a, c) => a + c, 0)))
    .reduce((a, c) => a + c, 0);

  // After each spin cycle, remember what the board looked like and what cycle it happened at.
  // When we reach the same state a second time, figure out how many cycles the loop took.
  // We can then skip forward all the way to the last time we would see that state,
  // then fetch the final (already-seen-by-then) state and score that.
  const seenStates = new Map();
  seenStates.set(columns.map(c => c.join('')).join(';'), 0);
  const cycleToState = new Map([[...seenStates.entries()][0].reverse()]);

  for (let cycle = 1; cycle <= 1_000_000_000; cycle++) {
    spinCycle();

    const state = columns.map(c => c.join('')).join(';');
    if (seenStates.has(state)) {
      const jumpBy = cycle - seenStates.get(state);
      cycle += Math.floor((1_000_000_000 - cycle) / jumpBy) * jumpBy;
      const finalState = cycleToState.get(seenStates.get(state) + (1_000_000_000 - cycle));
      const finalCols = finalState.split(';').map(col => col.split(''));
      return score(finalCols);
    } else {
      seenStates.set(state, cycle);
      cycleToState.set(cycle, state);
    }
  }
  throw new Error('Failed to find a loop!');
  // return score(columns); // Would do this, but the whole gimmick is that there IS a loop.
};

aoc(2023, 14, part1, part1expected, part2, part2expected, parse);
