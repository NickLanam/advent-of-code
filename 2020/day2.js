const { getInput } = require('./utils');

const day2star1 = (() => {
  const lines = getInput(2);
  const validators = lines.map(l => {
    let [r, p] = l.split(': ');
    r = r.replace(/^(\d+)-(\d+) ([a-z])$/, '^([^$3]*$3[^$3]*){$1,$2}$$');
    return { regex: new RegExp(r), password: p };
  });
  return validators.filter(v => v.regex.test(v.password)).length;
})();

const day2star2 = (() => {
  const lines = getInput(2);
  const valid = lines.map(l => {
    const [_, i, j, c, p] = l.match(/^(\d+)-(\d+) ([a-z]): ([a-z]+)$/);
    return (p[i - 1] === c) ^ (p[j - 1] === c) ? p : null;
  }).filter(p => p);
  return valid.length;
})();

console.log('Star 1: ', day2star1);
console.log('Star 2: ', day2star2);