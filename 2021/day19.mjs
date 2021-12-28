import { range } from './utils/array.mjs';
import { Point3D } from './data-structures/point.mjs';

/*
 * Day 19. The hard one.
 *
 * Some essential details:
 * - The scanners aren't all in the same place. They need to be translated AND oriented.
 *   We'll align everything relative to scanner 0. Its location is the origin; it faces +z with "up" being +y.
 * - We're trying to find the scanners AND the beacons.
 *   Part 1 wants to know how many BEACONS there are (sans duplicates).
 *   Part 2 wants the greatest manhattan distance between any two SCANNERS.
 * - 6 directions * 4 rotations * 2 mirrors = 48 possible permutations of a point.
 * - A scanner only counts if it has 12 or more beacons in common with another scanner.
 *
 * The approach:
 * - If there are two beacons that were both picked up by different scanners,
 *   even if those scanners are in different orientations and positions,
 *   the distance between those beacons remains the same.
 * - By finding pairs of points that exist in two scans, we can know
 *   which points the two have in common. We also can figure out which point is which,
 *   giving us enough info to figure out the right rotation and translation.
 *   That transform applies to all points in the second scan.
 *   The inverse of the translation is also the position of the second scanner.
 * - Note: there may not be a link directly to scanner 0 from another scanner.
 *   It may be that we have to transform multiple times after finding links.
 * - Each time we do this, we learn the position of a scanner and the scanner-0-relative
 *   coordinates of each beacon that the scanner found.
 * - Add that scanner to the list, and add each found beacon to a unique set.
 * - The size of the set is the answer to part 1.
 * - The greatest manhattan distance between any two scanners is the answer to part 2.
 *
 * Limitations of this solution:
 * - Pairs of beacons that have the same distance as other pairs break disambiguation.
 * - This algorithm has exponential complexity. Large scans or many scans get very slow.
 * - The requirement for N beacons in common between two scanners doesn't account for multiple clusters.
 *   There could be two pairs of scanners that are close to their partner, but far from the other group.
 *
 * Properties of the input that make these limitations okay:
 * - There are always less than 50 scanners
 * - Each scanner always picks up less than 30 beacons
 * - CHECK REAL INPUT: Every scanner has at exactly 66 edges in common with at least one other beacon.
 * - CHECK REAL INPUT: Every beacon has a chain of these connections to find scanner 0.
 *   If this holds, then we don't have disparate groups or scanners that should be discarded.
 * - CHECK REAL INPUT: In the sample input, every scanner has exactly 66 edges in common with at least one other beacon. It may have a smaller number in common with others.
 *   If this holds in real data, it means there are no scenarios where three beacons are in a perfect triangle in a single scanner's vision. This skirts disambiguation issues.
 *   This also makes it possible to pick the most common working alignment and be sure it's the correct one and not a coincidence.
 */

// These leave the diagonal in (comparing a point to itself), so that indices stay consistent.
const manhattanScan = scan => scan.map(p1 => scan.map(p2 => Point3D.manhattanDistance(p1, p2)));
const squaredDistScan = scan => scan.map(p1 => scan.map(p2 => Point3D.squaredDistance(p1, p2)));

function findLinks(scans) {
  const links = Array(scans.length).fill(0).map(_ => Array(scans.length).fill(0));
  for (let s0 of range(scans.length - 1)) {
    const d0 = squaredDistScan(scans[s0]);
    for (let s1 of range(s0 + 1, scans.length)) {
      const d1 = squaredDistScan(scans[s1]);

      for (let i of range(d0.length - 1)) {
        for (let ii of range(i + 1, d0[i].length)) {
          for (let j of range(d1.length - 1)) {
            for (let jj of range(j + 1, d1[j].length)) {
              if (d0[i][ii] === d1[j][jj]) {
                links[s0][s1]++;
                links[s1][s0]++;
              }
            }
          }
        }
      }
    }
  }
  return links.map(outer => outer.map((inner, s1) => inner >= 66 ? s1 : false).filter(v => v !== false));
}

function findPathToZero(links, fromIndex) {
  if (fromIndex === 0) return [0];

  const pending = [[fromIndex]];
  let LIM = 1_000;
  while (pending.length && --LIM > 0) {
    const look = pending.pop();
    let options = links[look[look.length - 1]].filter(v => !look.includes(v));
    for (const next of options) {
      const newPath = [...look, next];
      if (next === 0) return newPath;
      else pending.push(newPath);
    }
  }
  throw new Error(`Path to 0 from ${fromIndex} was not found in 1,000 steps.`);
}

// Finds a rotation+translation that, when applied to every point in source, gets the most overlapped points with target.
// Looping over this is the overwhelming majority of the run time. It can likely be made way faster.
function findFieldTransform(target, source) {
  const found = [];
  for (let r of range(48)) {
    const rotated = source.map(p => p.orient(r));
    const freq = {};
    let best = -Infinity;
    for (const p0 of rotated) {
      for (const p1 of target) {
        const dist = Point3D.difference(p0, p1);
        const s = dist.join(',');
        freq[s] = (freq[s] ?? 0) + 1;
        best = Math.max(freq[s], best);
      }
    }
    if (best >= 12) {
      const translations = Object.keys(freq).filter(k => freq[k] === best);
      for (const t of translations) {
        found.push({ count: best, r, t: Point3D.fromString(t) });
      }
    }
  }
  if (found.length !== 1) throw new Error(`Found ${found.length} with ${best} overlaps; should be exactly 1.`);
  return found[0];
}

function findTransforms(scans) {
  const links = findLinks(scans);
  const transforms = links.map((link, i) => link.map(j => ({
    to: i,
    from: j,
    transform: findFieldTransform(scans[i], scans[j]),
  }))).flat();

  const pathsToZero = [...range(scans.length)].map(i => findPathToZero(links, i));

  const out = [];
  
  for (let i of range(scans.length)) {
    const path = pathsToZero[i];
    let forward = [];
    let backward = [];
    for (let pi of range(1, path.length)) {
      const from = path[pi - 1];
      const to = path[pi];
      forward.push(transforms.find(t => t.from === from && t.to === to));
      backward.unshift(transforms.find(t => t.to === from && t.from === to));
    }
    out.push({ forward, backward });
  }

  // Now we have a converter to get to scanner 0 from each other scanner.
  return out;
}

function applyTransform(transform, point) {
  return point.orient(transform.r).translate(...transform.t.toArray());
}

function applyTransformChain(chain, point) {
  let result = point;
  for (const link of chain) {
    result = applyTransform(link.transform, result);
  }
  return result;
}

function buildMap(scans) {
  // Use scanner 0 as the origin point, such that it faces +z, and its "up" is +y.
  // All other scanners and beacons will have their coordinates described relative to this.
  const beacons = new Set();
  const scanners = [];

  const chains = findTransforms(scans);

  for (let c of range(chains.length)) {
    scanners.push(applyTransformChain(chains[c].forward, new Point3D(0, 0, 0)));
  }

  for (const s of range(scans.length)) {
    for (const point of scans[s]) {
      beacons.add(applyTransformChain(chains[s].forward, point).toString());
    }
  }

  return {
    beacons: [...beacons].map(b => Point3D.fromString(b)),
    scanners,
  };
}

(await import('./aoc.mjs')).default(
  2021, 19,
  scans => buildMap(scans).beacons.length, 79,
  scans => manhattanScan(buildMap(scans).scanners).flat().sort((a, b) => b - a)[0], 3621,
  lines => lines.join('\n').split('\n\n')
    .map(group => group.split('\n').slice(1) // Strip the header
      .map(line => Point3D.fromString(line)))
);