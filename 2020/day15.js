const { getInput, fromRaw } = require('./utils');
const input = getInput(15);
const samples = [
  ['0,3,6']
];

const which = samples[0];

const run = (start, stopOn, log = false) =>  {
  // A note on using a map rather than a plain object here:
  // Although objects have shorter syntax, they are designed for small datasets. Maps fit large data.
  // Objects use a cheap hash algorithm, at the cost of more collisions. This is good for small data.
  // Maps use a more expensive, more collision-resistant hash algorithm. This is good for large data.
  // 30 million turns is enough to see. Run time with objects is 10 minutes; with Maps is 10 seconds.
  const spoken = new Map();
  start = start[0].split(',').map(s => Number(s));
  let lastSpoken = start.slice(-1)[0];
  for (let i = 0; i < start.length; i++) {
    const v = start[i];
    const next = spoken.has(v) ? [spoken.get(v)[1], i + 1] : [i + 1, i + 1];
    spoken.set(v, next);
  }

  if (log) console.log('Starting with ', spoken, 'most recently spoke', lastSpoken);

  let distance;

  for (let turn = spoken.size + 1; turn <= stopOn; turn++) {
    const last = spoken.get(lastSpoken);
    distance = last[1] - last[0];
    const old = spoken.get(distance);
    spoken.set(distance, spoken.has(distance) ? [spoken.get(distance)[1], turn] : [turn, turn]);
    lastSpoken = distance;
    if (log) {
      if (turn % 100000 === 0) process.stdout.write('.');
      if (turn % 1000000 === 0) console.log(`On turn ${turn/1000000}m, will stop at ${stopOn}`);
    }
  }
  return lastSpoken;
}

const day15star1 = run(input, 2020);
console.log('Star 1: ', day15star1);

const day15star2 = run(input, 30000000);
console.log('Star 2: ', day15star2)