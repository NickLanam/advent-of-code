import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number|'NYI'} Part1Solution */
/** @typedef {number|'NYI'} Part2Solution */

/** @type Part1Solution */
const part1expected = 9;

/** @type Part2Solution */
const part2expected = 'NYI';

/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  const where = lines.indexOf('');
  const inits = lines.slice(0, where).map(l => l.split(': '));
  const conds = lines.slice(where + 1).map(l => String(l).match(/^([a-z\d]+) ([A-Z]+) ([a-z\d]+) -> ([a-z\d]+)$/));
  const initialValues = new Map();
  for (const [name, val] of inits) {
    initialValues.set(name, +val === 1);
  }

  const gates = new Map();
  for (const [_, a, cond, b, c] of conds) {
    gates.set(c, { a, b, cond });
  }
  return { initialValues, gates };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ initialValues, gates }) => {
  const values = new Map([...initialValues]);
  const evalStack = [];
  for (const c of gates.keys()) {
    evalStack.push(c);
  }
  while (evalStack.length > 0) {
    const c = evalStack.at(-1);
    if (values.has(c)) {
      evalStack.pop();
      continue;
    }
    const { a, cond, b } = gates.get(c);
    if (values.has(a) && values.has(b)) {
      evalStack.pop();
      let av = values.get(a);
      let bv = values.get(b);
      let out;
      switch (cond) {
        case 'AND': out = av && bv; break;
        case 'OR': out = av || bv; break;
        case 'XOR': out = av !== bv; break;
        default: throw new Error('What is this cond? ' + cond);
      }
      values.set(c, out);
    } else {
      if (!values.has(a)) evalStack.push(a);
      if (!values.has(b)) evalStack.push(b);
    }
  }
  
  const zGates = [...values.keys()].filter(k => k.startsWith('z'));
  zGates.sort((a, b) => parseInt(a.substring(1)) - parseInt(b.substring(1)));
  return parseInt(zGates.reduce((a, c) => String(Number(values.get(c))) + a, ''), 2);
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const part2 = (parsed) => {
  return 'NYI';
};

aoc({
  year: 2024,
  day: 24,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
