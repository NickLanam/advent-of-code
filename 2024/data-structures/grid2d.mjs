/** @typedef {'N' | 'E' | 'S' | 'W'} CardinanDirection */
/** @typedef {'NW' | 'NE' | 'SW' | 'SE'} IntercardinalDirection */
/** @typedef {CardinanDirection | IntercardinalDirection} Direction */

/**
 * Given a 2d array of values and an x,y coordinate,
 * return an array of { x, y, v, dir } objects that exist
 * in neighboring positions.
 * 
 * @param {T[][]} grid 
 * @param {number} x 
 * @param {number} y 
 * @param {boolean} includeDiagonals
 * @returns {{ x: number, y: number, v: T, dir: Direction }[]}
 */
export function neighbors(grid, x, y, includeDiagonals = true) {
  const w = grid[0].length;
  const h = grid.length;
  return [
    [x - 1, y - 1, 'NW'], [x, y - 1, 'N'], [x + 1, y - 1, 'NE'],
    [x - 1, y, 'W'],                       [x + 1, y, 'E'],
    [x - 1, y + 1, 'SW'], [x, y + 1, 'S'], [x + 1, y + 1, 'SE']
  ]
    .filter(([x2, y2, dir]) => (
      x2 >= 0 &&
      x2 < w &&
      y2 >= 0 &&
      y2 < h &&
      (includeDiagonals || dir.length === 1)
    )).map(([x3, y3, dir]) => ({
      x: x3,
      y: y3,
      dir,
      v: grid[y3][x3],
    }));
}

/**
 * 
 * @param {T[][]} grid 
 * @param {number} x 
 * @param {number} y 
 * @param {Direction} dir 
 */
export function lineOfSight(grid, x, y, dir) {
  const w = grid[0].length;
  const h = grid.length;
  const nextCoord = (x2, y2) => {
    switch (dir) {
      case 'N': return  [x2,     y2 - 1];
      case 'NE': return [x2 + 1, y2 - 1];
      case 'E': return  [x2 + 1, y2    ];
      case 'SE': return [x2 + 1, y2 + 1];
      case 'S': return  [x2,     y2 + 1];
      case 'SW': return [x2 - 1, y2 + 1];
      case 'W': return  [x2 - 1, y2,   ];
      case 'NW': return [x2 - 1, y2 - 1];
    }
  }
  let x3 = x;
  let y3 = y;
  const line = [];
  while (true) {
    const n = nextCoord(x3, y3);
    x3 = n[0];
    y3 = n[1];
    if (
      x3 >= 0 &&
      x3 < w &&
      y3 >= 0 &&
      y3 < h
    ) {
      line.push({ x: n[0], y: n[1], dir, v: grid[n[1]][n[0]]});
    } else {
      break;
    }
  };
  return line;
}