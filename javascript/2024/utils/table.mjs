import { green, yellow, cyan, blue, red, grey } from './color.mjs';

// Keyed on north, east, south, west as bits in a 4-bit value
const connectorsRounded = {
  [0b0000]: '○', // No connection
  [0b0001]: '╴', // West
  [0b0010]: '╷', // South
  [0b0011]: '╮', // Southwest
  [0b0100]: '╶', // East
  [0b0101]: '─', // West-East
  [0b0110]: '╭', //Southeast
  [0b0111]: '┬', // Non-north
  [0b1000]: '╵', // North
  [0b1001]: '╯', // Northwest
  [0b1010]: '│', // North-south
  [0b1011]: '┤', // Non-East
  [0b1100]: '╰', // Northeast
  [0b1101]: '┴', // Non-south
  [0b1110]: '├', // Non-west
  [0b1111]: '┼', // All
};

const connectorsSquared = {
  [0b0000]: '▫', // No connection
  [0b0001]: '╴', // West
  [0b0010]: '╷', // South
  [0b0011]: '┐', // Southwest
  [0b0100]: '╶', // East
  [0b0101]: '─', // West-East
  [0b0110]: '┌', //Southeast
  [0b0111]: '┬', // Non-north
  [0b1000]: '╵', // North
  [0b1001]: '┘', // Northwest
  [0b1010]: '│', // North-south
  [0b1011]: '┤', // Non-East
  [0b1100]: '└', // Northeast
  [0b1101]: '┴', // Non-south
  [0b1110]: '├', // Non-west
  [0b1111]: '┼', // All
};

/**
 * 
 * @param {any} v 
 * @returns {{ plain: string, fancy: string }}
 */
function stringify(v) {
  let s;
  let f;
  if (typeof v === 'string') {
    s = v;
    f = green(s);
  } else if (typeof v === 'number' || typeof v === 'bigint') {
    s = String(v);
    f = yellow(s);
  } else if (typeof v === 'boolean') {
    s = String(v);
    f = blue(s);
  } else if (typeof v === 'symbol') {
    s = String(v);
    f = cyan(s);
  } else if (typeof v === 'function') {
    s = String(v);
    f = red(s);
  } else if (typeof v === 'undefined') {
    s = 'undefined';
    f = grey(s);
  } else if (v === null) {
    s = 'null';
    f = grey(s);
  } else if (typeof v === 'object') {
    if (Array.isArray(v)) {
      s = '[' + v.map(v2 => stringify(v2).plain).join(', ') + ']';
      if (s.length < 30) {
        f = grey('[') + v.map(v2 => stringify(v2).fancy).join(grey(', ')) + grey(']');
      } else {
        s = `Array(${v.length})`;
        f = s;
      }
    } else {
      s = '{' + Object.entries(v).map(([k2, v2]) => stringify(k2).plain + ': ' + stringify(v2).plain).join(', ') + '}';
      if (s.length < 30) {
        f = grey('{') + Object.entries(v).map(([k2, v2]) => stringify(k2).fancy + grey(': ') + stringify(v2).fancy).join(grey(', ')) + grey('}');
      } else {
        s = `Object(${Object.keys(v).length} keys)`;
        f = s;
      }
    }
  } else {
    s = typeof v;
    f = red(s);
  }
  return { plain: s, fancy: f };
}

/**
 * Prints a 2D array using box-drawing characters and color
 * @param {any[][]} grid 
 */
export function printTable(grid, style = 'round') {
  const c = { 'square': connectorsSquared, 'round': connectorsRounded }[style] ?? connectorsSquared;
  let h = grid.length;
  let w = grid[0]?.length;
  let plained = Array(h).fill(0).map(() => Array(w).fill(0));
  let fancied = Array(h).fill(0).map(() => Array(w).fill(0));
  let cellWidths = Array(w).fill(0);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const v = grid[y][x];
      const { plain, fancy } = stringify(v);
      cellWidths[x] = Math.max(cellWidths[x], plain.length);
      plained[y][x] = plain;
      fancied[y][x] = fancy;
    }
  }
  // First line
  console.log(c[0b0110] + cellWidths.map(cw => c[0b0101].repeat(cw + 2)).join(c[0b0111]) + c[0b0011]);
  for (let fy = 0; fy < h; fy++) {
    console.log(
      c[0b1010] + ' ' +
      fancied[fy].map((f, fx) => {
        let padBy = cellWidths[fx] - plained[fy][fx].length;
        let pad = padBy <= 0 ? '' : ' '.repeat(padBy);
        return f + pad;
      }).join(` ${c[0b1010]} `) +
      ' ' + c[0b1010]
    );
    if (fy < h - 1) {
      // Divider between rows
      console.log(c[0b1110] + cellWidths.map(cw => c[0b0101].repeat(cw + 2)).join(c[0b1111]) + c[0b1011]);
    }
  }
  // Last line
  console.log(c[0b1100] + cellWidths.map(cw => c[0b0101].repeat(cw + 2)).join(c[0b1101]) + c[0b1001]);
}