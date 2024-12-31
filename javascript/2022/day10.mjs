import { sum } from './utils/array.mjs'

function report(t, c) {
  if (((t + 20)) % 40 === 0) {
    return t * c;
  }
  return null;
}

(await import('./aoc.mjs')).default(
  2022, 10,
  (data) => {
    let t = 0;
    let x = 1;
    const scoreHistory = [];
    
    for (const line of data) {
      if (line === 'noop') {
        t++;
        scoreHistory.push(report(t, x));
      } else {
        const pendingX = Number(line.split(' ')[1]);
        t++;
        scoreHistory.push(report(t, x));
        t++;
        scoreHistory.push(report(t, x));
        x += pendingX;
      }
    }
    return sum(scoreHistory.filter(s => s !== null));
  }, 13140,
  (data) => {
    let pixels = [];
    let t = 0;
    let x = 1;

    const doPx = () => {
      if ([x - 1, x, x + 1].includes(t % 40)) pixels.push('#');
      else pixels.push('.');
    }
    
    for (const line of data) {
      if (line === 'noop') {
        doPx();
        t++;
      } else {
        const pendingX = Number(line.split(' ')[1]);
        doPx();
        t++;
        doPx();
        t++;
        x += pendingX;
      }
    }
    return '\n' + pixels.map((p, i) => (i % 40 === 39) ? `${p}\n` : p).join('');
  }, `
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
`,
  data => data
);