const { getInput } = require('./utils');

function treeCheck(lines, rowInc, colInc) {
  let t = 0;
  for (let r = 0, c = 0; r < lines.length; r += rowInc, c = (c + colInc) % lines[0].length) {
    if (lines[r][c] === '#') t++;
  }
  return t;
}

const day3star1 = (() => {
  const lines = getInput(3);
  return treeCheck(lines, 1, 3);
})();

const day3star2 = (() => {
  const lines = getInput(3);
  return treeCheck(lines, 1, 1) * treeCheck(lines, 1, 3) * treeCheck(lines, 1, 5) * treeCheck(lines, 1, 7) * treeCheck(lines, 2, 1);
})();

console.log('Star 1: ', day3star1);
console.log('Star 2: ', day3star2);