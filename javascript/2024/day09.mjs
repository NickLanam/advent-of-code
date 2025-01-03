import aoc from './aoc.mjs';

/** @typedef {({ type: 'file', id: number, len: number } | { type: 'space', len: number })[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = 1928;

/** @type Part2Solution */
const part2expected = 2858;

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  /** @type ParsedInput */
  const blocks = [];
  let fid = 0;
  let pos = 0;
  for (let i = 0; i < lines[0].length; i++) {
    const len = +lines[0][i];
    const type = i % 2 === 0 ? 'file' : 'space';
    if (len > 0) {
      blocks.push({ type, len, id: type === 'file' ? fid++ : undefined });
    }
    pos += len;
  }
  return blocks;
};

/**
 * Both parts use the same function to determine the final answer from the mutated input
 * @param {ParsedInput} blocks 
 * @returns {number}
 */
function score(blocks) {
  let pos = 0;
  return blocks.reduce((a, c) => {
    let out = a;
    if (c.type === 'file') {
      for (let i = pos; i < pos + c.len; i++) {
        out += i * c.id;
      }
    }
    pos += c.len;
    return out;
  }, 0);
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = (blocks) => {
  while(true) {
    const lbi = blocks.findLastIndex(({ type }) => type === 'file');
    const lb = blocks[lbi];
    const fsi = blocks.findIndex(({ type, len }) => type === 'space' && len >= 1);
    const fs = blocks[fsi];

    if (fs && lb && lbi > fsi) {
      const newFileBlock = { type: 'file', len: 1, id: lb.id };
      const newSpaceBlock = fs.len >= 2 ? { type: 'space', len: fs.len - 1 } : undefined;

      if (newSpaceBlock) {
        blocks.splice(fsi, 1, newFileBlock, newSpaceBlock);
      } else {
        blocks.splice(fsi, 1, newFileBlock);
      }
      if (lb.len >= 2) {
        lb.len--;
      } else {
        blocks.splice(newSpaceBlock ? lbi + 1 : lbi, 1);
      }
    } else {
      break;
    }
  }

  return score(blocks);
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (blocks) => {
  const toMove = blocks.filter(b => b.type === 'file');
  toMove.sort((a, b) => b.id - a.id);
  for (const m of toMove) {
    const lbi = blocks.findIndex(b => b.id === m.id);
    const lb = blocks[lbi];
    const fsi = blocks.findIndex(({ type, len }) => type === 'space' && len >= lb.len);
    const fs = blocks[fsi];
    if (fs && lb && lbi > fsi) {
      const splitSpaceBlock = fs.len > lb.len ? { type: 'space', len: fs.len - lb.len } : undefined;
      const gapSpaceBlock = { type: 'space', len: lb.len };

      if (splitSpaceBlock) {
        blocks.splice(fsi, 1, m, splitSpaceBlock);
      } else {
        blocks.splice(fsi, 1, m);
      }
      blocks.splice(splitSpaceBlock ? lbi + 1 : lbi, 1, gapSpaceBlock);
    }
  }

  return score(blocks);
};

aoc({
  year: 2024,
  day: 9,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
