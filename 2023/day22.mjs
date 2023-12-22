import aoc from './aoc.mjs';

/** @typedef {number} Point3dKey (x * 1e8 + y * 1e4 + z), all coordinates are nonnegative integers <= 999 */
/** @typedef {number} BrickId Just the index into the bricks array (don't shuffle them!) */
/** @typedef {[[x1: number, y1: number, z1: number], [x2: number, y2: number, z2: number]]} Brick */
/** @typedef {Map<Point3dKey,BrickId>} Grid */

/** @typedef {{ bricks: Brick[], grid: Grid }} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 5;

/** @type Part2Solution */
const part2expected = 7;

const pointToKey = (x, y, z) => x * 1e8 + y * 1e4 + z;
const keyToPoint = (key) => {
  const x = Math.floor(key / 1e8);
  const y = Math.floor((key - (x * 1e8)) / 1e4);
  const z = key - (x * 1e8) - (y * 1e4);
  return [x, y, z];
};

/**
 * @param {Brick} brick 
 * @returns [x: number, y: number, z: number][]
 */
const getBrickCells = ([[x1, y1, z1], [x2, y2, z2]]) => {
  const touched = [];
  const xLo = Math.min(x1, x2);
  const xHi = Math.max(x1, x2);
  const yLo = Math.min(y1, y2);
  const yHi = Math.max(y1, y2);
  const zLo = Math.min(z1, z2);
  const zHi = Math.max(z1, z2);
  for (let x = xLo; x <= xHi; x++) {
    for (let y = yLo; y <= yHi; y++) {
      for (let z = zLo; z <= zHi; z++) {
        touched.push([x, y, z]);
      }
    }
  }
  return touched;
};

/**
 * @param {Brick} brick
 * @returns {boolean}
 */
const isBrickGrounded = ([[,,z1], [,,z2]]) => z1 <= 1 || z2 <= 1;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  /** @type Brick[] */
  const bricks = lines.map(l => l.split('~').map(h => h.split(',').map(n => +n)));

  // Before locking in IDs, sort by lowest to the ground first (ties are broken by shortest brick)
  // This should make iteration a fair bit faster in part 1.
  bricks.sort((a, b) => {
    const closerToGround = Math.min(a[0][2], a[1][2]) - Math.min(b[0][2], b[1][2]);
    const shorter = Math.abs(a[0][2] - a[1][2]) - Math.abs(b[0][2] - b[1][2]);
    if (closerToGround !== 0) return closerToGround;
    return shorter;
    // A tie is still possible (two 1x1x1 cubes on the ground next to each other),
    // but (1) they'll just stay in the same order in the array in that case, and
    // (2) we only really care to start iterating from the ground up.
  });

  /** @type Grid */
  const grid = new Map();

  for (let b = 0; b < bricks.length; b++) {
    for (const [x, y, z] of getBrickCells(bricks[b])) {
      grid.set(pointToKey(x, y, z), b);
    }
  }

  return { bricks, grid };
};

const getSupportingBricks = (bricks, grid, brickId) => {
  const [[x1, y1, z1], [x2, y2, z2]] = bricks[brickId];

  const searchZ = Math.min(z1, z2) - 1;
  if (searchZ <= 0) return new Set(); // Brick is on the ground.

  const supportingBrickIds = new Set();
  for (let x = Math.min(x1, x2); x <= Math.max(x1, x2); x++) {
    for (let y = Math.min(y1, y2); y <= Math.max(y1, y2); y++) {
      const k = pointToKey(x, y, searchZ);
      if (grid.get(k) != null) {
        supportingBrickIds.add(grid.get(k));
      }
    }
  }
  return supportingBrickIds;
};

const getSupportedBricks = (bricks, grid, brickId) => {
  const [[x1, y1, z1], [x2, y2, z2]] = bricks[brickId];
  const searchZ = Math.max(z1, z2) + 1;

  const supportingBrickIds = new Set();
  for (let x = Math.min(x1, x2); x <= Math.max(x1, x2); x++) {
    for (let y = Math.min(y1, y2); y <= Math.max(y1, y2); y++) {
      const k = pointToKey(x, y, searchZ);
      if (grid.get(k) != null) {
        supportingBrickIds.add(grid.get(k));
      }
    }
  }
  return supportingBrickIds;
};

