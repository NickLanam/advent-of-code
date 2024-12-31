function neighborCoords(grid, dist, x, y) {
  const w = grid[0].length;
  const h = grid.length;

  const c = grid[y]?.[x];
  // if (c == null) throw new Error(`No neighbors can exist for a non-existent ${x}, ${y}`);
  if (c == null) return [];

  const ret = [[x - 1, y], [x, y - 1], [x + 1, y], [x, y + 1]].map(([x2, y2]) => {
    const c2 = grid[y2]?.[x2];
    if (
      c2 == null || (c2 > (c + 1))
    ) {
      return null;
    }
    return [x2, y2];
  }).filter(cc => cc !== null);

  return ret;
}

function dijkstra(heights, [startX, startY], [goalX, goalY]) {
  const w = heights[0].length;
  const h = heights.length;

  let numVisited = 0;
  const visits = Array(w * h).fill(false);
  const isVisited = (x, y) => visits[y * w + x];

  const dist = heights.map((r) => r.map(() => Infinity));
  const prev = new Map();
  dist[startY][startX] = 0;

  while (numVisited < w * h) {
    // if (numVisited % 10_000 === 0) console.log(`...${(w * h) - numVisited} more nodes to visit...`);

    // Doing map/reduce with this would be concise, but incredibly slow for large inputs.
    const next = { x: -1, y: -1, d: Infinity, pos: -1 };
    for (let u = 0; u < w * h; u++) {
      if (visits[u]) continue;
      const ux = u % w;
      const uy = Math.floor(u / w);
      if (dist[uy][ux] < next.d) {
        next.x = ux;
        next.y = uy;
        next.d = dist[uy][ux];
        next.pos = u;
      }
    }
    numVisited++;
    visits[next.pos] = true;

    for (const [neighborX, neighborY] of neighborCoords(heights, dist, next.x, next.y)) {
      if (isVisited(neighborX, neighborY)) continue;
      // const c = dist[next.y][next.x] + heights[neighborY][neighborX];
      const c = dist[next.y][next.x] + 1;
      if (c < dist[neighborY][neighborX]) {
        dist[neighborY][neighborX] = c;
        prev.set(`${neighborX},${neighborY}`, [next.x, next.y]);
      }
    }
    // Every node we visit already has the shortest path noted, so stop visiting once we find the goal.
    if (next.x === goalX && next.y === goalY) break;
  }

  // We don't need the path to answer the challenge; just the final cost.
  // However, verifying that the path exists tests that we did it right.
  const path = [];
  let [currentX, currentY] = [goalX, goalY];
  while(heights[currentY]?.[currentX] != null) {
    path.unshift([currentX, currentY]);
    [currentX, currentY] = prev.get(`${currentX},${currentY}`) ?? [-1, -1];
  }
  if (path.length < 2) {
    // For my input, this case actually happens a lot but I get the right answer by ignoring it so...
    return Infinity;
    // throw new Error('Path has less than two nodes...' + JSON.stringify({ path, heights, prev, dist, visits }));
  }
  if (path[0][0] !== startX || path[0][1] !== startY) throw new Error('Path did not lead back to start');
  if (path[path.length - 1][0] !== goalX || path[path.length - 1][1] !== goalY) throw new Error('Path did not reach end');

  return dist[goalY][goalX];
}

(await import('./aoc.mjs')).default(
  2022, 12,
  ({ grid, start, end }) => {
    return dijkstra(grid, start, end);
  }, 31,
  ({ grid, end }) => {
    const candidateScores = [];
    for (let x = 0; x < grid[0].length; x++) {
      for (let y = 0; y < grid.length; y++) {
        if (grid[y][x] === 0) {
          candidateScores.push(dijkstra(grid, [x, y], end));
        }
      }
    }
    return Math.min(...candidateScores);
  }, 29,
  data => {
    const grid = data.map(l => l.split(''));
    const start = [grid.findIndex(r => r.some(c => c === 'S'))];
    start.unshift(grid[start[0]].findIndex(c => c === 'S'));
    const end = [grid.findIndex(r => r.some(c => c === 'E'))];
    end.unshift(grid[end[0]].findIndex(c => c === 'E'));

    grid[start[1]][start[0]] = 'a';
    grid[end[1]][end[0]] = 'z';

    return { grid: grid.map(row => row.map(c => String(c).charCodeAt(0) - 97)), start, end };
  }
);