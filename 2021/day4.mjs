function hasBingo(seen, board) {
  for (const row of board) {
    if (row.every(c => seen.includes(c))) return true;
  }
  for (let col = 0; col < board[0].length; col++) {
    if (board.every(row => seen.includes(row[col]))) return true;
  }
  return false;
}

function getScore(seen, board) {
  if (!hasBingo(seen, board)) return 0;
  return seen[seen.length - 1] * board.flat()
    .reduce((a, c) => (a + (seen.includes(c) ? 0 : c)), 0);
}

function play({ seq, state }, stopOnFirstWinner = false) {
  const winners = [];
  for (let i = 0; i < seq.length; i++) {
    state.seen.push(seq[i]);
    for (const b in state.boards) {
      state.scores[b] = getScore(state.seen, state.boards[b]);
      if (state.scores[b] > 0 && !winners.includes(b)) winners.push(b);
    }
    if (winners.length && stopOnFirstWinner) break; // Stop when ANYONE wins
    if (state.scores.every(s => s > 0)) break; // Stop when EVERYONE has won
  }
  return { scores: state.scores, winners };
}

(await import('./aoc.mjs')).default(
  2021, 4,
  (data) => {
    const { scores, winners } = play(data, true);
    return scores[winners[0]];
  }, 4512,
  (data) => {
    const { scores, winners } = play(data, false);
    return scores[winners[winners.length - 1]];
  }, 1924,
  data => {
    const lines = [...data];
    const seq = lines.shift().split(',').map(n => +n);
    const boards = [];
    while (lines.length) {
      lines.shift(); // Empty
      boards.push(
        lines.splice(0, 5).map(
          line => line.split(' ').filter(n => n.trim().length).map(n => +n.trim())));
    }
    return {
      seq,
      state: {
        seen: [],
        boards,
        scores: Array(boards.length).fill(0)
      }
    };
  }
);