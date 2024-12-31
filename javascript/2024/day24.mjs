import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {number} Part1Solution */
/** @typedef {string} Part2Solution */

/** @type Part1Solution */
const part1expected = 2024;

/** @type Part2Solution */
const part2expected = 'SKIP';

const gateRegExp = new RegExp(/^(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})$/);
/**
 * @param {string[]} lines Unparsed input lines
 * @param {1|2} forPart Which star we're working on
 * @returns {ParsedInput}
 */
const parse = (lines, forPart) => {
  
  const where = lines.indexOf('');
  const initialValues = new Map();
  const gates = new Map();

  let width = 0;
  for (const line of lines.slice(0, where)) {
    let [name, val] = line.split(': ');
    val = val === '1';
    initialValues.set(name, +val === 1);
    if (name.startsWith('x')) {
      width++;
    }
  }

  for (const line of lines.slice(where + 1)) {
    const [_, a, cond, b, c] = line.match(gateRegExp);
    gates.set(c, { a, b, cond });
  }

  return { initialValues, gates, width };
};

function valueFromRegisters(values, prefix) {
  let out = 0n;
  for (const [k, v] of values) {
    if (k[0] !== prefix) continue;
    const o = BigInt(k.substring(1));
    if (v === true) {
      out += 0b1n << o;
    }
  }
  return Number(out);
}

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const part1 = ({ initialValues, gates }) => {
  const values = new Map(initialValues);
  while (true) {
    let madeChanges = false;
    for (const c of gates.keys()) {
      if (values.has(c)) continue;
      const { a, b, cond } = gates.get(c);
      if (values.has(a) && values.has(b)) {
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
        madeChanges = true;
      }
    }
    if (!madeChanges) break;
  }

  return valueFromRegisters(values, 'z');
};

/**
 * Checks for wiring defects in a ripple-carry adder
 * @param {ParsedInput} parsed 
 * @param {boolean} isSample
 * @returns {Part2Solution}
 */
const part2 = ({ initialValues, gates, width }, isSample) => {
  // The sample is a completely unrelated circuit with a different solution.
  if (isSample) return part2expected;

  let knownWrong = [];

  const gateEntries = [...gates.entries()];
  for (let i = 0; i < width; i++) {
    const id = String(i).padStart(2, '0');
    const xid = 'x' + id;
    const yid = 'y' + id;
    const zid = 'z' + id;
    const leftXor = gateEntries.find(([_, { a, b, cond }]) => (
      cond === 'XOR' && ((a === xid && b === yid) || (a === yid && b === xid))
    ));
    const leftAnd = gateEntries.find(([_, { a, b, cond }]) => (
      cond === 'AND' && ((a === xid && b === yid) || (a === yid && b === xid))
    ));
    const zGate = gates.get(zid);

    if (!leftXor || !leftAnd || !zGate) continue;

    // In both a half adder and a full adder, the only gate that outputs to a z register is an XOR gate
    if (zGate.cond !== 'XOR') {
      knownWrong.push(zid);
    }

    // AND gates can only feed into the OR gate at the end of a full adder, except for the half adder in x00 AND y00 -> z00
    const andNext = gateEntries.find(([_, { a, b }]) => [a, b].includes(leftAnd[0]));
    if (!!andNext && andNext[1].cond !== 'OR' && i > 0) {
      knownWrong.push(leftAnd[0]);
    }

    // XOR gates can only output to to other XOR gates or an AND gate or a z register
    const xorNext = gateEntries.find(([_, { a, b }]) => [a, b].includes(leftXor[0]));
    if (!!xorNext && xorNext[1].cond === 'OR') {
      knownWrong.push(leftXor[0]);
    }
  }

  // All XOR gates either take in one of the x/y registers, or output to a z register
  for (const [c, { a, b, cond }] of gateEntries) {
    if (cond !== 'XOR') continue;
    const xIn = a.startsWith('x') || b.startsWith('x');
    const yIn = a.startsWith('y') || b.startsWith('y');
    const zOut = c.startsWith('z');
    if (!xIn && !yIn && !zOut) {
      knownWrong.push(c);
    }
  }

  knownWrong.sort();
  return knownWrong.join(',');
}

aoc({
  year: 2024,
  day: 24,
  part1,
  part1expected,
  part2,
  part2expected,
  parse,
});
