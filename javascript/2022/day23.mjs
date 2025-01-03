const p2s = ([x, y]) => `${x},${y}`;
const s2p = (s) => s.split(',').map(n => +n);

function parse(lines) {
  const elves = new Set();
  for (let y = 0; y < lines.length; y++) {
    for (let x = 0; x < lines[y].length; x++) {
      if (lines[y].substring(x, x + 1) === '#') {
        elves.add(p2s([x, y]));
      }
    }
  }
  return elves;
}

function proposeMoves(elfSet, roundNumber) {
  const res = new Map(); // `${toX},${toY}` -> [[elfX, elfY], ...]
  for (const elf of elfSet) {
    const [x, y] = s2p(elf);
    const neighbors = [
      [x - 1, y - 1], [x,     y - 1], [x + 1, y - 1],
      [x - 1, y    ], [x,     y    ], [x + 1, y    ],
      [x - 1, y + 1], [x,     y + 1], [x + 1, y + 1],
    ].map(p => elfSet.has(p2s(p)));
    // An elf does not move if it has no neighbors (including diagonals)
    if (neighbors.reduce((a, n) => a + n, 0) > 1) {
      // Check each direction [North, South, West, East] (rotate with roundNumber).
      // The first one where there are no neighbors in the three slots facing that way
      // is the one this elf proposes.
      let prop = null;
      for (let d = 0; d < 4; d++) {
        let dd = (d + roundNumber) % 4; // Rotate which direction we check first each round
        if (dd === 0 /* North */ && !neighbors[0] && !neighbors[1] && !neighbors[2]) prop = [x, y - 1];
        if (dd === 1 /* South */ && !neighbors[6] && !neighbors[7] && !neighbors[8]) prop = [x, y + 1];
        if (dd === 2 /* West  */ && !neighbors[0] && !neighbors[3] && !neighbors[6]) prop = [x - 1, y];
        if (dd === 3 /* East  */ && !neighbors[2] && !neighbors[5] && !neighbors[8]) prop = [x + 1, y];
        if (prop) break;
      }
      if (prop) {
        const elvesWantingThatPosition = res.get(p2s(prop)) ?? [];
        elvesWantingThatPosition.push([x, y]);
        res.set(p2s(prop), elvesWantingThatPosition);
      }
    }
  }
  return res;
}

function performMoves(elfSet, proposals) {
  // Each elf moves to its proposed position if it is the only one that proposed THAT position.
  for (const [pos, elves] of proposals) {
    if (elves.length > 1) continue;
    elfSet.delete(p2s(elves[0]));
    elfSet.add(pos);
  }
}

function score(elfSet) {
  let ret = 0;
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  for (const elf of elfSet) {
    const [x, y] = s2p(elf);
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
  }
  for (let py = minY; py <= maxY; py++) {
    for (let px = minX; px <= maxX; px++) {
      if (!elfSet.has(p2s([px, py]))) ret++;
    }
  }
  return ret;
}

function solve(elfSet, maxRounds) {
  let round;
  for (round = 0; round < maxRounds; round++) {
    const prevElves = new Set(elfSet);
    const props = proposeMoves(elfSet, round);
    performMoves(elfSet, props);
    if (prevElves.size === elfSet.size && [...prevElves].every(e => elfSet.has(e))) {
      break;
    }
  }
  if (maxRounds < Infinity) {
    return score(elfSet);
  } else {
    return round + 1;
  }
}

(await import('./aoc.mjs')).default(
  2022, 23,
  (elfSet) => solve(elfSet, 10), 110,
  (elfSet) => solve(elfSet, Infinity), 20,
  parse, true, false
);