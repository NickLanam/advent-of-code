function toCloud(points) {
  let minx=Infinity, miny=Infinity, minz=Infinity, maxx=-Infinity, maxy=-Infinity, maxz=-Infinity;
  for (const point of points) {
    minx = Math.min(minx, point[0]);
    maxx = Math.max(maxx, point[0]);
    miny = Math.min(miny, point[1]);
    maxy = Math.max(maxy, point[1]);
    minz = Math.min(minz, point[2]);
    maxz = Math.max(maxz, point[2]);
  }

  const cloud = Array(maxz + 1).fill(null)
    .map(() => Array(maxy + 1).fill(null)
      .map(() => Array(maxx + 1).fill(false)));

  for (const point of points) {
    try {
      cloud[point[2]][point[1]][point[0]] = true;
    } catch (e) {
      console.error(point, cloud.length, cloud[0].length, cloud[0][0].length);
      throw e;
    }
  }

  return cloud;
}

(await import('./aoc.mjs')).default(
  2022, 18,
  (cloud) => {
    let surfaceArea = 0;
    for (let z = 0; z < cloud.length; z++) {
      for (let y = 0; y < cloud[z].length; y++) {
        for (let x = 0; x < cloud[z][y].length; x++) {
          if (cloud[z][y][x] !== true) continue;
          if ((cloud[z][y][x - 1] ?? false) !== true) surfaceArea++;
          if ((cloud[z][y][x + 1] ?? false) !== true) surfaceArea++;
          if ((cloud[z][y - 1]?.[x] ?? false) !== true) surfaceArea++;
          if ((cloud[z][y + 1]?.[x] ?? false) !== true) surfaceArea++;
          if ((cloud[z - 1]?.[y]?.[x] ?? false) !== true) surfaceArea++;
          if ((cloud[z + 1]?.[y]?.[x] ?? false) !== true) surfaceArea++;
        }
      }
    }
    return surfaceArea;
  }, 64,
  (cloud) => {
    const p2s = p => p.join(',');
    const s2p = s => s.split(',').map(n => +n);

    const depth = cloud.length;
    const height = cloud[0].length;
    const width = cloud[0][0].length;

    let exposed = 0;
    const visited = new Map();
    visited.set('-1,-1,-1', 0);
    const pending = ['-1,-1,-1'];
    while (pending.length) {
      const look = pending.pop();
      const [x, y, z] = s2p(look);

      if ((visited.get(look) ?? 0) > 0) continue;

      // Flipping these two lines either gets 28 or 174, but it should be 58. What'd I break?
      visited.set(look, (visited.get(look) ?? 0) + 1);

      // If we found an air block, keep exploring its neighbors (as long as they're in bounds and not already visited)
      const neighbors = [
        [x - 1, y, z],
        [x + 1, y, z],
        [x, y - 1, z],
        [x, y + 1, z],
        [x, y, z - 1],
        [x, y, z + 1],
      ].filter(([nx, ny, nz]) => {
        if (nx < -1 || ny < -1 || nz < -1) return false;
        if (nx > width || ny > height || nz > depth) return false;
        if (visited.has(p2s([nx, ny, nz])) && (cloud[nz]?.[ny]?.[nx] ?? false) === false) return false;
        return true;
      });
      for (const [nx, ny, nz] of neighbors) {
        if ((cloud[nz]?.[ny]?.[nx] ?? false) === true) {
          visited.set(p2s([nx, ny, nz]), (visited.get(p2s([nx, ny, nz])) ?? 0) + 1);
          exposed++;
        }
        else pending.push(p2s([nx, ny, nz]));
      }
    }
    return exposed;
  }, 58,
  data => toCloud(data.map(l => l.split(',').map(n => +n)))
);