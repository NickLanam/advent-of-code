const p2s = ([x, y]) => `${x},${y}`;
const s2p = (s) => s.split(',').map(n => +n);

function parse(data) {
  const start = [1, 0];
  const goal = [data[0].length - 2, data.length - 1];
  const w = data[0].length - 2;
  const h = data.length - 2;
  let blizzards = new Map();
  for (let y = 1; y <= h; y++) {
    for (let x = 1; x <= w; x++) {
      const c = data[y].substring(x, x + 1);
      if (c !== '.') blizzards.set(p2s([x, y]), [c]);
    }
  }
  return { start, goal, blizzards, minX: 1, minY: 1, maxX: w, maxY: h };
}

function stepState({ blizzards, minX, minY, maxX, maxY }) {
  const next = new Map();
  for (const [k, b] of blizzards) {
    for (const v of b) {
      let [x, y] = s2p(k);
      if (v === '<') x--;
      if (v === '>') x++;
      if (v === '^') y--;
      if (v === 'v') y++;
      if (x < minX) x = maxX;
      if (x > maxX) x = minX;
      if (y < minY) y = maxY;
      if (y > maxY) y = minY;
      next.set(p2s([x, y]), [...(next.get(p2s([x, y])) ?? []), v]);
    }
  }
  return next;
}

function stateAtRound(round, cache, limits) {
  if (round < 0) throw new Error('Tried to fetch stateAtRound less than 0, did you forget to seed something?');
  if (!cache.has(round)) {
    cache.set(round, stepState({ blizzards: stateAtRound(round - 1, cache, limits), ...limits }));
  }
  return cache.get(round);
}

function findBestPathOut({ cache, start, goal, ...limits }, startRound = 0) {
  const manhattan = (x, y) => Math.abs(goal[0] - x) + Math.abs(goal[1] - y);
  const score = (round, [x, y]) => (manhattan(x, y) << 4) + round;

  let queue = [ { round: startRound, pos: start }, { round: startRound, pos: start }];
  let anythingChanged = true;
  for (let bfsStep = 1; bfsStep < 500 /* Safety limit */; bfsStep++) {
    // If everything in the queue is already in the goal state, we've finished the search.
    if (!anythingChanged) {
      break;
    }
    if (!queue.length) throw new Error('Nothing to loop through!');
    anythingChanged = false;
    const nextQueue = [];
    const nextSeen = new Set(); // For quickly preventing duplicates
    for (const { round: qRound, pos: [qx, qy] } of queue) {
      // Keep states that already found the goal, but only one copy of each.
      if (qx === goal[0] && qy === goal[1]) {
        if (!nextSeen.has(`${qRound};${qx},${qy}`)) {
          nextQueue.push({ round: qRound, pos: [qx, qy] });
          nextSeen.add(`${qRound};${qx},${qy}`);
        }
        continue;
      }
      anythingChanged = true;

      // Find spaces in this spot and neighboring spots that WON'T contain a
      // blizzard or wall in the next round. For each one, explore that move.
      const next = stateAtRound(qRound + 1, cache, limits);
      for (const [nx, ny] of [
        [qx - 1, qy], [qx, qy - 1], [qx + 1, qy], [qx, qy + 1], [qx, qy]
      ]) {
        const inRange = nx >= limits.minX
          && nx <= limits.maxX
          && ny >= limits.minY
          && ny <= limits.maxY;
        const inStartOrGoal = (nx === start[0] && ny === start[1]) || (nx === goal[0] && ny === goal[1]);
        const numBlizzards = (next.get(p2s([nx, ny])) ?? []).length;
        if ((inRange || inStartOrGoal) && numBlizzards < 1) {
          if (!nextSeen.has(`${qRound + 1};${nx},${ny}`)) {
            nextQueue.push({ round: qRound + 1, pos: [nx, ny] });
            nextSeen.add(`${qRound + 1};${nx},${ny}`);
          }
        }
      }
    }
    nextSeen.clear(); // Mark-sweep will have an easier time with this
    if (!nextQueue.length) {
      console.log(`After ${bfsStep - 1} loops, we had ${queue.length} options to check and ${finishes.length} of them were in the goal`);
      throw new Error('Found no legal positions?');
    }
    nextQueue.sort((b, a) => score(b.round, b.pos) - score(a.round, a.pos));
    queue = nextQueue.slice(0, 10_000);
  }
  return queue[0];
}

(await import('./aoc.mjs')).default(
  2022, 24,
  ({ start, goal, blizzards, ...limits }) => {
    const cache = new Map([[0, blizzards]]);
    return findBestPathOut({ cache, start, goal, ...limits }).round;
  }, 18,
  ({ start, goal, blizzards, ...limits }) => {
    const cache = new Map([[0, blizzards]]);
    const there = findBestPathOut({ cache, start, goal, ...limits }, 0).round;
    const back  = findBestPathOut({ cache, start: goal, goal: start, ...limits }, there).round;
    const again = findBestPathOut({ cache, start, goal, ...limits }, back).round;
    return again;
  }, 54,
  parse
);