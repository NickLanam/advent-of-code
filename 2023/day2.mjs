import { sum } from './utils/array.mjs';

(await import('./aoc.mjs')).default(
  2023, 2,
  (data) => {
    const limits = { red: 12, green: 13, blue: 14 };
    const possibleIds = [];
    gameLoop: for (const { id, pulls } of data) {
      for (const p of pulls) {
        if (p.red > limits.red || p.green > limits.green || p.blue > limits.blue) {
          continue gameLoop;
        }
      }
      possibleIds.push(id);
    }
    return sum(possibleIds);
  }, 8,
  (data) => {
    const powers = [];
    for (const { pulls } of data) {
      const maxes = { red: 1, green: 1, blue: 1 };
      for (const p of pulls) {
        maxes.red = Math.max(maxes.red, p.red ?? -Infinity);
        maxes.green = Math.max(maxes.green, p.green ?? -Infinity);
        maxes.blue = Math.max(maxes.blue, p.blue ?? -Infinity);
      }
      powers.push(maxes.red * maxes.green * maxes.blue);
    }
    return sum(powers);
  }, 2286,
  data => data.map(line => {
    const id = +line.match(/^Game (\d+)/)[1];
    const sections = line.split(': ')[1].split('; ');
    const pulls = sections
      .map(s => s.split(', ')
        .reduce((a, c) => ({
          ...a,
          [c.split(' ')[1]]: +c.split(' ')[0],
        }), {}));
    return { id, sections, pulls };
  }),
);
