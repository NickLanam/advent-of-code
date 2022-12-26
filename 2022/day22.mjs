function parse(unparsed) {
  let rawLines = unparsed.map(l => l.replaceAll(/[\r\n]/g, ''));
  while (rawLines[0] === '') rawLines.shift();
  while (rawLines[rawLines.length - 1] === '') rawLines.pop();

  let instRaw = rawLines.slice(-1)[0].split('');
  let inst = [];
  while (instRaw.length) {
    const i = instRaw.findIndex(v => Number.isNaN(+v));
    if (i === 0) inst.push(instRaw.shift());
    else if (i > 0) inst.push(+(instRaw.splice(0, i).join('')));
    else  { inst.push(+instRaw.join('')); instRaw = []; }
  }

  const mapLines = rawLines.slice(0, -2);
  const h = mapLines.length;
  const w = Math.max(...mapLines.map(l => l.length));

  const tiles = Array(h).fill(null)
    .map((_, y) => Array(w).fill(null)
      .map((_, x) => mapLines[y][x] ?? ' '));

  // Only two net shapes to care about: the one in the sample, and the one in my input.
  // Coding for all 11 possible cube nets would be more complex than I care to write.
  // For these two nets, we can tell them apart AND get the face width from the last line.
  const isSampleNet = tiles[h - 1][0] === ' ';
  const fw = tiles[h - 1].filter(v => v !== ' ').length / (isSampleNet ? 2 : 1);

  // Mapping every face/dir pair to the corresponding face/dir when walking off an edge.
  // This includes faces that are directly adjacent in the 2D map; simpler that way.
  const portals = (isSampleNet
    ? new Map([
      // Shape:
      //         [ ]
      // [ ] [ ] [ ]
      //         [ ] [ ]
      ['0,0', '5,2'], ['0,1', '3,1'], ['0,2', '2,1'], ['0,3', '1,1'],
      ['1,0', '2,0'], ['1,1', '4,3'], ['1,2', '5,3'], ['1,3', '0,1'],
      ['2,0', '3,0'], ['2,1', '4,0'], ['2,2', '1,2'], ['2,3', '0,0'],
      ['3,0', '5,1'], ['3,1', '4,1'], ['3,2', '2,2'], ['3,3', '0,3'],
      ['4,0', '5,0'], ['4,1', '1,3'], ['4,2', '2,3'], ['4,3', '3,3'],
      ['5,0', '0,2'], ['5,1', '1,0'], ['5,2', '4,2'], ['5,3', '3,2'],
    ])
    : new Map([
      // Shape:
      //     [ ] [ ]
      //     [ ]
      // [ ] [ ]
      // [ ]
      ['0,0', '1,0'], ['0,1', '2,1'], ['0,2', '3,0'], ['0,3', '5,0'],
      ['1,0', '4,2'], ['1,1', '2,2'], ['1,2', '0,2'], ['1,3', '5,3'],
      ['2,0', '1,3'], ['2,1', '4,1'], ['2,2', '3,1'], ['2,3', '0,3'],
      ['3,0', '4,0'], ['3,1', '5,1'], ['3,2', '0,0'], ['3,3', '2,0'],
      ['4,0', '1,2'], ['4,1', '5,2'], ['4,2', '3,2'], ['4,3', '2,3'],
      ['5,0', '4,3'], ['5,1', '1,1'], ['5,2', '0,1'], ['5,3', '3,3'],
    ])
  );

  const getFace = (x, y) => {
    const fx = Math.floor((((x % w) + w) % w) / fw);
    const fy = Math.floor((((y % h) + h) % h) / fw);
    if (isSampleNet) {
      if (fx === 2 && fy === 0) return 0;
      if (fx === 0 && fy === 1) return 1;
      if (fx === 1 && fy === 1) return 2;
      if (fx === 2 && fy === 1) return 3;
      if (fx === 2 && fy === 2) return 4;
      if (fx === 3 && fy === 2) return 5;
      return NaN;
    } else {
      if (fx === 1 && fy === 0) return 0;
      if (fx === 2 && fy === 0) return 1;
      if (fx === 1 && fy === 1) return 2;
      if (fx === 0 && fy === 2) return 3;
      if (fx === 1 && fy === 2) return 4;
      if (fx === 0 && fy === 3) return 5;
      return NaN;
    }
  };

  const getPortalExit = (x, y, d) => {
    // Figure out where we are and if there's supposed to be a portal here...
    const faceId = getFace(x, y);
    let onCorrectEdge = false;
    switch (d) {
      case 0: onCorrectEdge = (x % fw) === (fw - 1); break;
      case 1: onCorrectEdge = (y % fw) === (fw - 1); break;
      case 2: onCorrectEdge = (x % fw) === 0; break;
      case 3: onCorrectEdge = (y % fw) === 0; break;
    }

    if (!onCorrectEdge) throw new Error(`getPortalExit(${x}, ${y}, ${d}): That is not on a matching portal edge!`);

    const portal = portals.get(`${faceId},${d}`);
    if (!portal) throw new Error(`getPortalExit(${x}, ${y}, ${d}): Portal is missing for that edge!`);

    const [pf, pd] = portal.split(',').map(n => +n); // [targetFace, targetDir] but with shorter names

    // Find relative coordinates within this face, then rotate to the corresponding relative coords in the other face.
    const relX = x % fw;
    const relY = y % fw;
    let prX = NaN, prY = NaN; // x,y relative to the target face
    if (pd === 0) { prX = 0;      prY = [relY,          fw - 1 - relX, fw - 1 - relY, relX         ][d]; }
    if (pd === 1) { prY = 0;      prX = [fw - 1 - relY, relX,          relY,          fw - 1 - relX][d]; }
    if (pd === 2) { prX = fw - 1; prY = [fw - 1 - relY, relX,          relY,          fw - 1 - relX][d]; }
    if (pd === 3) { prY = fw - 1; prX = [relY,          fw - 1 - relX, fw - 1 - relY, relX         ][d]; }

    // Find the top-left corner of the face we're going to, so we get the real exit coordinates of the portal.
    let fx = 0;
    let fy = 0;
    let fc = -1;
    faceCoordSearch: for (fy = 0; fy < h; fy += fw) {
      for (fx = 0; fx < w; fx += fw) {
        if ((tiles[fy]?.[fx] ?? ' ') !== ' ') fc++;
        if (fc === pf) break faceCoordSearch;
      }
    }

    return { x: prX + fx, y: prY + fy, d: pd };
  };

  const nextCoord = ({ x, y, d, wrapMode }) => {
    if (tiles[y][x] === '#') throw new Error(`nextCoord: Starting position ${x},${y} is a wall`);
    if (tiles[y]?.[x] == null) throw new Error(`nextCoord: Starting position ${x},${y} is out of bounds`);
    const w = tiles[0].length;
    const h = tiles.length;

    let nx = x;
    let ny = y;
    let nd = d;

    let dx = [1, 0, -1, 0][nd];
    let dy = [0, 1, 0, -1][nd];
    if (wrapMode === 'flat') {
      // Part 1: walking off the edge means wrapping around in 2D to the next non-empty tile.
      // Direction doesn't change in this case.
      do {
        nx = (nx + dx + w) % w;
        ny = (ny + dy + h) % h;
      }
      while (tiles[ny][nx] === ' ');
    } else if (wrapMode === 'cube') {
      // Part 2: pretend the 2D map folds into a 3D cube; walking off one face walks onto another.
      // Direction DOES change in this case, since final coordinates are still on the 2D map.

      // If we're not leaving the current face, simply walk along it. Nothing special to do.
      if (getFace(x + dx, y + dy) === getFace(x, y)) {
        nx = x + dx;
        ny = y + dy;
      } else {
        // If we're here, it means we need to warp through a portal. Find the right one, and look through it.
        // If it's a space or the portal doesn't exist, the portal network has a mistake. Throw an error.
        // If it's a wall, the portal is blocked and we must stay where we are.
        // If it's a dot, we can go through the portal.
        const { x: px, y: py, d: pd } = getPortalExit(nx, ny, nd);
        if (!(tiles[py]?.[px] ?? ' ').trim()) {
          console.error(`Tried to warp from ${nx},${ny}@${nd} to ${px},${py}@${pd}, but that didn't exist`);
          throw new Error('Portal network is wrong');
        }
        if ((px !== nx || py !== ny) && tiles[py]?.[px] === '.') {
          nx = px;
          ny = py;
          nd = pd;
        }
      }
    } else throw new Error('Unknown wrapMode: ' + wrapMode);


    if (tiles[ny]?.[nx] == null) throw new Error(`nextCoord: Travel from ${x},${y}@${d} to ${nx},${ny}@${nd} landed out of bounds`);
    if (tiles[ny][nx] === '#') return { x, y, d };
    else return { x: nx, y: ny, d: nd };
  };

  return { tiles, inst, nextCoord };
}

function step({ x, y, d, nextCoord }, instruction, wrapMode) {
  if (instruction === 'L') d = (d + 3) % 4;
  else if (instruction === 'R') d = (d + 5) % 4;
  else {
    for (let i = 0; i < instruction; i++) {
      ({ x, y, d } = nextCoord({ x, y, d, wrapMode }));
    }
  }

  return { x, y, d };
}

function solve({ tiles, inst, nextCoord }, wrapMode) {
  let x = tiles[0].findIndex(v => v !== ' ');
  let y = 0;
  let d = 0; // 0 = right, 1 = down, 2 = left, 3 = up
  for (const instruction of inst) {
    ({ x, y, d } = step({ x, y, d, nextCoord }, instruction, wrapMode));
  }
  return (1000 * (y + 1)) + (4 * (x + 1)) + d;
}

(await import('./aoc.mjs')).default(
  2022, 22,
  (data) => solve(data, 'flat'), 6032,
  (data) => solve(data, 'cube'), 5031,
  parse, false
);