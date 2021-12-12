const SCORES_1 = { ')': 3, ']': 57, '}': 1197, '>': 25137 };
const SCORES_2 = { ')': 1, ']': 2, '}': 3, '>': 4 };
const OPENERS = ['(', '[', '{', '<'];
const CLOSERS = [')', ']', '}', '>'];

function process(line) {
  let illegal = [];
  const stack = [];
  for (const char of line) {
    if (OPENERS.includes(char)) {
      stack.push(char);
    } else if (CLOSERS.includes(char) && stack[stack.length - 1] == OPENERS[CLOSERS.indexOf(char)]) {
      stack.pop();
    } else {
      illegal.push(char);
    }
  }
  return { valid: !illegal.length && !stack.length, illegal, stack, line };
}

(await import('./aoc.mjs')).default(
  2021, 10,
  (data) => data
    .filter(line => line.illegal.length > 0)
    .map(line => SCORES_1[line.illegal[0]])
    .reduce((a, c) => a + c, 0),
  26397,
  (data) => {
    const incomplete = data.filter(line => line.illegal.length === 0 && !line.valid);
    const closers = incomplete.map(line => line.stack.reverse().map(char => CLOSERS[OPENERS.indexOf(char)]));
    const scores = closers.map(line => line.reduce(
      (a, c) => ((a * 5) + SCORES_2[c]),
      0
    ));
    if (scores.length % 2 !== 1) throw new Error('There should be an odd number of lines for this.');
    scores.sort((a, b) => b - a);
    return scores[Math.floor(scores.length / 2)];
  }, 288957,
  data => data.map(l => l.split('')).map(process)
);