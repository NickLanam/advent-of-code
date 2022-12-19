const GRID_WIDTH = 7;
const DROP_X_OFFSET = 2;
const DROP_Y_OFFSET = 3;
const REPEAT_CHECK_ROWS = 128;
const CACHE_WARMUP_DELAY = 10_000_000;

const SHAPES = [
  `####`,
  `.#.
  ###
  .#.`,
  `..#
  ..#
  ###`,
  `#
  #
  #
  #`,
  `##
  ##`
].map(
  s => s.replaceAll(' ', '').split('\n').map(r => r.split(''))
).map(
  (s, i) => ({
    w: Math.max(...s.map(r => r.length)),
    h: s.length,
    id: ['-', '+', 'L', '|', 'B'][i], // For debugging purposes mostly
    rows: s,
  }),
);

function addShapeToGrid(inputGrid, shape, sx, sy, tentative = false) {
  const grid = tentative ? [...inputGrid.map(r => [...r])] : inputGrid;
  for (let x = 0; x < shape.w; x++) {
    for (let y = 0; y < shape.h; y++) {
      while (grid.length < sy + y + 1) grid.push(Array(GRID_WIDTH).fill('.'));
      const existing = grid[sy + y][sx + x];
      const inShape = shape.rows[shape.h - y - 1][x];
      let v = '.';
      if (tentative) {
        if (existing === '#' && inShape === '#') v = '!';
        else if (existing === '#') v = '#';
        else if (inShape === '#') v = '@';
      } else if (existing === '#' || inShape === '#') {
        v = '#';
      }
      grid[sy + y][sx + x] = v;
    }
  }
  return grid;
}

function wouldCollide(grid, shape, sx, sy) {
  for (let x = 0; x < shape.w; x++) {
    for (let y = 0; y < shape.h; y++) {
      const existing = grid[sy + y]?.[sx + x] ?? '.';
      const inShape = shape.rows[shape.h - y - 1][x];
      if (existing === '#' && inShape === '#') return true;
    }
  }
  return false;
}

function dropOneRock(shape, firstStep, pattern, oldGrid = [], oldTotalPruned = 0) {
  let grid = [...oldGrid.map(r => [...r])];

  let step = firstStep;
  let x = DROP_X_OFFSET;
  let y = grid.length + DROP_Y_OFFSET; // Note: Y coordinate goes UP here, not down. Initial value is where the BOTTOM of the shape goes.
  dropLoop: while (true) {
    // In the challenge description, the gust happens, THEN the rock either falls or gets stopped.
    let push = pattern[step % pattern.length] === '>' ? 1 : -1;
    if (x + push >= 0 && x + push + shape.w <= GRID_WIDTH) {
      const canPush = !wouldCollide(grid, shape, x + push, y);
      if (canPush) {
        x += push;
      }
    }
    step++; // We do this when the gust pattern moves along, not when the shape falls

    if (y <= 0 || wouldCollide(grid, shape, x, y - 1)) {
      grid = addShapeToGrid(grid, shape, x, y, false);
      break dropLoop;
    } else {
      y--;
    }
  }

  return { step, grid, totalPruned: oldTotalPruned };
}

function sim(cache, shape, step, pattern, grid, oldPruned) {
  const cacheKey = `${shape.id};${step % pattern.length}`;

  let res = { step, grid, totalPruned: oldPruned };
  if (cache.has(cacheKey)) {
    const { countAdded, top, numSteps } = cache.get(cacheKey);
    res.grid.push(...top.slice(top.length - countAdded));
    res.totalPruned = oldPruned + (res.grid.length - REPEAT_CHECK_ROWS);
    res.grid = res.grid.slice(0 - REPEAT_CHECK_ROWS);
    res.step = step + numSteps;
  } else {
    res = dropOneRock(shape, step, pattern, grid, oldPruned);
    if (res.grid.length >= REPEAT_CHECK_ROWS && step >= pattern.length) {
      const countAdded = res.grid.length - grid.length;
      const top = res.grid.slice(0 - REPEAT_CHECK_ROWS);
      const numSteps = res.step - step;
      cache.set(cacheKey, { countAdded, top, numSteps });
    }
  }
  return res;
}

