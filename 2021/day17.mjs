import { triangulate, triangularRoot } from './utils/number.mjs';

// Test if x and y are within the hitbox.
function hit(x, y, xmin, xmax, ymin, ymax) {
  return x >= xmin && x <= xmax && y >= ymin && y <= ymax;
}

function paths(xmin, xmax, ymin, ymax) {
  const minVelX = Math.ceil(triangularRoot(xmin)); // First X that at least reaches the target before vx=0
  const maxVelX = xmax; // Last X that definitely reaches the target at least once
  
  const validVelocities = [];
  let bestPeak = 0;
  for (let ivx = minVelX; ivx <= maxVelX; ivx++) {
    const xPeak = triangulate(ivx);
    for (let ivy = ymin; ivy < Math.abs(ymin); ivy++) {
      const yPeak = ivy > 0 ? triangulate(ivy) : 0;

      const numSteps = (ivy < 0 ? 0 : 2 * ivy) + Math.ceil(Math.abs(triangularRoot(ymin))) + 2;
      let xs = Array(numSteps).fill(0).map((_, i) => xPeak - Math.abs(triangulate(ivx - Math.min(ivx, i))));
      let ys = Array(numSteps).fill(0).map((_, i) => {
        if (ivy >= 0) {
          return yPeak - Math.abs(triangulate(ivy - (i <= ivy ? i : i - 1)));
        } else {
          return triangulate(ivy - i + 1) - triangulate(ivy + 1);
        }
      });
      const hits = xs.map((x, i) => [x, ys[i]]).filter(([x, y]) => hit(x, y, xmin, xmax, ymin, ymax));

      if (hits.length) {
        validVelocities.push([ivx, ivy]);
        bestPeak = Math.max(bestPeak, yPeak);
      }
    }
  }
  return { validVelocities, bestPeak };
}

(await import('./aoc.mjs')).default(
  2021, 17,
  ({ xmin, xmax, ymin, ymax }) => paths(xmin, xmax, ymin, ymax).bestPeak, 45,
  ({ xmin, xmax, ymin, ymax }) => paths(xmin, xmax, ymin, ymax).validVelocities.length, 112,
  data => {
    const [, xmin, xmax, ymin, ymax] = data[0].match(/^target area: x=([-\d]+)\.\.([-\d]+), y=([-\d]+)\.\.([-\d]+)$/);
    return { xmin: +xmin, xmax: +xmax, ymin: +ymin, ymax: +ymax };
  }
);