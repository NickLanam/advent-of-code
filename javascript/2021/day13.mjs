function gridString(dots) {
  const maxX = dots.reduce((a, [x]) => Math.max(a, x), 0);
  const maxY = dots.reduce((a, [,y]) => Math.max(a, y), 0);
  let grid = Array(maxY + 1).fill(0).map(_ => Array(maxX + 1).fill('.'));
  for (let [x, y] of dots) {
    grid[y][x] = '#';
  }
  return '\n' + grid.map(row => row.join('')).join('\n');
}

function solve({ dots, folds }, part1) {
  for (const { dir, val: newMax } of folds) {
    const side = dir === 'x' ? 0 : 1;
    for (let i in dots) {
      if (dots[i][side] > newMax) {
        dots[i][side] = 2 * newMax - dots[i][side];
      }
    }

    if (part1) return dots.filter(
      ([x, y], i) => dots.findIndex(([x2, y2]) => x2 === x && y2 === y) === i).length;
  }
  return gridString(dots);
}

(await import('./aoc.mjs')).default(
  2021, 13,
  data => solve(data, true), 17,
  data => solve(data, false), `
#####
#...#
#...#
#...#
#####`,
  lines => {
    const dots = lines.slice(0, lines.indexOf('')).map(dot => dot.split(',').map(v => +v));
    const folds = lines.slice(1 + lines.indexOf('')).map(fold => {
      const [, dir, val] = fold.match(/^fold along ([xy])=(\d+)$/);
      return { dir, val: +val };
    });
    return { dots, folds };
  }
);