/**
 * Attempt to move the brick numbered brickId down towards the ground
 * until it collides with something.
 * 
 * Returns the list of bricks that it is colliding with (those are now supporting it),
 * and mutates the bricks list and the grid automatically.
 *
 * @param {Brick[]} bricks 
 * @param {Grid} grid 
 * @param {BrickId} brickId 
 * @returns {{ didMove: boolean, grounded: boolean, blockedBy: BrickId[] }}
 */
const moveBrickDown = (bricks, grid, brickId) => {
  let didMove = false;
  let grounded = false;
  let blockedBy = [];
  do {
    if (isBrickGrounded(bricks[brickId])) {
      grounded = true;
      break;
    }
    const unders = getSupportingBricks(bricks, grid, brickId);
    if (unders.size) {
      blockedBy = [...unders];
      break;
    }

    // Actually move the brick down, since it's clear to do so
    // Remove it from the grid, then mutate it, then add it back to the grid.
    for (const [x, y, z] of getBrickCells(bricks[brickId])) {
      grid.delete(pointToKey(x, y, z));
    }
    bricks[brickId][0][2]--;
    bricks[brickId][1][2]--;
    for (const [x, y, z] of getBrickCells(bricks[brickId])) {
      grid.set(pointToKey(x, y, z), brickId);
    }
    // We'll keep doing this until we hit something
    didMove = true;
  } while (true);

  return { didMove, grounded, blockedBy };
};

/**
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ bricks, grid }) => {
  // First, drop all bricks to the ground or until they're supported entirely by grounded bricks
  // Since we sorted closest-to-ground-first in parse(), this should resolve quickly enough.
  while (true) {
    let anyMoved = false;
    for (let brickId = 0; brickId < bricks.length; brickId++) {
      const { didMove } = moveBrickDown(bricks, grid, brickId);
      if (didMove) anyMoved = true;
    }
    if (!anyMoved) break;
  }

  // The answer is the number of bricks which, if removed, would NOT remove the last support
  // from a brick that's above them
  let removable = 0;
  for (let brickId = 0; brickId < bricks.length; brickId++) {
    const supporting = [...getSupportedBricks(bricks, grid, brickId)];
    if (supporting.every(s => getSupportingBricks(bricks, grid, s).size > 1)) {
      removable++;
    }
  }
  return removable;
};

/**
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = ({ bricks, grid }) => {
  // First, let the bricks fall as we do in part 1.
  while (true) {
    let anyMoved = false;
    for (let brickId = 0; brickId < bricks.length; brickId++) {
      const { didMove } = moveBrickDown(bricks, grid, brickId);
      if (didMove) anyMoved = true;
    }
    if (!anyMoved) break;
  }

  // Next, for each brick, simulate what would happen if it were removed.
  // We don't need to let bricks fall again, we only need to check which bricks would
  // fall from removing the target brick, pretend we removed that one too,
  // and repeat until we stop finding theoretically-unsupported bricks.
  // Each brick gets a count of bricks (besides itself) that would fall if it were removed this way.
  // The answer is the sum of those counts.
  let sumNumWouldFall = 0;;
  for (let brickId = 0; brickId < bricks.length; brickId++) {
    let numWouldFall = -1;

    /** @type Set<BrickId> */
    const removed = new Set();

    /** @type BrickId[] */
    let targets = [brickId];

    while (targets.length) {
      const target = targets.shift();
      if (removed.has(target)) continue;
      numWouldFall++;
      removed.add(target);
      for (const aboveId of getSupportedBricks(bricks, grid, target)) {
        const belowIds = getSupportingBricks(bricks, grid, aboveId);
        if (belowIds.size === 0 || [...belowIds].every((belowId) => removed.has(belowId))) {
          targets.push(aboveId);
        }
      }
    }

    sumNumWouldFall += numWouldFall;
  }
  return sumNumWouldFall;
};

aoc({
  year: 2023,
  day: 22,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
