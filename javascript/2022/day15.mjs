const manhattan = (x1, y1, x2, y2) => Math.abs(x2 - x1) + Math.abs(y2 - y1);
const p2s = point => `${point[0]},${point[1]}`;
const s2p = str => JSON.parse(`[${str}]`);

// Finds all the points that are exactly one step outside of the manhattan radius of a sensor/beacon pair
function outerRing(sx, sy, bx, by)  {
  let options = [];
  const m = manhattan(sx, sy, bx, by) + 1;

  for (let x = sx - m; x <= sx + m; x++) {
    let m2 = m - Math.abs(x - sx);
    options.push([x, sy - m2]);
    options.push([x, sy + m2]);
  }
  return [...new Set(options.map(o => p2s(o)))].map(s => s2p(s));
}

(await import('./aoc.mjs')).default(
  2022, 15,
  (data, isSample) => {
    const searchY = isSample ? 10 : 2_000_000;
    let found = 0;

    let minX = Infinity; let maxX = -Infinity;
    for (const { sx, sy, bx, by } of data) {
      const m = manhattan(sx, sy, bx, by);
      minX = Math.min(minX, sx - m, sx + m);
      maxX = Math.max(maxX, sx - m, sx + m);
    }

    xLoop: for (let x = minX; x <= maxX; x++) {
      for (const { sx, sy, bx, by } of data) {
        if (
          manhattan(sx, sy, x, searchY) <= manhattan(sx, sy, bx, by )
          && !(x === bx && searchY === by)
        ) {
          found++;
          continue xLoop;
        }
      }
    }

    return found;
  }, 26,
  (data, isSample) => {
    const [MIN_SEARCH_X, MAX_SEARCH_X, MIN_SEARCH_Y, MAX_SEARCH_Y] = [0, 4_000_000, 0, 4_000_000];
    let minX = Infinity; let minY = Infinity; let maxX = -Infinity; let maxY = -Infinity;
    for (const { sx, sy, bx, by } of data) {
      const m = manhattan(sx, sy, bx, by);
      minX = Math.max(MIN_SEARCH_X, Math.min(minX, sx));
      maxX = Math.min(MAX_SEARCH_X, Math.max(maxX, sx));
      minY = Math.max(MIN_SEARCH_Y, Math.min(minY, sy));
      maxY = Math.min(MAX_SEARCH_Y, Math.max(maxY, sy));
    }

    console.log({ minX, maxX, minY, maxY });

    const rings = data.map(({ sx, sy, bx, by }) => (
      outerRing(sx, sy, bx, by).filter(c => {
        if (c[0] < minX || c[0] > maxX || c[1] < minY || c[1] > maxY) {
          return false;
        }
        for (const sb of data) {
          const mLim = manhattan(sb.sx, sb.sy, sb.bx, sb.by);
          const mReal = manhattan(sb.sx, sb.sy, c[0], c[1]);
          if (mReal <= mLim) return false;
        }
        return true;
      })
    ));

    const uniqueOptions = [...new Set(rings.flat(1).map(p => p2s(p)))].sort().map(s => s2p(s));

    if (isSample && !uniqueOptions.some(p => p[0] === 14 && p[1] === 11)) {
      throw new Error('Unique options should have included point 14,11 but did not');
    }
    if (uniqueOptions.length > 1) {
      console.log('More than one point remained', uniqueOptions);
      throw new Error(`Should have only been able to find one matching point, found ${uniqueOptions.length}`);
    }

    return uniqueOptions[0][0] * MAX_SEARCH_X + uniqueOptions[0][1];
  }, 56000011,
  data => data.map(line => {
    const [, sx, sy, bx, by] = line.replaceAll(/[^\d=-]+/g, '').split('=').map(v => +v);
    return { sx, sy, bx, by };
  }),
  true, false
);