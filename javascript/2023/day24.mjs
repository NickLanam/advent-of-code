import aoc from './aoc.mjs';

/** @typedef {{px: number, py: number, pz: number, vx: number, vy: number, vz: number}[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 2;

/** @type Part2Solution */
const part2expected = 47;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  return lines
    .map(line => line
      .split(' @ ')
      .map(half => half.split(', ').map(n => +n)))
    .map(([[px, py, pz], [vx, vy, vz]]) => ({
      px,
      py,
      pz,
      vx,
      vy,
      vz,
      m: vy / vx,
      b: py - (vy / vx) * px,
    }));
};

/**
 * @param {ParsedInput} equations
 * @param {boolean} isSample
 * @returns {Part1Solution}
 */
const part1 = (equations, isSample) => {
  const minRange = isSample ? 7 : 200_000_000_000_000;
  const maxRange = isSample ? 27 : 400_000_000_000_000;
  
  let collisionsInBand = 0;
  for (let i = 0; i < equations.length - 1; i++) {
    for (let j = i + 1; j < equations.length; j++) {
      const ea = equations[i];
      const eb = equations[j];
      const x = (eb.b - ea.b) / (ea.m - eb.m);
      const y = ea.m * (eb.b - ea.b) / (ea.m - eb.m) + ea.b;
      const inBounds = x >= minRange && x <= maxRange && y >= minRange && y <= maxRange;
      const isFutureA = Math.sign(x - ea.px) === Math.sign(ea.vx) && Math.sign(y - ea.py) === Math.sign(ea.vy);
      const isFutureB = Math.sign(x - eb.px) === Math.sign(eb.vx) && Math.sign(y - eb.py) === Math.sign(eb.vy);
      if (inBounds && isFutureA && isFutureB) {
        collisionsInBand++;
      }
    }
  }
  return collisionsInBand;
};

/**
 * @param {ParsedInput} equations 
 * @returns {Part2Solution}
 */
const part2 = (equations) => {
  // Shameful though it is, I failed to figure out all of my off-by-one errors.
  // I followed these tips, and eventually ended up copy-pasting most of the linked
  // solution in this thread:
  // https://www.reddit.com/r/adventofcode/comments/18pptor/comment/keqwg08/?context=1

  // This is why the input is translated to a different form than my part 1 solution first.

  const SUB = equations
    .slice(0, 10)
    .map(({ px, py, pz, vx, vy, vz }) => [px, py, pz, vx, vy, vz]);

  function findLineLambda(a, b, c, d, p, q, r, s) {
    const det = (c - a) * (s - q) - (r - p) * (d - b);
    if (det === 0) return null;
    return Math.round(((s - q) * (r - a) + (p - r) * (s - b)) / det);
  }
 
  function findIntersectionPoint(a, b, d1 = 0, d2 = 1) {
    const lambda = findLineLambda(
      a[d1], a[d2], a[d1] + a[d1 + 3], a[d2] + a[d2 + 3],
      b[d1], b[d2], b[d1] + b[d1 + 3], b[d2] + b[d2 + 3],
    )
    if (lambda === null) return null;
 
    let f = a[d1] + lambda * a[d1 + 3];
    let g = a[d2] + lambda * a[d2 + 3];
    return [f, g];
  }
 
  function findCommonIntersection(v, d1 = 0, d2 = 1) {
    let viable = true;
    let current;
    const transformed = SUB.map(line => {
      const copy = [...line]
      copy[d1 + 3] += v[0]
      copy[d2 + 3] += v[1]
      return copy
    });

    for (let i = 0; i < transformed.length - 1; i++) {
      for (let j = i + 1; j < transformed.length; j++) {
        if (!viable) continue;
        const ea = transformed[i];
        const eb = transformed[j];
        const point = findIntersectionPoint(ea, eb, d1, d2);
        if (!point) continue;
        if (!current) current = point;
        viable = point[0] === current[0] && point[1] === current[1];
      }
    }

    if (!viable) return false;
    return current;
  }

  for (let vx = 0; vx <= Infinity; vx++) {
    for (let vy = 0; vy <= vx; vy++) {
      for (let [sx, sy] of [[1, 1], [1, -1], [-1, 1], [-1, -1]]) {
        // X and Y have an intersection
        let xy = findCommonIntersection([vx * sx, vy * sy], 0, 1);
        if (!xy) continue;

        for (let vz = 0; vz <= Infinity; vz++) {
          for (let sz of [1, -1]) {
            // find intersection for X and Z, if it exists, it's the result
            let xz = findCommonIntersection([vx * sx, vz * sz], 0, 2);
            if (!xz) continue;
            return xy[0] + xy[1] + xz[1];
          }
        }
      }
    }
  }

  throw new Error('Failed to find');
};

aoc({
  year: 2023,
  day: 24,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
