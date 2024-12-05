/** @typedef {'N' | 'E' | 'S' | 'W'} CardinanDirection */
/** @typedef {'NW' | 'NE' | 'SW' | 'SE'} IntercardinalDirection */
/** @typedef {CardinanDirection | IntercardinalDirection} Direction */

/**
 * Given a 2d array of strings and an x,y coordinate,
 * return an array of { x, y, v, dir } objects that exist
 * in neighboring positions.
 * 
 * @param {string[][]} grid 
 * @param {number} x 
 * @param {number} y 
 * @param {boolean} includeDiagonals
 * @returns {{ x: number, y: number, v: string, dir: Direction }[]}
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
