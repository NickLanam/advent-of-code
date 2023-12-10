import aoc from './aoc.mjs';

const part1expected = 22; // 8 for the part 1 sample, varies for the part 2 samples
const part2expected = 4;

const parse = (data) => {
  const h = data.length;
  const w = data[0].length;

  // First pass: turn the grid into a data structure we can use,
  // and find the S while we're at it.
  let start = [0, 0];
  const map = new Map(); // `${x},${y}` -> { x, y, links: [[x, y]] }
  for (let y = 0; y < h; y++) {
    let line = data[y].split('');
    for (let x = 0; x < w; x++) {
      let c = line[x];
      let links;
      switch (c) {
        case '.': links = []; break;
        case '-': links = [[x - 1, y], [x + 1, y]]; break;
        case '|': links = [[x, y - 1], [x, y + 1]]; break;
        case 'L': links = [[x, y - 1], [x + 1, y]]; break;
        case 'J': links = [[x - 1, y], [x, y - 1]]; break;
        case '7': links = [[x - 1, y], [x, y + 1]]; break;
        case 'F': links = [[x + 1, y], [x, y + 1]]; break;
        case 'S': {
          start = [x, y];
          links = [];
          break;
        }
      }
      links = links.filter(([x, y]) => (
        x >= 0 && x < w && y >= 0 && y < h
      ));
      map.set(`${x},${y}`, {
        x, y, links, c,
        isStart: c === 'S',
      });
    }
  }

  // Second pass: figure out what was under the S
  const [sx, sy] = start;
  let linksFromStart = [
    [sx, sy - 1],
    [sx - 1, sy],
    [sx, sy + 1],
    [sx + 1, sy],
  ].filter(([tx, ty]) => (
    map.get(`${tx},${ty}`)?.links?.some(([lx, ly ]) => lx === sx && ly === sy)
  ));

  map.set(`${sx},${sy}`, {
    x: sx,
    y: sy,
    links: linksFromStart,
    isStart: true,
    c: 'S',
  });

  // TODO: Third pass if needed: remove links that aren't bidirectional? Depends on what part 2 is.

  return { start: map.get(`${sx},${sy}`), map, w, h };
};

const part1 = ({ start, map }) => {
  // Since we're trying to figure out what the destination IS, we have to do a full search.
  // No Dijkstra's here, just pure BFS and tracking the worst offender.
  const explored = new Map();
  const unexplored = [[start.x, start.y]];
  let farthest = -Infinity;
  while (unexplored.length) {
    const [ux, uy] = unexplored.shift();
    // Since we're only looking to see which node is FARTHEST away,
    // we know that once a node has been seen once in BFS, its score won't get lower.
    if (explored.has(`${ux},${uy}`)) continue;
    const { links, isStart } = map.get(`${ux},${uy}`);
    const nearScores = links.map(([lx, ly]) => explored.get(`${lx},${ly}`) ?? -Infinity).filter(ns => ns >= 0);
    if (!nearScores.length) {
      // Disconnected nodes don't get a score, but do need to be marked to not touch again.
      explored.set(`${ux},${uy}`, isStart ? 0 : -Infinity);
    } else {
      explored.set(`${ux},${uy}`, Math.min(...nearScores) + 1);
      farthest = Math.max(farthest, Math.min(...nearScores) + 1);
    }
    unexplored.push(...links);
  }
  return farthest;
};

const part2 = ({ start, map, w, h }) => {
  // TL;DR: Flood fill, squeezing between gaps; answer is nodes that can't be reached.
  // - Pick the ring of dots outside of the original grid.
  // - Start a flood fill that includes all of those, and works its way in.
  // - It will not fill a node that's part of the main loop.
  // - It WILL squeeze between pipes.
  // - Answer is the number of dots that were NOT reachable this way!

  // First step: remove any links that aren't connected to the loop containing S.
  // We do this because we want to "touch" every dot that isn't enclosed by that loop.
  const { x: sx, y: sy } = start;
  const mainLoopExplored = new Set();
  const mainLoopUnexplored = [[sx, sy]];
  while (mainLoopUnexplored.length) {
    const [ux, uy] = mainLoopUnexplored.shift();
    const uk = `${ux},${uy}`;
    if (mainLoopExplored.has(uk)) continue;
    const { links } = map.get(uk);
    mainLoopExplored.add(uk);
    const next = links.filter(([lx, ly]) => {
      const lk = `${lx},${ly}`;
      const isReturned = map.has(lk) && map.get(lk).links.some(([tx, ty]) => tx === ux && ty === uy);
      return isReturned && !mainLoopExplored.has(lk);
    });
    mainLoopUnexplored.push(...next);
  }

  // TODO: THERE'S ONE MISSING REQUIREMENT TO HANDLE
  // THE FLOOD FILL IS SUPOSSED TO BE ABLE TO SQUEEZE BETWEEN PIPES
  // TO DO THAT, MY IDEA IS THUS:
  // - Make a new map with extra points at all of the x.5, y.5 positions
  // - Each of them is only a dot if none of their x.0, y.0 neighbors cross over it
  // - Use this special map to do the flood filling
  // - Final answer still only looks for dots in the original map though
  // - Have to do this in two passes: stretch along x, then along y
  // - Only the x.5 lines and y.5 lines, NOT the intersections between them? How to traverse?

  // Flood fill to find all dots that aren't enclosed by the main loop.
  // Yet another BFS, but totally different innards.
  const explored = new Set();
  const unexplored = [[-1, -1]];
  while (unexplored.length) {
    const [ux, uy] = unexplored.shift();
    const uk = `${ux},${uy}`;
    if (explored.has(uk)) continue;
    explored.add(uk);
    for (const [nx, ny] of [
      [ux - 1, uy],
      [ux + 1, uy],
      [ux, uy - 1],
      [ux, uy + 1],
    ]) {
      // Don't look past the outer ring
      const nk = `${nx},${ny}`;
      const outOfBounds = (nx < -1 || nx > w || ny < -1 || ny > h);
      const seen = (explored.has(nk) || mainLoopExplored.has(nk));
      if (!outOfBounds && !seen) {
        unexplored.push([nx, ny]);
      }
    }
  }

  console.log("Should be able to produce an answer now", {onMainLoop: mainLoopExplored.size, total: [...map.keys()].length, reachableExcludingRing: explored.size - w - w - h - h - 4 });

  // Finally: count how many dots (in the unmodified map) were not found by the flood fill
  // Currently gets 711, too high
  return [...map.keys()].filter(k => !explored.has(k) && !mainLoopExplored.has(k)).length;
};

aoc(2023, 10, part1, part1expected, part2, part2expected, parse);
