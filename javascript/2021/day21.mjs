import { range } from './utils/array.mjs';

function toKey({ p1pos, p1score, p2pos, p2score, next }) {
  return [p1pos, p1score, p2pos, p2score, next].join(';');
}

function nextTurn({ p1pos, p1score, p2pos, p2score, next }, roll) {
  if (next === 0) {
    const pos = ((p1pos - 1 + roll) % 10) + 1
    return {
      p1pos: pos,
      p1score: p1score + pos,
      p2pos,
      p2score,
      next: 1,
    };
  } else {
    const pos = ((p2pos - 1 + roll) % 10) + 1;
    return {
      p1pos,
      p1score,
      p2pos: pos,
      p2score: p2score + pos,
      next: 0
    };
  }
}

(await import('./aoc.mjs')).default(
  2021, 21,
  ([p1start, p2start]) => {
    let state = {
      p1pos: p1start,
      p1score: 0,
      p2pos: p2start,
      p2score: 0,
      next: 0,
    };
    for (let turn = 0; turn < 2002; turn++) {
      // Roll 0: 1+2+3 = 6; Roll 1: 4+5+6 = 15; (3n+1) + (3n+2) + (3n+3) = 9n + 6
      state = nextTurn(state, (9 * turn) + 6);
      if (state.p1score >= 1000) return (turn + 1) * 3 * state.p2score;
      if (state.p2score >= 1000) return (turn + 1) * 3 * state.p1score;
    }
    throw new Error('Game cannot take more than 1,000 turns before someone has 1,000 points.');
  }, 739785,
  ([p1start, p2start]) => {
    // My initial solutions each failed to account for probabilities or did incorrect graph traversal.
    // The entire time, I thought there had to be a shortcut that avoided the grid entirely.
    // I failed to find one, and I ended up adapting this solution:
    //   https://github.com/timvisee/advent-of-code-2021/blob/master/day21b/src/main.rs
    // As penance, I've figured out why it works and commented where the original author did not.

    // 3d3 can roll 3 (one way), 4 (three ways), 5 (six ways), 6 (seven ways), 7 (six ways), 8 (three ways), or 9 (one way).
    const probabilities = [1, 3, 6, 7, 6, 3, 1];

    function scores(pos) {
      // For one player with a given starting position, sums the score in each position at each turn for all possible rolls that reach that position.
      const turns = Array(11).fill(0).map(() => Array(11).fill(0).map(() => Array(22).fill(0)));
      turns[0][pos][0] = 1; // Mark the starting location
      for (const t of range(1, 11)) { // Outer: Turn counter
        for (const p of range(1, 11)) { // Middle: positions (1-10 inclusive) on the board.
          for (const s of range(0, 21)) { // Inner: Counts how many games got a given score in that position on that turn.
            for (const i of range(probabilities.length)) {
              // probabilities[0] represents a 3d3 roll of 3, which can only happen one way (w=1).
              // probabilities[3] represents a 3d3 roll of 6, which can happen 7 ways (w=7).
              const w = probabilities[i];
              // Rotate the player's position (1-10 inclusive) forward by the given roll. For p=1, that's a roll of 4, so advance four steps.
              const q = ((p + i + 2) % 10) + 1;
              // Add the player's new position to the score, but cap it at 21 (the win condition).
              const v = Math.min(q + s, 21);
              // Add (weight * score at previous position in previous turn) to this turn's score at this position.
              // That is, the innermost cell isn't just the score at that position on that turn, it's the sum of such scores for all possible paths to reach it.
              turns[t][q][v] += w * turns[t - 1][p][s];
            }
          }
        }
      }

      // [#ways to get to 21 points from the starting position at each turn, sum of possible scores at each turn]
      const out = [Array(11).fill(0), Array(11).fill(0)];
      for (const t of range(turns.length)) {
        const positions = turns[t];
        for (const scores of positions.slice(1)) {
          for (const score of scores.slice(0, 21)) {
            out[1][t] += score;
          }
          out[0][t] += scores[21];
        }
      }
      return out;
    }

    // Actual puzzle answer: for the player that won the most simulated games, how many did they win?
    const p = [scores(p1start), scores(p2start)];
    return Math.max(
      // How many games player 1 wins: number of realities where they got a winning score, times the weighted scores of the other player by then.
      [...range(1, 11)].map(t => p[0][0][t] * p[1][1][t - 1]).reduce((a, c) => a + c, 0),
      // How many games player 2 wins: same logic, but player 2 goes second (and the game ends when either player reaches 21).
      [...range(1, 11)].map(t => p[1][0][t - 1] * p[0][1][t - 1]).reduce((a, c) => a + c, 0)
    );
  }, 444356092776315,
  lines => lines.map(l => +l.slice(l.lastIndexOf(' ') + 1))
);
