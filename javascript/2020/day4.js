const { getInput } = require('./utils');

const validHeight = h => {
  const where = h.split ``.findIndex(c => /[a-z]/.test(c));
  const n = Number(h.substring(0, where));
  const metric = h.substring(where) === 'cm';
  return metric ? n >= 150 && n <= 193 : n >= 59 && n <= 76;
}

const ppts = getInput(4, false, '\n\n')
  .map(ppt => ppt
    .split(/\s+/g)
    .reduce((a, c) => ({
      ...a,
      [c.split(':')[0]]: c.split(':')[1]
    }), {}));
const s1 = ppts.filter(ppt => ppt.byr && ppt.iyr && ppt.eyr && ppt.hgt && ppt.hcl && ppt.ecl && ppt.pid);
const s2 = ppts.filter(ppt =>
  Number(ppt.byr) >= 1920 && Number(ppt.byr <= 2002) &&
  Number(ppt.iyr) >= 2010 && Number(ppt.iyr <= 2020) &&
  Number(ppt.eyr) >= 2020 && Number(ppt.eyr <= 2030) &&
  ppt.hgt && validHeight(ppt.hgt) &&
  /^#[0-9a-f]{6}$/.test(ppt.hcl) && ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(ppt.ecl) &&
  /^\d{9}$/.test(ppt.pid));

console.log('Star 1: ', s1.length);
console.log('Star 2: ', s2.length);