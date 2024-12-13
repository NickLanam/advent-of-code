import aoc from './aoc.mjs';

/** @typedef {{a: {x: number, y: number}, b: {x: number, y: number}, c: {x: number, y: number}}[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 480;

/** @type Part2Solution */
const part2expected = 875318608908;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  const partMod = forPart === 1 ? 0 : 10_000_000_000_000;
  const grouped = lines.join('\n').split('\n\n').map(group => group.split('\n'));
  return grouped.map(([a, b, p]) => {
    const axy = a.match(/^Button A: X\+(\d+), Y\+(\d+)$/);
    const bxy = b.match(/^Button B: X\+(\d+), Y\+(\d+)$/);
    const pxy = p.match(/^Prize: X=(\d+), Y=(\d+)$/);
    return {
      a: { x: +axy[1], y: +axy[2] },
      b: { x: +bxy[1], y: +bxy[2] },
      p: { x: +pxy[1] + partMod, y: +pxy[2] + partMod },
    };
  })
};

/**
 * 
 * @param {ParsedInput} groups 
 * @returns {Part1Solution}
 */
const part1Naive = (groups) => {
  return groups.map(({ a, b, p }) => {
    let minPresses = Infinity;
    for (let ap = 0; ap <= 100; ap++) {
      let x = a.x * ap;
      let y = a.y * ap;
      if (x > p.x || y > p.y) break;
      for (let bp = 0; bp <= 100; bp++) {
        let x2 = x + bp * b.x;
        let y2 = y + bp * b.y;
        if (x2 > p.x || y2 > p.y) break;
        if (x2 === p.x && y2 === p.y) {
          minPresses = Math.min(minPresses, ap * 3 + bp);
        }
      }
    }
    return minPresses < Infinity ? minPresses : 0;
  }).reduce((a, c) => a + c, 0);
  // My answer comes out to 31552 this way in about 4ms.
  // This approach quickly gets to O(never) for part 2.
};

/**
 * 
 * @param {ParsedInput} groups 
 * @param {boolean} isPart1 
 * @param {boolean} isSample
 * @returns {number}
 */
function solveGCDLCM(groups, isPart1, isSample) {
  // Idea: GCD/LCM
  // Result: This doesn't even make sense, we need ap & bp separately
  const gcd = (a, b) => b === 0 ? a : gcd(b, a % b);
  const lcm = (a, b) => a / gcd(a, b) * b;
  return groups.map(({ a, b, p }) => {
    const gcdX = gcd(a.x, b.x);
    const lcmX = lcm(a.x, b.x);
    const gcdY = gcd(a.y, b.y);
    const lcmY = lcm(a.y, b.y);
    const gcdXY = gcd(lcmX, lcmY);
    const lcmXY = lcm(lcmX, lcmY);
    if (isSample) console.info({ a, b, p, gcdX, gcdY, lcmX, lcmY, gcdXY, lcmXY });
    return 0;
  }).reduce((a, c) => a + c, 0);
}

/**
 * Turn it into a geometry problem.
 * We have a point (the goal), and two slopes.
 * We can compute the y-intercept of the second line
 * by assuming it touches the goal, and set the y-intercept
 * of the first line to 0.
 * 
 * Then we do a bit of algebra to find where the two lines
 * intersect, and that tells us how many times to press
 * each button.
 * 
 * @param {ParsedInput} groups 
 * @param {boolean} isPart1 
 * @param {boolean} isSample
 * @returns {number}
 */
function solve(groups, isPart1, isSample) {
  return groups.map(({ a: p0, b: p1, p: goal }) => {
    const m0 = p0.y / p0.x;
    const b0 = 0;
    
    const m1 = p1.y / p1.x;
    const b1 = -m1 * goal.x + goal.y;
    
    // Edge case: if the cheaper of the two buttons has the same
    // slops as the goal point, then only press that button.
    const mg = goal.y / goal.x;
    if (mg === m1) {
      return goal.x / p1.x;
    }

    // m0 * x + b0 = m1 * x + b1
    // (m0-m1)*x = b1-b0
    // Intersection is at x = (b1-b0)/(m0-m1)
    const x = (b1 - b0) / (m0 - m1);
    // Expanding it doesn't change the precision (that's good)
    // (but also means my math is slightly off somehow)
    // const x = ((-(p1.y*goal.x) / p1.x) + goal.y) / (p0.y / p0.x - p1.y / p1.x);
    if (!Number.isNaN(x) && x < Infinity) {
      const aPress = Math.round(x / p0.x);
      const bPress = (goal.x - (p0.x * aPress)) / p1.x;
      if (isPart1 && (aPress > 100 || bPress > 100)) return 0;
      if (Math.floor(bPress) !== bPress) return 0;
      // if (isSample) console.info({ p0, p1, goal, x, m0, b0, m1, b1, mg, aPress, bPress });
      return aPress * 3 + bPress;
    } else {
      return 0;
    }
  }).reduce((a, c) => a + c, 0);
  // Note: NOT doing any rounding makes part 1 go too low (way too low)
  // This solution gets the right answer for part 1 (sample AND real).
  // But on part 2: 99_021_065_350_198 is too high (no round on line 118, do round on line 120, or round both)
  //                96_650_856_775_202.9: Still too high (round on line 118, no round on line 120)
  //                70_332_781_361_962: Too low (no rounding); gets wrong answer for part 1
  //                74_366_652_648_490: Ran out of hints. Also still wrong for part 1.
  // Good news is it correctly does as the description says: only groups 2 and 4 have a match, so... what's going on here?
  // Seems like there's floating point issues here.
}

aoc({
  year: 2024,
  day: 13,
  part1: (g, isSample) => solve(g, true, isSample),
  part1expected,
  part2: (g, isSample) => solve(g, false, isSample),
  part2expected,
  parse,
});
