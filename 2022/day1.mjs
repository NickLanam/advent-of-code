import { sum } from './utils/array.mjs';

(await import('./aoc.mjs')).default(
  2022, 1,
  (maxCalories) => maxCalories[0], 24000,
  (maxCalories) => sum(maxCalories.slice(0, 3)), 45000,
  (data) => {
    let e = [[]];
    for (const c of data) {
      if (!c) e.push([]);
      else e[e.length - 1].push(+c);
    }
    e = e.map(v => sum(v));
    e.sort((a, b) => b - a);
    return e;
  }
);