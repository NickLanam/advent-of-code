const { getInput, fromRaw } = require('./utils');
const input = getInput(14);
// Should get 165 for part 1, part 2 irrelevant
const part1sample = fromRaw(`mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0`);
// Should get 51 for part 1 and 208 for part 2
const part2sample = fromRaw(`mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1`);
const which = input;

function applyMaskWithDontCares(mask, value) {
  const valBits = Number(value)
    .toString(2)
    .padStart(mask.length, '0')
    .split('');
  const result = [...valBits];
  const maskBits = mask.split('');
  for (let [i, bit] of maskBits.entries()) {
    if (bit !== 'X') result[i] = bit;
  }
  return parseInt(result.join(''), 2);
}

function applyMaskWithFloating(mask, value) {
  const valBits = Number(value)
    .toString(2)
    .padStart(mask.length, '0')
    .split('');
  const base = [...valBits];
  const maskBits = mask.split('');
  for (let [i, bit] of maskBits.entries()) {
    if (bit === 'X') base[i] = 'X';
    else if (bit === '1') base[i] = '1';
  }
  const numFloating = base.filter(c => c === 'X');
  const expectedCount = 2 ** numFloating.length;
  const results = [
    [...base].join(''),
  ];
  let expansionIndex = 0;
  let limit = 10000;
  while (expansionIndex < results.length && limit-- > 0) {
    const toExpand = results[expansionIndex];
    const hasFloatingBit = toExpand.indexOf('X') >= 0;
    if (hasFloatingBit) {
      // Note: not global regexes, so it only replaces one.
      const zero = toExpand.replace(/X/, '0');
      const one = toExpand.replace(/X/, '1');
      results.splice(expansionIndex, 1, zero, one);
      // Not incrementing the index, as what we just put there may still have more floaters to address.
    } else {
      expansionIndex++;
    }
  }
  return results;
}

const day14star1 = ((instructions) =>  {
  let mask;
  // No memory addresses in the input with 6+ digits, so 10k will be enough.
  const mem = Array(100000)
    .fill(0);
  for (const line of instructions) {
    const [instruction, arg] = line.split(' = ');
    if (instruction === 'mask') {
      mask = arg;
    } else {
      const [_, address] = instruction.match(/^mem\[(\d+)\]$/);
      mem[Number(address)] = applyMaskWithDontCares(mask, arg);
    }
  }
  return mem.reduce((a, c) => a + c, 0);
})([...which]);

const day14star2 = ((instructions) => {
  let mask;
  // There are large, sparse indices in the instructions; a map is more efficient here.
  const mem = {};
  for (const line of instructions) {
    const [instruction, arg] = line.split(' = ');
    if (instruction === 'mask') {
      mask = arg;
    } else {
      let [_, address] = instruction.match(/^mem\[(\d+)\]$/);
      const addresses = applyMaskWithFloating(mask, address);
      for (const a of addresses) {
        mem[parseInt(a, 2)] = parseInt(arg);
      }
    }
  }
  // The memory buffer is kinda huge
  return Object.values(mem)
    .reduce((a, c) => a + c, 0);
})([...which]);

console.log('Star 1: ', day14star1);
console.log('Star 2: ', day14star2)