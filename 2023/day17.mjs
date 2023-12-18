import aoc from './aoc.mjs';
import { bold, cyan, green, red, yellow, grey, underline } from './utils/color.mjs';

const part1expected = 30; // 102; 30 if goal is [8,0] but 102 if we got to bottom right corner
const part2expected = 'NYI';

const parse = lines => lines.map(line => line.split('').map(n => +n));

const colorCoord = (x, y, pad=2) => `${grey('(')}${yellow(String(x).padStart(pad, ' '))}${grey(',')}${yellow(String(y).padStart(pad, ' '))}${grey(')')}`;

function addCostsExclusive(x1, y1, x2, y2, dist) {
  const w = dist[0].length;
  const h = dist.length;

  if (y2 - y1 !== 0 && x2 - x1 !== 0) {
    throw new Error('addCostsInclusive has to travel both horizontally and vertically, should not do that');
  }
  let out = 0;
  let xMin = Math.min(x1, x2);
  let xMax = Math.max(x1, x2);
  let yMin = Math.min(y1, y2);
  let yMax = Math.max(y1, y2);
  for (let y = yMin; y <= yMax; y++) {
    for (let x = xMin; x <= xMax; x++) {
      if (x < 0 || x >= w || y < 0 || y >= h) throw new Error('addCostsInclusive went out of bounds');
      out += (x === x1 && y === y1) ? 0 : dist[y][x].baseCost;
    }
  }
  return out;
}

function getNeighbors({ x: x1, y: y1 }, dist) {
  const w = dist[0].length;
  const h = dist.length;

  return [
    [x1 - 1, y1],
    [x1 - 2, y1],
    [x1 - 3, y1],
    [x1 + 1, y1],
    [x1 + 2, y1],
    [x1 + 3, y1],
    [x1, y1 - 1],
    [x1, y1 - 2],
    [x1, y1 - 3],
    [x1, y1 + 1],
    [x1, y1 + 2],
    [x1, y1 + 3],
  ].filter(([x2, y2]) => {
    if (x2 < 0 || y2 < 0 || x2 >= w || y2 >= h) return false;

    const diffX = x2 - x1;
    const diffY = y2 - y1;
    const { dx: prevDiffX, dy: prevDiffY } = dist[y1][x1];

    // No going backwards
    if (diffX !== 0 && prevDiffX !== 0 && Math.sign(diffX) !== Math.sign(prevDiffX)) return false;
    if (diffY !== 0 && prevDiffY !== 0 && Math.sign(diffY) !== Math.sign(prevDiffY)) return false;

    // No going more than three steps in a straight line
    if (Math.abs(diffX + prevDiffX) > 3) return false;
    if (Math.abs(diffY + prevDiffY) > 3) return false;

    return true;
  });
}

function buildPath(dist, prev, [startX, startY], [goalX, goalY]) {
  const path = [];
  let [currentX, currentY] = [goalX, goalY];
  while(dist[currentY]?.[currentX] != null) {
    path.unshift([currentX, currentY]);
    [currentX, currentY] = prev.get(currentX * 1_000 + currentY) ?? [-1, -1];
  }
  if (path[0][0] !== startX || path[0][1] !== startY) throw new Error('Path did not lead back to start');
  // if (path[path.length - 1][0] !== goalX || path[path.length - 1][1] !== goalY) throw new Error('Path did not reach end');
  return path;
}

/**
 * Debugging util.
 * @param {{heading: string, rows: (string|number)[][]}[]} columns
 */
