import { range } from './utils/array.mjs';

function printBurrow({ hall, rooms }, depth = 2) {
  // Assumes the hall is at least wide enough to access every room and the surrounding walls.
  const pad = hall.length + 2 - (rooms.length * 2 - 1);
  console.log('╔' + '═'.repeat(hall.length * 2 + 1) + '╗');
  console.log('║ ' + hall.map(c => c === '.' ? '○' : c).join(' ') + ' ║');
  console.log('╚' + '═'.repeat(pad - 3) + '╦' + '╌╌╌╤'.repeat(rooms.length - 1) + '╌╌╌╦' + '═'.repeat(pad - 3) + '╝');
  for (const d of range(depth)) {
    let line = ' '.repeat(pad - 2) + '║ ';
    line += rooms.map(r => r[depth - d - 1] ?? '○').join(' │ ');
    line += ' ║';
    console.log(line);
  }
  console.log(' '.repeat(pad - 2) + '╚' + '═══╧'.repeat(rooms.length - 1) + '═══╝');
}

function getGoalKeyForState(initialState, depth = 2) {
  const hall = '.'.repeat(initialState.hall.length);
  const rooms = initialState.rooms.map((_, i) => String.fromCharCode(65 + i).repeat(depth));
  return [hall, ...rooms].join('/');
}

function stateToKey({ hall, rooms }, depth = 2) {
  return hall.join('') + '/' + rooms.map(r => r.join('').padEnd(depth, '.')).join('/');
}

function keyToState(key) {
  const [hall, ...rooms] = key.split('/').map(o => o.split(''));
  return { hall, rooms: rooms.map(r => r.filter(c => c !== '.')) };
}

/*
 * Most of this challenge is in state management and finding valid transitions.
 * The rest is a pretty typical pathfinding problem.
 */
function buildReachableStates(startKey, depth = 2) {
  const { hall, rooms } = keyToState(startKey);
  const reachable = []; // Array<{ key: string; cost: number }>

  const roomOffset = Math.floor(hall.length / 2) - rooms.length + 1;
  const roomCoords = rooms.map((_, i) => i * roomOffset + 2);

  // The rules are different for actors in a room, and actors in the hall.
  const actorsInHall = hall.map((c,i) => c === '.' ? null : [c, i]).filter(c => c);
  for (const [kind, pos] of actorsInHall) {
    const targetRoom = kind.charCodeAt(0) - 65;
    const targetTenants = rooms[targetRoom].filter(c => c !== '.');
    const stepCost = [1, 10, 100, 1000][targetRoom];
    // A move starting from the hall can ONLY end in the destination room.
    // The destination room needs to have space open, and any other actors
    // already inside must be of the same kind (it's also their destination).
    if (targetTenants.length < depth && targetTenants.every(r => r === kind)) {
      const door = roomCoords[targetRoom];
      const rangeStart = door > pos ? pos + 1 : door;
      const rangeEnd = door < pos ? pos : door + 1;
      const stretch = hall.slice(rangeStart, rangeEnd);

      // No clear path.
      if (!stretch.every(c => c === '.')) continue;

      // The cost is how many moves it makes in the hallway to get above the room,
      // Plus (depth - number of actors already in that room), times that actor kind's step cost.
      const pathCost = stepCost * (stretch.length + depth - rooms[targetRoom].length);

      // Build the new state and push it to the reachable list.
      const newHall = [...hall];
      newHall[pos] = '.';
      const newRooms = rooms.slice();
      newRooms[targetRoom] = [...targetTenants, kind];
      const newKey = stateToKey({ hall: newHall, rooms: newRooms });
      validityCheck(newKey, rooms.length * depth);
      reachable.push({ key: newKey, cost: pathCost });
    }
  }

  // Now for the actors starting their move in a room.
  for (const roomIndex of range(rooms.length)) {
    const tenants = rooms[roomIndex].filter(c => c !== '.');
    // Only the actor closest to the door can move at all.
    if (!tenants.length) continue;

    const startRoomCoord = roomCoords[roomIndex];
    const top = tenants.slice(-1)[0];
    const targetRoom = top.charCodeAt(0) - 65;
    const targetRoomCoord = roomCoords[targetRoom];
    const stepCost = [1, 10, 100, 1000][targetRoom];

    // There are two sorts of moves an actor starting in a room can make:
    // - Leave the room, then take any number of steps in the hallway.
    // - Leave the room, move towards the target room, then enter that room.
    // In both cases, it still can't cross a space that's already occupied, and can't stop above a room.
    const costToReachHall = stepCost * (depth - tenants.length + 1);

    const reachableHallPos = [];
    for (const left of range(startRoomCoord, -1)) {
      if (hall[left] === '.') reachableHallPos.push(left);
      else break;
    }
    for (const right of range(startRoomCoord, hall.length)) {
      if (hall[right] === '.') reachableHallPos.push(right);
      else break;
    }
    for (const hp of reachableHallPos) {
      if (hp === startRoomCoord) continue; // That path would be a no-op.
      const costToMoveInHall = stepCost * Math.abs(hp - startRoomCoord);
      // If the position is not over a room, it's legal to stop there directly.
      if (!roomCoords.includes(hp)) {
        const pathCost = costToReachHall + costToMoveInHall;
        const newHall = [...hall];
        newHall.splice(hp, 1, top);
        const newRooms = [...rooms];
        newRooms[roomIndex] = tenants.slice(0, -1);
        const newKey = stateToKey({ rooms: newRooms, hall: newHall });
        validityCheck(newKey, rooms.length * depth);
        reachable.push({ key: newKey, cost: pathCost });
      }
      // If the position is over the destination room, and it's legal to move there, that state is good.
      if (hp === targetRoomCoord && rooms[targetRoom].length < depth && rooms[targetRoom].every(r => r === top)) {
        const costToEnterRoom = stepCost * (depth - rooms[targetRoom].length);
        const pathCost = costToReachHall + costToMoveInHall + costToEnterRoom;
        const newRooms = rooms.slice();
        newRooms[roomIndex] = tenants.slice(0, -1);
        newRooms[targetRoom] = [...rooms[targetRoom], top];
        const newKey = stateToKey({ hall, rooms: newRooms });
        validityCheck(newKey, rooms.length * depth);
        reachable.push({ key: newKey, cost: pathCost });
      }
      // Otherwise, it's not a legal move. Nothing to push.
    }
  }

  // Error check...
  for (const { key } of reachable) {
    validityCheck(key, rooms.length * depth);
  }

  return reachable;
}

