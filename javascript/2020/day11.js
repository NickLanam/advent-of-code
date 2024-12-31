const { getInput, fromRaw } = require('./utils');
const input = getInput(11).map(l => l.split(''));
/*
Sample stabilizes with 37 occupied states and looks like this for part 1:
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
*/
const sample = `L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL`.split('\n').map(l => l.split(''));

const which = input;
const width = which[0].length;
const height = which.length;

function getSafe(r, c, seats) {
  return seats[r] && seats[r][c] || null;
}
function isOccupied(r, c, seats) {
  const seat = getSafe(r, c, seats);
  return seat === '#' ? 1 : 0;
}

function isOccupiedDirection(r, c, dr, dc, seats) {
  if (dr === 0 && dc === 0) throw new Error('direction is zero');
  let nr = r + dr;
  let nc = c + dc;
  while (nr >= 0 && nr < height && nc >= 0 && nc < width) {
    const s = getSafe(nr, nc, seats);
    if (s === '#') return 1;
    if (s === 'L') return 0;
    nr += dr;
    nc += dc;
  }
  return 0;
}

function iterateStar1(seats) {
  const out = seats.map(l => [...l]);
  for (let c = 0; c < width; c++) {
    for (let r = 0; r < height; r++) {
      const seat = seats[r][c];
      if (seat === '.') continue;
      const occupiedNeighbors = [
        [r - 1, c - 1],
        [r - 1, c + 0],
        [r - 1, c + 1],
        [r + 0, c - 1],
        [r + 0, c + 1],
        [r + 1, c - 1],
        [r + 1, c + 0],
        [r + 1, c + 1],
      ].map(([r2, c2]) => isOccupied(r2, c2, seats)).reduce((a, v) => a + v, 0);
      if (seat === 'L' && occupiedNeighbors === 0) out[r][c] = '#';
      if (seat === '#' && occupiedNeighbors >= 4) out[r][c] = 'L';
    }
  }
  return out;
}

function iterateStar2(seats) {
  const out = seats.map(l => [...l]);
  for (let c = 0; c < width; c++) {
    for (let r = 0; r < height; r++) {
      const seat = seats[r][c];
      if (seat === '.') continue;
      const occupiedNeighbors = [
        [-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1],
      ].map(([dr, dc]) => isOccupiedDirection(r, c, dr, dc, seats)).reduce((a, c) => a + c, 0);
      if (seat === 'L' && occupiedNeighbors === 0) out[r][c] = '#';
      if (seat === '#' && occupiedNeighbors >= 5) out[r][c] = 'L';
    }
  }
  return out;
}

function flat(seats) {
  return [].concat(...seats).join('');
}

const day11star1 = (() =>  {
  const seenStates = new Set();
  let state = which.map(l => [...l]);
  let limit = 1000;
  while (!seenStates.has(flat(state)) && limit-- > 0) {
    seenStates.add(flat(state));
    state = iterateStar1(state);
  }
  return flat(state).replace(/[L\.]/g, '').length;
})();

const day11star2 = (() => {
  const seenStates = new Set();
  let state = which.map(l => [...l]);
  let limit = 1000;
  while (!seenStates.has(flat(state)) && limit-- > 0) {
    seenStates.add(flat(state));
    state = iterateStar2(state);
  }
  return flat(state).replace(/[L\.]/g, '').length;
})();

console.log('Star 1: ', day11star1);
console.log('Star 2: ', day11star2)