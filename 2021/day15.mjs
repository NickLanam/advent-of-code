function neighborCoords(grid, x, y) {
  const possible = [
    [x, y - 1], [x - 1, y], [x + 1, y], [x, y + 1]
  ];
  return possible.filter(([x2, y2]) => grid[y2]?.[x2] != null);
}

function djikstra(costs, [startX, startY], [goalX, goalY]) {
  const w = costs[0].length;
  const h = costs.length;

  let numVisited = 0;
  const visits = Array(w * h).fill(false);
  const isVisited = (x, y) => visits[y * w + x];

  const dist = Array(costs.length).fill(0).map(_ => Array(costs[0].length).fill(Infinity));
  const prev = new Map();
  dist[startY][startX] = 0;

  while (numVisited < w * h) {
    if (numVisited % 10_000 === 0) console.log(`...${(w * h) - numVisited} more nodes to visit...`);

    // Doing map/reduce with this would be concise, but incredibly slow for large inputs.
    const next = { x: -1, y: -1, d: Infinity, pos: -1 };
    for (let u = 0; u < w * h; u++) {
      if (visits[u]) continue;
      const ux = u % w;
      const uy = Math.floor(u / h);
      if (dist[uy][ux] < next.d) {
        next.x = ux;
        next.y = uy;
        next.d = dist[uy][ux];
        next.pos = u;
      }
    }
    numVisited++;
    visits[next.pos] = true;

    for (const [neighborX, neighborY] of neighborCoords(dist, next.x, next.y)) {
      if (isVisited(neighborX, neighborY)) continue;
      const c = dist[next.y][next.x] + costs[neighborY][neighborX];
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
  while(costs[currentY]?.[currentX] != null) {
    path.unshift([currentX, currentY]);
    [currentX, currentY] = prev.get(`${currentX},${currentY}`) ?? [-1, -1];
  }
  if (path.length < 2) throw new Error('Path has less than two nodes...' + JSON.stringify(path));
  if (path[0][0] !== startX || path[0][1] !== startY) throw new Error('Path did not lead back to start');
  if (path[path.length - 1][0] !== goalX || path[path.length - 1][1] !== goalY) throw new Error('Path did not reach end');

  return dist[goalY][goalX];
}

(await import('./aoc.mjs')).default(
  2021, 15,
  // This is literally a pathfinding problem. Djikstra's algorithm is the obvious thing to try.
  (grid) => djikstra(grid, [0, 0], [grid[0].length - 1, grid.length - 1]), 40,
  // Note: with real input, this has 250,000 nodes. Djikstra's is not linear to the input. This takes a while.
  // (See https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Running_time - it's complicated)
  // On the plus side, O(n) memory usage.
  (small) => {
    const big = Array(small.length * 5).fill(0).map((_, r) => Array(small[0].length * 5).fill(0).map((_, c) => {
      const sx = c % small[0].length, sy = r % small.length;
      const sd = small[sy][sx];
      const addX = Math.floor(c / small[0].length), addY = Math.floor(r / small.length);
      return ((sd + addX + addY - 1) % 9) + 1;
    }));
    return djikstra(big, [0, 0], [big[0].length - 1, big.length - 1]);
  }, 315,
  data => data.map(line => line.split('').map(n => +n))
);