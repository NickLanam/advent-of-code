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

// The extra requirement for part 2 requires squeezing BETWEEN pipes.
// To handle this, we stretch the grid in both directions and insert
// the appropriate connections in between.
//
// To do this, stretch the map to twice its width and height,
// stretching links as well. This will widen gaps from 0 to 1,
// letting the flood fill later use them.
function expandForSqueezing({ map, w, h }) {
  const expanded = new Map();
  for (let y = 0; y < h; y += 0.5) {
    for (let x = 0; x < w; x += 0.5) {
      const k = `${x},${y}`;
      if (map.has(k)) {
        const oldLinks = map.get(k).links;
        expanded.set(k, {
          ...map.get(k),
          // Connect to the fake links to make pathing below easier to handle
          links: [
            ...oldLinks,
            x >= 0 && oldLinks.some(([tx, ty]) => tx === x - 1 && ty === y) && [x - 0.5, y],
            y >= 0 && oldLinks.some(([tx, ty]) => tx === x && ty === y - 1) && [x, y - 0.5],
            x < w - 1 && oldLinks.some(([tx, ty]) => tx === x + 1 && ty === y) && [x + 0.5, y],
            y < h - 1 && oldLinks.some(([tx, ty]) => tx === x && ty === y + 1) && [x, y + 0.5],
          ].filter(p => p)
        });
      } else if (Math.floor(x) !== x && Math.floor(y) !== y) {
        expanded.set(k, { x, y, links: [], c: ' ', isFake: true });
      } else if (Math.floor(x) !== x) {
        expanded.set(k, {
          x, y,
          links: [
            [x - 0.5, y],
            [x + 0.5, y],
          ].filter(([lx, ly]) => {
            const lk = `${lx},${ly}`;
            const node = map.get(lk);
            if (!node) return false; // Probably out of range, not likely an error
            return node.links.some(([tx, ty]) => [x - 0.5, x + 0.5].includes(tx) && ty === y);
          }),
          c: ' ',
          isFake: true,
        });
      } else if (Math.floor(y) !== y) {
        expanded.set(k, {
          x, y,
          links: [
            [x, y - 0.5],
            [x, y + 0.5],
          ].filter(([lx, ly]) => {
            const lk = `${lx},${ly}`;
            const node = map.get(lk);
            if (!node) return false; // Probably out of range, not likely an error
            return node.links.some(([tx, ty]) => tx === x && [y - 0.5, y + 0.5].includes(ty));
          }),
          c: ' ',
          isFake: true,
        });
      } else {
        throw new Error(`Should not be possible: even coords ${k} missing from original map!`);
      }
    }
  }
  return expanded;
}

// TL;DR: Flood fill, squeezing between gaps; answer is #nodes that can't be reached.
// - Stretch the grid out and extend links, for the squeezing requirement.
// - Flood fill, starting at a ring outside of boundaries.
// - Flood will affect any node that is NOT part of the loop containing S.
// - Answer is the number of dots in the original map that the flood doesn't find.
const part2 = ({ start, map: original, w, h }) => {

  // Before we begin, we need to stretch the map out in both directions
  // so that flood-filling can squeeze between the gaps without a border crossing algorithm.
  const map = expandForSqueezing({ map: original, w, h });

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
      [ux - 0.5, uy],
      [ux + 0.5, uy],
      [ux, uy - 0.5],
      [ux, uy + 0.5],
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

  // Finally: count how many dots (in the unmodified map) were not found by the flood fill
  return [...original.keys()].filter(k => !explored.has(k) && !mainLoopExplored.has(k)).length;
};

aoc(2023, 10, part1, part1expected, part2, part2expected, parse);
