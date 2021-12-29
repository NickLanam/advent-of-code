function volume({ xmin, xmax, ymin, ymax, zmin, zmax }) {
  return (1 + Math.abs(xmax - xmin)) * (1 + Math.abs(ymax - ymin)) * (1 + Math.abs(zmax - zmin));
}

function intersect(space, cuboid) {
  if (
    space.xmin > cuboid.xmax || space.xmax < cuboid.xmin ||
    space.ymin > cuboid.ymax || space.ymax < cuboid.ymin ||
    space.zmin > cuboid.zmax || space.zmax < cuboid.zmin
  ) return null; // Cuboid is entirely outside of the space

  if (
    space.xmin <= cuboid.xmin && space.xmax >= cuboid.xmax &&
    space.ymin <= cuboid.ymin && space.ymax >= cuboid.ymax &&
    space.zmin <= cuboid.zmin && space.zmax >= cuboid.xmax
  ) return cuboid; // Cuboid is entirely inside of the space

  return {
    xmin: Math.max(space.xmin, cuboid.xmin),
    xmax: Math.min(space.xmax, cuboid.xmax),
    ymin: Math.max(space.ymin, cuboid.ymin),
    ymax: Math.min(space.ymax, cuboid.ymax),
    zmin: Math.max(space.zmin, cuboid.zmin),
    zmax: Math.min(space.zmax, cuboid.zmax),
  };
}

// Brute forcing part 1 doesn't work, so you end up with a more optimal solution anyway.
// Therefore, the region restriction in part 1 is actually an extra step at the beginning.
function clampSteps(steps) {
  const space = { xmin: -50, xmax: 50, ymin: -50, ymax: 50, zmin: -50, zmax: 50 };
  const clamped = [];
  for (const step of steps) {
    const result = intersect(space, step);
    if (result) {
      result.action = step.action;
      clamped.push(result);
    }
  }
  return clamped;
}

/** Returns a list of cuboids that describe what remains of source after cutting a hole. */
function cut(source, hole) {
  // If the two cuboids don't intersect at all, return the original source.
  if (
    hole.xmin > source.xmax || hole.xmax < source.xmin ||
    hole.ymin > source.ymax || hole.ymax < source.ymin ||
    hole.zmin > source.zmax || hole.zmax < source.zmin
  ) return [source];

  // If the hole completely covers the source, return an empty list.
  if (
    hole.xmin <= source.xmin && hole.xmax >= source.xmax &&
    hole.ymin <= source.ymin && hole.ymax >= source.ymax &&
    hole.zmin <= source.zmin && hole.zmax >= source.zmax
  ) return [];

  // Worst case, the hole is right in the center of the source. We can use six cuboids
  // to represent what remains: a wall to the left, a wall to the right, a ceiling in
  // the remaining space above, a floor in the remaining space below, then two more
  // walls to plug the ends.
  // If the hole isn't completely inside the source, then some of these can be skipped.
  const intersection = intersect(source, hole);
  const out = [];

  // First two walls: all of the source that's outside the hole in one dimension
  if (intersection.xmin > source.xmin) {
    out.push({ ...source, xmax: intersection.xmin - 1 });
  }
  if (intersection.xmax < source.xmax) {
    out.push({ ...source, xmin: intersection.xmax + 1 });
  }

  // Next two walls: same, but stop at the first walls we already filled in
  if (intersection.ymin > source.ymin) {
    out.push({ ...source, xmin: intersection.xmin, xmax: intersection.xmax, ymax: intersection.ymin - 1 });
  }
  if (intersection.ymax < source.ymax) {
    out.push({ ...source, xmin: intersection.xmin, xmax: intersection.xmax, ymin: intersection.ymax + 1 });
  }

  // Final walls: only extrude in one axis since the other two are already covered
  if (intersection.zmin > source.zmin) {
    out.push({ ...intersection, zmin: source.zmin, zmax: intersection.zmin - 1 });
  }
  if (intersection.zmax < source.zmax) {
    out.push({ ...intersection, zmin: intersection.zmax + 1, zmax: source.zmax });
  }

  if (out.length < 1 || out.length > 6) {
    throw new Error('Missed a case! Or math is off.');
  }

  return out;

}

/**
 * Rough approach:
 * - For each cuboid that intersects the new one,
 *   remove that one and add several that don't intersect the addition.
 *   The hard part is figuring out how to break the old cuboid into smaller ones.
 * - If the addition is an ON step, also add its cuboid at the end,
 *   since it no longer overlaps anything.
 * - At the end, the total volume of all cuboids is the number of active coords.
 * - This is another challenge that's easy to understand, but hard to write.
 */
 function solve(steps) {
  const cuboids = [];
  for (const step of steps) {
    const newcomer = { ...step };
    delete newcomer.action;

    const prevCount = cuboids.length;
    for (let i = 0; i < prevCount; i++) {
      const toSplit = cuboids.shift();
      cuboids.push(...cut(toSplit, newcomer));
    }
    if (step.action) cuboids.push(newcomer);
  }

  // Sum volume of what remains.
  return cuboids.reduce((a, c) => a + volume(c), 0);
}

(await import('./aoc.mjs')).default(
  2021, 22,
  steps => solve(clampSteps(steps)), 474140,
  // Note: Number.MAX_SAFE_INTEGER = 9_007_199_254_740_991. Therefore, this is fine.
  // But for values >= 2**53 - 1, it isn't fine to mix with non-integer values.
  steps => solve(steps), 2_758_514_936_282_235,
  lines => lines.map(line => {
    let [, action, ...rest] = line.match(
      /^(on|off) x=([-\d]+)\.\.([-\d]+),y=([-\d]+)\.\.([-\d]+),z=([-\d]+)\.\.([-\d]+)$/
    );
    action = action === 'on';
    const [xmin, xmax, ymin, ymax, zmin, zmax] = rest.map(n => +n);
    return { action, xmin, xmax, ymin, ymax, zmin, zmax };
  })
);