const { getInput } = require('./utils');

const groups = getInput(6, false, '\n\n');

const day6star1 = groups
  .map(g => [
    ...new Set(
      g.replace(/\s+/g, '')
      .split(''))
  ].length)
  .reduce((a, c) => a + c, 0);

const alpha = 'abcdefghijklmnopqrstuvwxyz'.split('');
const day6star2 = groups
  .map(g => alpha
    .filter(a => g
      .split('\n')
      .every(p => p.includes(a))).length)
  .reduce((a, c) => a + c, 0);

console.log('Star 1: ', day6star1);
console.log('Star 2: ', day6star2);