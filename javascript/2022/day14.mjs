const p2s = point => `${point[0]},${point[1]}`;
const s2p = str => JSON.parse(`[${str}]`);

function buildState(inputLines) {
  const sequences = inputLines.map(line => line.split(' -> ').map(c => c.split(',').map(n => +n)));

  // Mercifully, no pair ever goes diagonally - points are always aligned on the cardinals relative to each other.
  // Also, no sequence has only one point, so we can skip that edge case too.
  let maxY = 0;
  const state = new Map(); // Map<stringified-point, thing-at-that-point>
  for (const seq of sequences) {
    let start = seq.shift();
    let end = seq.shift();
    while (start && end) {
      maxY = Math.max(maxY, start[1]);
      state.set(p2s(start), '#');
      for (let x = start[0]; x !== end[0] + Math.sign(end[0] - start[0]); x += Math.sign(end[0] - start[0])) {
        state.set(p2s([x, start[1]]), '#');
      }
      for (let y = start[1]; y !== end[1] + Math.sign(end[1] - start[1]); y += Math.sign(end[1] - start[1])) {
        state.set(p2s([start[0], y]), '#');
        maxY = Math.max(maxY, y);
      }
      start = end;
      end = seq.shift();
    }
  }
  return { state, maxY };
}

function step(state, maxY, stepNumber, stopMode) {
  if (stepNumber > 100_000) {
    // console.log('Bailed with this state:');
    // printState(state, stopMode);
    throw new Error('Too many steps');
  }

  let placedSand = false;
  let x = 500;
  let y = 0;
  while (stopMode === 'void' ? (y < maxY) : (y < maxY + 2)) {
    let obstacles = [state.get(p2s([x - 1, y + 1])), state.get(p2s([x, y + 1])), state.get(p2s([x + 1, y + 1]))];
    if (y === maxY + 1) obstacles = ['=', '=', '='];
    if (obstacles[1]) {
      if (obstacles[0]) {
        if (obstacles[2]) {
          placedSand = true;
          break;
        } else {
          x = x + 1;
        }
      } else {
        x = x - 1;
      }
    }
    y++;
  }

  if (x === 500 && y === 0) {
    // console.log(`Tried to step past 500,0 and did not, so that must mean we hit the top`);
    return false;
  }

  if (placedSand) {
    // console.log(`Placed a sand at ${x}, ${y}`);
    state.set(p2s([x, y]), 'o');
  } else {
    // console.log(`Failed to place sand, final place I tried was ${x}, ${y}`);
  }
  // printState(state);
  return placedSand;
}

function printState(realState, stopMode) {
  const state = new Map(realState);
  let minX = Infinity; let minY = 0; let maxX = -Infinity; let maxY = -Infinity;
  for (const k of state.keys()) {
    const [x, y] = s2p(k);
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
  }

  if (stopMode === 'floor') {
    const y = maxY + 2;
    for (let x = minX - 2; x <= maxX + 2; x++) {
      state.set(p2s([x, y]), '=');
    }
    maxY += 2;
    minX -= 2;
    maxX += 2;
  }

  for (let y = minY; y <= maxY; y++) {
    let line = '';
    for (let x = minX; x <= maxX; x++) {
      line += state.get(p2s([x, y])) ?? '.';
    }
    console.log(line);
  }
}

(await import('./aoc.mjs')).default(
  2022, 14,
  ({ state, maxY }) => {
    // console.log('Initial state...');
    // printState(state, 'void');
    let proceed = true;
    let sandBuilt = 0;
    while (proceed) {
      proceed = step(state, maxY, sandBuilt, 'void');
      if (proceed) sandBuilt++;
    }
    // console.log('Final state...');
    // printState(state, 'void');
    return sandBuilt;
  }, 24,
  ({ state, maxY }) => {
    // console.log('Initial state...');
    // printState(state, 'floor');
    let proceed = true;
    let sandBuilt = 0;
    while (proceed) {
      proceed = step(state, maxY, sandBuilt, 'floor');
      if (proceed) sandBuilt++;
    }
    // console.log('Final state...');
    // printState(state, 'floor');
    return sandBuilt + 1;
  }, 93,
  buildState, true, false 
);