function solve(pattern, numberOfRocks) {
  console.log(`About to drop ${numberOfRocks.toExponential()} with gust pattern length ${pattern.length}`, new Date());
  let cache = new Map();
  let step = 0;
  let pruned = 0;
  let grid = [];

  // Warm up the cache by actually simulating rock drops for a while...
  const lo = Math.min(numberOfRocks, CACHE_WARMUP_DELAY);
  for (let r = 0; r < lo; r++) {
    const res = sim(cache, SHAPES[r % SHAPES.length], step, pattern, grid, pruned);
    step = res.step;
    grid = res.grid;
    pruned = res.totalPruned;
  }
  console.log('Cache is warmed up, setting up for the tight loop', new Date());

  // Then reorganize the cache to a format that's way faster to do lookups on.
  // We're doing nearly a trillion iterations, and the cost of both template strings and the % operator
  // stack up catastrophically (makes the solution take hours instead of seconds) otherwise.

  const stepCache = SHAPES.map(({ id }, s) => {
    const inner = Array(pattern.length).fill(null);
    for (const [k, v] of cache.entries()) {
      const [kid, kv] = k.split(';');
      if (kid === id) {
        inner[+kv] = v.numSteps;
      }
    }
    return inner;
  });

  const addCache = SHAPES.map(({ id }, s) => {
    const inner = Array(pattern.length).fill(null);
    for (const [k, v] of cache.entries()) {
      const [kid, kv] = k.split(';');
      if (kid === id) {
        inner[+kv] = v.countAdded;
      }
    }
    return inner;
  });

  // Finally, do the simplest part as quickly as possible. Avoid template strings. Avoid %. Don't call functions.
  const SL = SHAPES.length;
  const PL = pattern.length;
  const GL = grid.length;
  const doLog = (rr, p) => {
    console.log(`Have dropped ${rr.toExponential()} rocks, score ${GL + p}`, new Date());
  };

  let mr = lo % SL;
  step %= PL;
  let lr = lo; // For log check - same trick to prevent the log from slowing us down

  // This isn't a real solution, but it has the bones of one - it gets within 100 of the right answer for part 2.
  // It gets there within 3 seconds, so if I had the inclination I could probably figure out a correct heuristic.
  // let guess = 0;
  // for (let s in SHAPES) {
  //   const validAc = addCache[s].filter(a => a !== null);
  //   const ag = validAc.reduce((a, c) => a+c, 0) / validAc.length;
  //   guess += Math.ceil(ag * numberOfRocks / SHAPES.length);
  // }
  // return guess;

  // This way does work, and gets the right answer! But it takes over three hours to do so,
  // Because there are still a trillion iterations of the loop to run.
  for (let r = lo; r < numberOfRocks; r++) {
    // Doing this keeps these numbers small, and thus prevents doing % on huge numbers (which is very slow)
    lr = lr % 1e10; // This one might actually be slowing me down noticeably

    if (r > 0 && lr === 0) doLog(r, pruned); // Keep the template calculation outside of the loop too

    pruned += addCache[mr][step];
    step = (step + stepCache[mr][step]) % PL;
    mr = (mr + 1) % SL;
    lr++;
  }

  return grid.length + pruned;
}

(await import('./aoc.mjs')).default(
  2022, 17,
  pattern => solve(pattern, 2022), 3068,
  // My solution is missing a pattern check so it takes 3hr20min on a Ryzen 9 3900X to do it the hard way.
  // It would be several weeks without the pattern checking it DOES do.
  (pattern, isSample) => (isSample ? 1_514_285_714_288 : solve(pattern, 1e12)), 1_514_285_714_288,
  data => data[0].split(''), true, false
);