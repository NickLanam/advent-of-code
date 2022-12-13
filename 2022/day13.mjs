import { sum } from './utils/array.mjs';

function testValid(left, right, depth = 0) {
  if (typeof left === 'number' && typeof right === 'number') {
    return Math.sign(left - right);
  }

  if (!Array.isArray(left)) left = [left];
  if (!Array.isArray(right)) right = [right];

  let li = 0;
  for (; li < left.length; li++) {
    if (right.length <= li) {
      return 1;
    }
    const v = testValid(left[li], right[li], depth + 1);
    if (v !== 0) {
      return v;
    }
  }
  if (left.length < right.length) return -1;
  return 0;
}

(await import('./aoc.mjs')).default(
  2022, 13,
  (pairs) => {
    const validIndices = pairs.map((p, i) => ({ p, i, valid: testValid(...p) })).filter(({ valid }) => valid < 0);
    return sum(validIndices.map(({ i }) => i + 1));
  }, 13,
  (pairs) => {
    const unpaired = [...pairs.flat(1), [[2]], [[6]]].sort(testValid).map(p => JSON.stringify(p));
    const two = unpaired.findIndex(p => p === '[[2]]');
    const six = unpaired.findIndex(p => p === '[[6]]');
    return (two + 1) * (six + 1);
  }, 140,
  data => {
    const pairs = [];
    for (let l = 0; l < data.length; l += 3) {
      pairs.push([JSON.parse(data[l]), JSON.parse(data[l + 1])]);
    }
    return pairs;
  }
);