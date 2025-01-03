import aoc from './aoc.mjs';

const part1expected = 'SKIP'/* 6 */;
const part2expected = 6;

const parse = (data) => ({
  instructions: data[0].split(''),
  nodes: data.slice(2).map(l => l.replace(' = (', ',').replace(')', '').replace(', ', ',').split(',')).reduce((a, [n, l, r]) => ({ ...a, [n]: [l, r] }), {}),
});

const part1 = ({ instructions, nodes}, isSample) => {
  // There are different samples for part 1 and 2, and each breaks the other
  if (isSample) return 'SKIP';

  let node = 'AAA';
  let stepsTaken = 0;
  while (node !== 'ZZZ') {
    node = nodes[node][instructions[stepsTaken % instructions.length] === 'L' ? 0 : 1];
    stepsTaken++;
  }
  return stepsTaken;
};

const gcd = (a, b) => b === 0 ? a : gcd(b, a % b);
const lcm = (a, b) => a / gcd(a, b) * b;
const lcmAll = (...all) => all.reduce(lcm, 1);

const part2 = ({ instructions, nodes }, isSample) => {
  const starters = Object.keys(nodes).filter(k => k.endsWith('A'));
  // It's not computationally feasible to run the whole simulation until every loop aligns.
  // What is feasible is checking how long each one takes to get there, then getting the LCM.
  // The sample cycles cleanly, so we'll assume the real input does too.
  const stepsTakenByEach = starters.map((start) => {
    let stepsTaken = 0;
    let node = start;
    while (!node.endsWith('Z')) {
      node = nodes[node][instructions[stepsTaken % instructions.length] === 'L' ? 0 : 1];
      stepsTaken++;
    }
    return stepsTaken;
  });
  return lcmAll(...stepsTakenByEach);
};

aoc(2023, 8, part1, part1expected, part2, part2expected, parse);
