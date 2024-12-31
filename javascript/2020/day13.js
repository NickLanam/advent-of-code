const { getInput, fromRaw } = require('./utils');
const input = getInput(13);
const sample = fromRaw(`939
7,13,x,x,59,x,31,19`); // Gets 3606 for part 1, 1068781 for part 2
const part2samples = [
  ['17,x,13,19', 3417],
  ['67,7,59,61', 754018],
  ['67,x,7,59,61', 779210],
  ['67,7,x,59,61', 1261476],
  ['1789,37,47,1889', 1202161486],
];
const expectedPart2 = 379786358533423;

const which = input;

const day13star1 = (() =>  {
  let [earliest, busesIn] = which;
  earliest = Number(earliest);
  busesIn = busesIn.split(',')
    .filter(t => t && t !== 'x')
    .map(d => Number(d));
  const buses = busesIn.map((bus, b) => [bus, Math.ceil(earliest / bus) * bus]);
  const best = buses.sort((a, b) => a[1] - b[1])[0];
  const [soonestId, soonestTime] = best;
  return soonestId * (soonestTime - earliest);
})();

const day13star2fn = (run, log = false) => {
  let [_, busesIn] = run;

  // This is just an implementation of the Chinese Remainder Theorem, where the properties of this problem set up the congruences.
  const schedule = busesIn.split(',')
    .map(d => Number(d));

  const bi = Array(schedule.length)
    .fill(0)
    .map((_, i) => schedule.length - i - 1)
    .filter((_, i) => !Number.isNaN(schedule[i]))
  const ni = schedule
    .filter(s => !Number.isNaN(s))
  const N = ni.reduce((a, c) => a * c, 1); // This is the timestamp when the pattern begins to repeat.
  const Ni = ni.map(n => N / n);
  if (log) {
    console.log(`Schedule: [${schedule.join(',')}] (length ${schedule.length})`);
    console.log('We care about:', ni, `(length ${ni.length})`);
    console.log('The congruences are', bi.map((b, i) => `t ≋ ${b} (mod ${ni[i]})`));
    console.log('The pattern repeats starting at timestamp', N, 'which is', Ni.join(' * '));
  }
  const ti = bi.map((_, i) => {
    const factor = Ni[i] % ni[i]; // Smaller multiplications by doing this one upfront.
    for (let trial = 1; trial < 1000000; trial++) {
      if ((factor * trial) % ni[i] === 1) {
        return trial;
      }
    }
    throw new Error(`A value for ti could not be found within [1, 100000) for schedule departing every ${ni[i]} minutes`);
  });
  const products = ti.map((t, i) => t * bi[i] * Ni[i]);
  const sumOfProducts = products.reduce((a, c) => a + c, 0);
  return (sumOfProducts - schedule.length + 1) % N; // Remember, we actually computed when the LAST bus arrives but we want to know when the first one does.
};

// Using Chinese Remainder Theorem gets the correct answer (with a lot of code) for all of the samples, but not the real input!
console.log('Using pure math:')
for (let [schedule, expected] of[...part2samples, [input[1], expectedPart2]]) {
  let result;
  try { result = day13star2fn([0, schedule]); } catch (e) { console.error(e.message); }
  if (result !== expected) {
    console.error('✘ For input', schedule, 'got', result, 'but expected', expected);
    console.error('  Running it again with logging enabled to see what it did');
    day13star2fn([0, schedule], true);
  } else console.log('✓', schedule, expected);
}

// Cheesing it is way less code, actually. Go figure.
console.log('Using the power of code:');
for (let [schedule, expected] of[...part2samples, [input[1], expectedPart2]]) {
  const result = cheese2(schedule);
  if (result !== expected) {
    console.error('✘ For input', schedule, 'got', result, 'but expected', expected);
  } else console.log('✓', schedule, expected);
}

function cheese2(run) {
  const schedule = run.split(',')
    .map(s => Number(s));
  let r = 1;
  let earliest = 0;
  for (let [i, s] of schedule.entries()) {
    if (Number.isNaN(s)) continue;
    while ((earliest + i) % s !== 0) earliest += r;
    r *= s;
  }
  return earliest;
}

const day13star2 = day13star2fn(input);

console.log('Star 1: ', day13star1);
console.log('Star 2: ', day13star2);
if (day13star2 !== expectedPart2) console.error(`Expected ${expectedPart2} but got ${day13star2}. expected - actual = ${expectedPart2 - day13star2}`);