const { getInput, fromRaw } = require('./utils');
const sample = fromRaw(`35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576`).map(l => Number(l));
let input = getInput(9).map(l => Number(l));

const preambleLength = input === sample ? 5 : 25; // 5 for the sample

function findSum(buffer, v) {
  for (const i in buffer) {
    for (let j = i; j < buffer.length; j++) {
      if (buffer[i] + buffer[j] === v) return true;
    }
  }
  return false;
}

const day9star1 = (() =>  {
  const buffer = input.slice(0, preambleLength);
  for (let i = preambleLength; i < input.length; i++) {
    const v = input[i];

    const works = findSum(buffer, v);
    if (!works) return v;

    buffer.shift();
    buffer.push(v);
  }
  throw new Error('Failed to find');
})();

const day9star2 = (() => {
  const seek = day9star1;

  for (let i = 0; i < input.length; i++) {
    for (let j = i + 1; j < input.length; j++) {
      const slice = input.slice(i, j + 1);
      const sum = slice.reduce((a, c) => a + c, 0);
      const lo = Math.min(...slice);
      const hi = Math.max(...slice);
      if (sum === seek) return lo + hi;
    }
  }
  throw new Error('No slice works');
})();

console.log('Star 1: ', day9star1);
console.log('Star 2: ', day9star2)