function table(columns) {
  const getWidth = (s) => s.replace(/\u001b\[[\d;]+m/g, '').length;
  const widths = columns.map(({ heading, rows }) => Math.max(getWidth(heading), ...rows.map(r => getWidth(r.join(' ')))));
  const maxH = Math.max(...columns.map(({ rows }) => rows.length));
  console.log('┏' + widths.map(w => '━'.repeat(w + 2)).join('┳') + '┓');
  console.log(
    '┃ '
    + columns
      .map(({ heading }, i) => String(heading) + ' '.repeat(widths[i] - getWidth(heading)))
      .join(' ┃ ')
    + ' ┃');
  console.log('┡' + widths.map(w => '━'.repeat(w + 2)).join('╇') + '┩');
  for (let y = 0; y < maxH; y++) {
    console.log(
      '│ '
      + columns
        .map(({ rows }, i) => rows[y] ? rows[y].join(' ') : ' '.repeat(widths[i]))
        .join(' │ ')
      + ' │');
  }
  console.log('└' + widths.map(w => '─'.repeat(w + 2)).join('┴') + '┘');
}

function dumpState(dist, path, [startX, startY], [goalX, goalY]) {
  const dumpGrid = dist.map(row => row.map(({ baseCost }) => baseCost));
  let expectedCost = 0;
  for (let p = 0; p < path.length; p++) {
    const [x, y] = path[p];
    const { dx, dy } = dist[y][x];
    expectedCost += addCostsExclusive(path[p - 1]?.[0] ?? 0, path[p - 1]?.[1] ?? 0, x, y, dist);
    if (dx === 0 && dy === 0) dumpGrid[y][x] = dist[y][x].cost !== Infinity ? green('•') : grey('•');
    else if (dx !== 0 && dy !== 0) dumpGrid[y][x] = red('!');
    else if (dx < 0) dumpGrid[y][x] = ['', green('←'), cyan('←'), yellow('←'), red('←')][Math.abs(Math.max(dx, -4))];
    else if (dx > 0) dumpGrid[y][x] = ['', green('→'), cyan('→'), yellow('→'), red('→')][Math.min(dx, 4)];
    else if (dy < 0) dumpGrid[y][x] = ['', green('↑'), cyan('↑'), yellow('↑'), red('↑')][Math.abs(Math.max(dy, -4))];
    else if (dy > 0) dumpGrid[y][x] = ['', green('↓'), cyan('↓'), yellow('↓'), red('↓')][Math.min(dy, 4)];
    if ((x === startX && y === startY) || (x === goalX && y === goalY)) dumpGrid[y][x] = underline(dumpGrid[y][x]);
  }
  table([
    { heading: 'ORIGINAL COSTS', rows: dist.map(row => row.map(({ baseCost }) => baseCost)) },
    { heading: `PATH (cost ${yellow(expectedCost)})`, rows: dumpGrid },
    { heading: 'LINES', rows: dist.map((row) => row.map(({ x, y, dx, dy }) => {
      if (dx === 0 && dy === 0) return dist[y][x].cost !== Infinity ? green('•') : grey('•');
      else if (dx !== 0 && dy !== 0) return red('!');
      else if (dx < 0) return ['', green('←'), cyan('←'), yellow('←'), red('←')][Math.abs(Math.max(dx, -4))];
      else if (dx > 0) return ['', green('→'), cyan('→'), yellow('→'), red('→')][Math.min(dx, 4)];
      else if (dy < 0) return ['', green('↑'), cyan('↑'), yellow('↑'), red('↑')][Math.abs(Math.max(dy, -4))];
      else if (dy > 0) return ['', green('↓'), cyan('↓'), yellow('↓'), red('↓')][Math.min(dy, 4)];
      else return red('?');
    })) },
    { heading: 'COSTS', rows: dist.map((row) => row.map(({ cost }) => String(cost === Infinity ? '∞' : cost).padStart(3))) },
  ]);
}

/**
 * Dijkstra's pathfinding algorithm. Obviously.
 * @param {number[][]} costs 
 * @param {[x: number, y: number]} startPoint 
 * @param {[x: number, y: number]} goalPoint
 * @returns 
 */
function dijkstra(costs, [startX, startY], [goalX, goalY]) {
  const w = costs[0].length;
  const h = costs.length;

  let numVisited = 0;
  const visits = Array(w * h).fill(false);
  const isVisited = (x, y) => visits[y * w + x];

  const dist = costs.map((row, y) => row.map((baseCost, x) => ({ x, y, baseCost, cost: Infinity, dx: 0, dy: 0 })));
  const prev = new Map();
  dist[startY][startX].cost = 0;

  while (numVisited < w * h) {
    // if (numVisited % 10_000 === 0) console.log(`...${(w * h) - numVisited} more nodes to visit...`);

    // Doing map/reduce with this would be concise, but incredibly slow for large inputs.
    let node = { x: -1, y: -1, cost: Infinity, dx: Infinity, dy: Infinity, pos: -1 };
    for (let u = 0; u < w * h; u++) {
      if (visits[u]) continue;
      const ux = u % w;
      const uy = Math.floor(u / w);
      const { cost, dx, dy } = dist[uy][ux];
      if (cost < node.cost) {
        node.x = ux;
        node.y = uy;
        node.cost = cost;
        node.dx = dx;
        node.dy = dy;
        node.pos = u;
      }
    }
    numVisited++;
    visits[node.pos] = true;

    if (node.pos === -1) {
      console.error(red('Failed to find which node to visit next'));
      process.exit(1);
    }
    console.log(`${bold('Visit')} ${colorCoord(node.x, node.y)}, reached via ${colorCoord(node.x - node.dx, node.y - node.dy)}`);

    for (const [neighborX, neighborY] of getNeighbors(node, dist)) {
      // const doLog = [[10, 5], [10, 6]].some(([lx, ly]) => (next.x === lx && next.y === ly) || (neighborX === lx && neighborY === ly));
      const doLog = true;
      if (isVisited(neighborX, neighborY)) {
        if (doLog) {
          console.log(`  ${bold(grey('SKIP'))} ${colorCoord(neighborX, neighborY)}, already visited`);
        }
        continue;
      }
      if (doLog) console.log(`  ${bold('CHECK')} → ${colorCoord(neighborX, neighborY)}`);

      // Since the neighbors jump, sum the costs along that jump
      let cost = dist[node.y][node.x].cost + addCostsExclusive(node.x, node.y, neighborX, neighborY, dist);
      const diffX = neighborX - node.x;
      const diffY = neighborY - node.y;
      const { dx: prevDiffX, dy: prevDiffY } = dist[node.y][node.x];
      const totalDiffX = (diffX !== 0 && (Math.sign(diffX) === Math.sign(prevDiffX) || prevDiffX === 0))
        ? prevDiffX + diffX
        : 0;
      const totalDiffY = (diffY !== 0 && (Math.sign(diffY) === Math.sign(prevDiffY) || prevDiffY === 0))
        ? prevDiffY + diffY
        : 0;

      if (Math.abs(totalDiffX) > 3 || Math.abs(totalDiffY) > 3) {
        // if (doLog) console.log(`    ${red('Would be illegal')}: ${yellow(Math.max(Math.abs(totalDiffX), Math.abs(totalDiffY)))} in a row`);
        throw new Error('If this happened, then the neighbor calculation missed something');
        cost = Infinity;
      }

      const neighborNode = dist[neighborY][neighborX];
      if (cost < neighborNode.cost) {
        if (doLog) {
          // console.log(
          //   `    ${green('Improvement')}: ${yellow(cost)} < ${yellow(neighborNode.cost)}.`
          //   + ` Line: ${colorCoord(totalDiffX, totalDiffY)}`
          // );
        }
        neighborNode.cost = cost;
        neighborNode.dx = totalDiffX;
        neighborNode.dy = totalDiffY;
        prev.set(neighborX * 1_000 + neighborY, [node.x, node.y]);
      } else if (cost < Infinity) {
        // if (doLog) console.log(`    ${red('Not an improvement')}: ${yellow(cost)} ≥ ${yellow(dist[neighborY][neighborX].cost)}.`);
      }
    }

    const path = buildPath(dist, prev, [startX, startY], [node.x, node.y]);
    dumpState(dist, path, [startX, startY], [node.x, node.y]);


    // Every node we visit already has the shortest path noted, so stop visiting once we find the goal.
    if (node.x === goalX && node.y === goalY) break;
  }

  // We don't need the path to answer the challenge; just the final cost.
  // However, verifying that the path exists tests that we did it right.
  const path = buildPath(dist, prev, [startX, startY], [goalX, goalY]);

  dumpState(dist, path, [startX, startY], [goalX, goalY]);

  return dist[goalY][goalX].cost;
}

function aStar(costs, [startX, startY], [goalX, goalY]) {
  const w = costs[0].length;
  const h = costs.length;

  const openSet = [[startX, startY]];
  const cameFrom = new Map(); // (x * 1_000 + y) -> { x, y, dx, dy }. dx/dy can be recalculated by walking the path backwards until it turns.
  
  const gScores = new Map(); // Default to Infinity!
  gScores.set(startX * 1_000 + startY, 0);

  const fScores = new Map(); // Default to Infinity!
  fScores.set(startX * 1_000 + startY, heuristic(startX, startY));

  // Manhattan distance to goal. Not great, but will never exceed actual score because there are no costs < 1.
  function heuristic(hx, hy) {
    return Math.abs(goalX - hx) + Math.abs(goalY - hy);
  }

  // Follows Da Rules of straight lines
  function getNeighbors(x1, y1) {
    if (x1 < 0 || y1 < 0 || x1 >= w || y1 >= h) return [];
    return [
      [x1 - 1, y1],
      // [x1 - 2, y1],
      // [x1 - 3, y1],
      [x1 + 1, y1],
      // [x1 + 2, y1],
      // [x1 + 3, y1],
      [x1, y1 - 1],
      // [x1, y1 - 2],
      // [x1, y1 - 3],
      [x1, y1 + 1],
      // [x1, y1 + 2],
      // [x1, y1 + 3],
    ].filter(([x2, y2]) => {
      if (x2 < 0 || y2 < 0 || x2 >= w || y2 >= h) return false;
  
      const diffX = x2 - x1;
      const diffY = y2 - y1;
      const { dx: prevDiffX, dy: prevDiffY } = cameFrom.get(x1 * 1_000 + y1) ?? { dx: 0, dy: 0};
  
      // No going backwards
      if (diffX !== 0 && prevDiffX !== 0 && Math.sign(diffX) !== Math.sign(prevDiffX)) return false;
      if (diffY !== 0 && prevDiffY !== 0 && Math.sign(diffY) !== Math.sign(prevDiffY)) return false;
  
      // No going more than three steps in a straight line
      if (Math.abs(diffX + prevDiffX) > 3) return false;
      if (Math.abs(diffY + prevDiffY) > 3) return false;
  
      return true;
    });
  }

  function stepScore(x1, y1, x2, y2) {
    if (y2 - y1 !== 0 && x2 - x1 !== 0) {
      throw new Error(`stepScore trying to go both directions, don't. From ${colorCoord(x1, y1)} to ${colorCoord(x2, y2)}`);
    }
    let out = 0;
    let xMin = Math.min(x1, x2);
    let xMax = Math.max(x1, x2);
    let yMin = Math.min(y1, y2);
    let yMax = Math.max(y1, y2);
    for (let y = yMin; y <= yMax; y++) {
      for (let x = xMin; x <= xMax; x++) {
        if (x < 0 || x >= w || y < 0 || y >= h) throw new Error('stepScore went out of bounds');
        out += (x === x1 && y === y1) ? 0 : costs[y][x];
      }
    }
    return out;
  }

  function reconstructPathFrom(x1, y1) {
    const path = [[x1, y1]];
    let current = [x1, y1];
    while (cameFrom.has(current[0] * 1_000 + current[1])) {
      const { x, y } = cameFrom.get(current[0] * 1_000 + current[1]);
      path.unshift([x, y]);
      current = [x, y];
    }
    return path;
  }

  // Figures out how far we've traveled in a straight line since our last turn
  // Doing this the slow-but-safe way for now just to avoid a new bug class until I make sure everything is okay
  function computeStraightLine(x1, y1, x2, y2) {
    console.log(`computeStraightLine: ${colorCoord(x1, y1)} → ${colorCoord(x2, y2)}`);
    const toLatestX = x2 - x1;
    const toLatestY = y2 - y1;
    let dxOut = toLatestX;
    let dyOut = toLatestY;
    let check = [x1, y1];
    while (cameFrom.has(check[0] * 1_000 + check[1])) {
      const { x: x3, y: y3, dx, dy } = cameFrom.get(check[0] * 1_000 + check[1]);
      const nextDx = check[0] - x3;
      const nextDy = check[1] - y3;
      if ((dx != 0 && nextDy != 0) || (dy != 0 && nextDx != 0)) return {dx, dy};

      console.log(' ', JSON.stringify({ dxOut, dyOut, check, check, x3, y3, dx, dy, nextDx, nextDy }));
      if (Math.abs(dxOut + nextDx) !== 0 && Math.abs(dyOut + nextDy) !== 0) {
        // We turned.
        break;
      } else if (dxOut !== 0 && Math.sign(dxOut) !== Math.sign(nextDx)) {
        // We either turned or we tried to flip around
        break;
      } else if (dyOut !== 0 && Math.sign(dyOut) !== Math.sign(nextDy)) {
        // Same as above
        break;
      } else {
        // Alright, we're still moving in this direction
        dxOut += nextDx;
        dyOut += nextDy;
      }
      check = [x3, y3];
      break;
    }
    console.log(' ', { dxOut, dyOut });
    return { dx: dxOut, dy: dyOut };
  }

  function dumpAStarState(path, [startX, startY], [goalX, goalY]) {
    const dumpGrid = costs.map(row => [...row]);
    let expectedCost = 0;
    for (let p = 0; p < path.length; p++) {
      const [x, y] = path[p];
      const { dx, dy } = cameFrom.get(x * 1_000 + y) ?? { dx: 0, dy: 0 };
      expectedCost += stepScore(path[p - 1]?.[0] ?? 0, path[p - 1]?.[1] ?? 0, x, y);
      if (dx === 0 && dy === 0) dumpGrid[y][x] = costs[y][x] !== Infinity ? green('•') : grey('•');
      else if (dx !== 0 && dy !== 0) dumpGrid[y][x] = red('!');
      else if (dx < 0) dumpGrid[y][x] = ['', green('←'), cyan('←'), yellow('←'), red('←')][Math.abs(Math.max(dx, -4))];
      else if (dx > 0) dumpGrid[y][x] = ['', green('→'), cyan('→'), yellow('→'), red('→')][Math.min(dx, 4)];
      else if (dy < 0) dumpGrid[y][x] = ['', green('↑'), cyan('↑'), yellow('↑'), red('↑')][Math.abs(Math.max(dy, -4))];
      else if (dy > 0) dumpGrid[y][x] = ['', green('↓'), cyan('↓'), yellow('↓'), red('↓')][Math.min(dy, 4)];
      if ((x === startX && y === startY) || (x === goalX && y === goalY)) dumpGrid[y][x] = underline(dumpGrid[y][x]);
    }
    table([
      { heading: 'ORIGINAL COSTS', rows: costs },
      { heading: `PATH (cost ${yellow(expectedCost)})`, rows: dumpGrid },
      { heading: 'LINES', rows: costs.map((row, r) => row.map((baseCost, c) => {
        const { x, y, dx, dy } = cameFrom.get(c * 1_000 + r) ?? { dx: 0, dy: 0 };
        if (dx === 0 && dy === 0) return grey('•');
        else if (dx !== 0 && dy !== 0) return red('!');
        else if (dx < 0) return ['', green('←'), cyan('←'), yellow('←'), red('←')][Math.abs(Math.max(dx, -4))];
        else if (dx > 0) return ['', green('→'), cyan('→'), yellow('→'), red('→')][Math.min(dx, 4)];
        else if (dy < 0) return ['', green('↑'), cyan('↑'), yellow('↑'), red('↑')][Math.abs(Math.max(dy, -4))];
        else if (dy > 0) return ['', green('↓'), cyan('↓'), yellow('↓'), red('↓')][Math.min(dy, 4)];
        else return red('?');
      })) },
      // { heading: 'COSTS', rows: dist.map((row) => row.map(({ cost }) => String(cost === Infinity ? '∞' : cost).padStart(3))) },
    ]);
  }

  while (openSet.length) {
    openSet.sort(([x1, y1], [x2, y2]) => (fScores.get(x2 * 1_000 + y2) ?? Infinity) - (fScores.get(x1 * 1_000 + y1) ?? Infinity));
    console.log(openSet.length, openSet.slice(-10).map(p => colorCoord(...p)).join('  '));
    const current = openSet.pop();

    console.log(`${bold('VISIT')} ${colorCoord(current[0], current[1])}`);
    if (current[0] === goalX && current[1] === goalY) {
      console.info('Reached goal, time to reconstruct the path now');
      const reconstructed = reconstructPathFrom(...current);
      let score = 0;
      for (let p = 0; p < reconstructed.length - 1; p++) {
        const then = stepScore(...reconstructed[p], ...reconstructed[p + 1]);
        console.log(`  → ${colorCoord(...reconstructed[p + 1])} incurs ${yellow(then)}`);
        score += then;
      }
      dumpAStarState(reconstructed, [startX, startY], current);
      console.log(score);
      console.log(cameFrom);
      return score;
    }

    for (const [nx, ny] of getNeighbors(current[0], current[1])) {
      const nk = nx * 1_000 + ny;
      const tentativeGScore = (gScores.get(current[0] * 1_000 + current[1]) ?? Infinity) + stepScore(current[0], current[1], nx, ny);
      // console.log(`  ${bold('CHECK')} ${colorCoord(nx, ny)}`, { nk, tentativeGScore });
      if (tentativeGScore < (gScores.get(nk) ?? Infinity)) {
        const { dx, dy } = computeStraightLine(...current, nx, ny);
        cameFrom.set(nk, { x: current[0], y: current[1], dx, dy });
        gScores.set(nk, tentativeGScore);
        fScores.set(nk, tentativeGScore + heuristic(nx, ny));
        if (!openSet.some(([ox, oy]) => ox === nx && oy === ny)) {
          openSet.push([nx, ny]);
        }
      }
    }
  }

  return red('FAILED TO FIND A PATH');
}

const part1 = (grid) => {
  const [startX, startY] = [0, 0];
  // const [goalX, goalY] = [grid[0].length - 1, grid.length - 1];
  const [goalX, goalY] = [8, 0];
  // return dijkstra(grid, [startX, startY], [goalX, goalY]);
  return aStar(grid, [startX, startY], [goalX, goalY]);
};

const part2 = (grid) => {
  return 'NYI';
};

aoc(2023, 17, part1, part1expected, part2, part2expected, parse);
