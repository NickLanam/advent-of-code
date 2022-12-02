import { sum } from './utils/array.mjs';

(await import('./aoc.mjs')).default(
  2022, 2,
  (data) => {
    return sum(data.map(l => l.split(' ')).map(([a, b]) => {
      const c = ['A', 'B', 'C'].indexOf(a);
      const d = ['X', 'Y', 'Z'].indexOf(b);
      const rightWin = (c === 0 && d === 1) || (c === 1 && d === 2) || (c === 2 && d === 0);
      const leftWin = (c !== d) && !rightWin;
      return leftWin ? d + 1 : (rightWin ? d + 7 : d + 4);
    }));
  }, 15,
  (data) => {
    return sum(data.map(l => l.split(' ')).map(([a, b]) => {
      const c = ['A', 'B', 'C'].indexOf(a);
      let d = -1;
      switch (b) {
        case 'X': d = (c + 2) % 3; break;
        case 'Y': d = c; break;
        case 'Z': d = (c + 1) % 3; break;
      }
      const rightWin = (c === 0 && d === 1) || (c === 1 && d === 2) || (c === 2 && d === 0);
      const leftWin = (c !== d) && !rightWin;
      return leftWin ? d + 1 : (rightWin ? d + 7 : d + 4);
    }));
  }, 12,
  data => data
);