function validityCheck(key, expected) {
  const actors = key.replaceAll('/', '').replaceAll('.', '');
  if (actors.length !== expected) {
    throw new Error(`Key ${key} is an invalid state! It has the wrong number of actors.`);
  }
}

function heuristic(key, goal) {
  // Estimate the cost to get from this key to the goal state.
  // Balancing how expensive this method is to run with how expensive finding the real answer is,
  // is a bit of an art (read: guesswork until you can try several in real conditions).
  if (key === goal) return 0;
  let cost = 0;
  for (const c of range(key.length)) if (goal[c] !== key[c]) cost += [1, 10, 100, 1000, 0][(goal[c] ?? 69) - 65];
  return cost;
}

function solve(startState, depth = 2, isTest = false) {
  const startKey = stateToKey(startState);
  const goalKey = getGoalKeyForState(startState, depth);
  const graph = new Map(); // stateKey -> Array<{ key, cost }>; holds legal state transitions and their costs.

  graph.set(startKey, buildReachableStates(startKey, depth));

  /* A* approach */
  const openSet = new Set();
  openSet.add(startKey);
  const cameFrom = new Map(); // key -> key that reached it
  const gScore = new Map(); // If missing a value, should yield Infinity
  gScore.set(startKey, 0);
  const fScore = new Map(); // If missing a value, should yield Infinity
  fScore.set(startKey, heuristic(startKey, goalKey));

  while (openSet.size) {
    // if (openSet.size % 100 === 0) console.log(openSet.size);
    const current = [...openSet].sort(
      (a, b) => (fScore.get(a) ?? Number.MAX_SAFE_INTEGER) - (fScore.get(b) ?? Number.MAX_SAFE_INTEGER)
    )[0];

    if (current === goalKey) {
      console.log(`Explored a total of ${graph.size} states to find a path costing ${gScore.get(goalKey)}`);
      // Don't need to reconstruct the path, just need to know what it costs.
      return gScore.get(goalKey);
    }

    openSet.delete(current);
    const neighbors = graph.get(current) ?? graph.set(current, buildReachableStates(current, depth)).get(current);
    for (const { key: neighbor, cost: jumpCost } of neighbors) {
      const newG = (gScore.get(current) ?? Infinity) + jumpCost;
      if (newG < (gScore.get(neighbor) ?? Infinity)) {
        cameFrom.set(neighbor, current);
        gScore.set(neighbor, newG);
        fScore.set(newG + heuristic(neighbor, goalKey));
        if (!openSet.has(neighbor)) openSet.add(neighbor);
      }
    }
  }
  throw new Error('No path found using A*.');
};

// Part 2 is the same problem, but with deeper wells.
function expandStart(initial) {
  const rooms = [...initial.rooms];
  rooms[0] = [rooms[0][0], 'D', 'D', rooms[0][1]];
  rooms[1] = [rooms[1][0], 'B', 'C', rooms[1][1]];
  rooms[2] = [rooms[2][0], 'A', 'B', rooms[2][1]];
  rooms[3] = [rooms[3][0], 'C', 'A', rooms[3][1]];
  return { hall: initial.hall, rooms };
}

(await import('./aoc.mjs')).default(
  2021, 23,
  (start, isTest) => solve(start, 2, isTest), 12521,
  (start, isTest) => solve(expandStart(start), 4, isTest), 44169,
  data => {
    const hall = data[1].slice(1, -1).split('');
    // This puzzle has a strange input format.
    // My pre-parser trims lines, so the leading two spaces on the last lines are missing.
    const rooms = [
      // Bottom entry first. This way, pop() pulls the top and push() sets the top.
      [data[3][1], data[2][3]],
      [data[3][3], data[2][5]],
      [data[3][5], data[2][7]],
      [data[3][7], data[2][9]],
    ];
    return { hall, rooms };
  },
);