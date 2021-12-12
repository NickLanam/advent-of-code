const { isSymbol } = require('util');
const { getInput, fromRaw } = require('./utils');
const input = getInput(10).map(l => Number(l));
const shortSample = fromRaw(`16
10
15
5
1
11
7
19
6
12
4`).map(l => Number(l)); // Star 1: 7*5=35; Star 2: 8
const longSample = fromRaw(`28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3`).map(l => Number(l)); // Star 1: 22*10=220; Star 2: 19208

const day10star1 = (() =>  {
  const list = input.sort((a, b) => a - b);
  list.unshift(0);
  list.push(list.slice(-1)[0] + 3);
  const diffs = { '1': 0, '2': 0, '3': 0 };
  list.forEach((v, i) => {
    diffs[String(list[i + 1] - list[i])]++;
  });
  return diffs['1'] * diffs['3'];
})();

const day10star2 = (() => {
  const list = input.sort((a, b) => a - b);
  list.unshift(0);
  list.push(list.slice(-1)[0] + 3);
  const seen = Array(list).fill(null);
  console.log(list.length);
  for (let i = list.length - 1; i >= 0; i--) {
    seen[i] = combinations(i, seen);
  }
  seen[0]++; // There is one path coming in.
  return seen[0];

  // Starting from the end allows us to always reach a memoized value, and do this iteratively rather than recursively.
  function combinations(startIndex, breadths) {
    if (breadths[startIndex] != null) return breadths[startIndex];
    const start = list[startIndex];
    const nexts = list
      .slice(startIndex + 1)
      .map((v, i) => i + startIndex + 1)
      .filter(j => list[j] - start <= 3 && list[j] - start > 0);
    // console.log(`list[${startIndex}] = ${list[startIndex]} `
    // + `→ [${nexts.map(n => list[n]).join(', ')}] (${nexts.length}) `
    // + `→ ${[Math.max(0, nexts.length - 1), ...nexts.map(n => breadths[n])].join(' + ')} `
    // + `= ${nexts.reduce((a, i) => a + breadths[i], Math.max(0, nexts.length - 1))}`);
    return nexts.reduce((a, i) => a + breadths[i], Math.max(0, nexts.length - 1));
  }
})();

// const day10star2 = (() => {
//   const list = shortSample.sort((a, b) => a - b);
//   list.unshift(0);
//   list.push(list.slice(-1)[0] + 3);
//   console.log('Okay list is ready to process');
//   let stopLim = 1000000;
//   let sum = 1;
//   let stack = list.slice(1).map((v, i) => i + 1).filter(i => list[i] - start <= 3 && list[i] - start > 0);
//   while (stack.length && stopLim-- > 0) {

//   }
// });

console.log('Star 1: ', day10star1);
console.log('Star 2: ', day10star2)