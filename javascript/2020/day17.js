const { getInput, fromRaw } = require('./utils');
const input = getInput(17);
const sample = fromRaw(`.#.
..#
###`);

const which = sample;

// Z contains Y contains X, using Maps since the coordinates can move unpredictably far in all directions
const zAxis = new Map();
const startingYAxis = new Map();
for (const y in which) {
  const xA = new Map(which[y].split('')
    .map((v, i) => [i, v === '#']));
  startingYAxis.set(Number(y), xA);
}
zAxis.set(0, startingYAxis);

function get(x, y, z, space = zAxis) {
  const zItem = space.get(z);
  if (!zItem) return false;
  const yItem = zItem.get(y);
  if (!yItem) return false;
  const xItem = yItem.get(x);
  return !!xItem;
}

function set(x, y, z, active, space = zAxis) {
  const zItem = space.get(z) || new Map();
  const yItem = zItem.get(y) || new Map();
  yItem.set(x, active);
  zItem.set(y, yItem);
  space.set(z, zItem);
}

function neighborCoordinates(x, y, z) {
  const coords = [];
  for (let lx = x - 1; lx <= x + 1; lx++) {
    for (let ly = y - 1; ly <= y + 1; ly++) {
      for (let lz = z - 1; lz <= z + 1; lz++) {
        if (lx === x && ly === y && lz === z) continue;
        coords.push([lx, ly, lz]);
      }
    }
  }
  return coords;
}

function countActiveNeighbors(x, y, z, space = zAxis) {
  let active = 0;
  for (const [nx, ny, nz] of neighborCoordinates(x, y, z)) {
    if (get(nx, ny, nz, space)) active++;
  }
  return active;
}

// Since coords don't get added in order, we can't just look at the first and last entries.
function coordRange(space = zAxis) {
  let [minX, maxX, minY, maxY, minZ, maxZ] = Array(6)
    .fill(0);
  // First find the existing boundaries
  for (const [z, zSlice] of space) {
    for (const [y, ySlice] of zSlice) {
      for (const [x, xSlice] of ySlice) {
        minX = Math.min(x, minX);
        maxX = Math.max(x, maxX);
        minY = Math.min(y, minY);
        maxY = Math.max(y, maxY);
        minZ = Math.min(z, minZ);
        maxZ = Math.max(z, maxZ);
      }
    }
  }
  return { minX, maxX, minY, maxY, minZ, maxZ };
}

function printState(space) {
  const { minX, maxX, minY, maxY, minZ, maxZ } = coordRange(space);
  for (let z = minZ; z <= maxZ; z++) {
    console.log(`z=${z}`);
    for (let y = minY; y <= maxY; y++) {
      let r = '';
      for (let x = minX; x <= maxX; x++) {
        r += get(x, y, z, space) ? '#' : '.';
      }
      console.log(r);
    }
    console.log();
  }
}

function stepGame(space = zAxis) {
  const next = new Map();
  const { minX, maxX, minY, maxY, minZ, maxZ } = coordRange(space);
  for (let x = minX - 1; x <= maxX + 1; x++) {
    for (let y = minY - 1; y <= maxY + 1; y++) {
      for (let z = minZ - 1; z <= maxZ + 1; z++) {
        const active = get(x, y, z, space);
        const activeNeighbors = countActiveNeighbors(x, y, z, space);
        if (active && !(activeNeighbors === 2 || activeNeighbors === 3)) {
          set(x, y, z, false, next);
        } else if (!active && activeNeighbors === 3) {
          set(x, y, z, true, next);
        }
      }
    }
  }
  return next;
}

function sumActiveCells(space) {
  return [...space.values()].reduce((accZ, curZ) =>
    accZ + [...curZ.values()].reduce((accY, curY) =>
      accY + [...curY.values()].reduce((accX, curX) =>
        accX + (curX === true), 0), 0), 0);
}

const day17star1 = (() =>  {
  let state = zAxis;
  console.log('Initial state:\n\n');
  printState(state);
  for (let t = 0; t < 6; t++) {
    state = stepGame(state);
    console.log(`\n\nAfter ${t + 1} cycles:\n\n`);
    printState(state);
  }
  return sumActiveCells(state);
})();

const day17star2 = (() => {
  //
})();

console.log('Star 1: ', day17star1);
console.log('Star 2: ', day17star2)