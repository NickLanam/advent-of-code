const { getInput } = require('./utils');

const input = getInput(8);
const sample = `nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6`.split('\n');

const run = (instructions, s1 = true) => {
  // console.log(`>> RUN <<`);
  let pc = 0;
  let acc = 0;
  let sanityLimit = 100000;
  const seen = Array(instructions.length).fill(false);
  program: while (sanityLimit-- > 0) {
    if (seen[pc]) {
      return [false, acc];
    } else if (pc === instructions.length) {
      return [true, acc];
    } else if (pc > instructions.length) {
      throw new Error('jumped past halt point');
    }
    const [instruction, argString] = instructions[pc].split(' ');
    seen[pc] = true;
    const arg = Number(argString);
    // console.log(`pc=${pc}; acc=${acc}; instruction=${instruction}; arg=${arg}`);
    line: switch (instruction) {
      case 'acc':
        acc += arg;
        pc++;
        break line;
      case 'jmp':
        pc += arg;
        break line;
      case 'nop':
        pc++;
        break line;
    }
  }
  throw new Error('Reached sanity limit');
};

console.log('Star 1: ', run(input)[1]);

const switchIndices = input
  .map((_, i) => i)
  .filter(i => input[i].startsWith('nop') || input[i].startsWith('jmp'));
let found = false;
for (let i = -1; i < switchIndices.length; i++) {
  let instructions = [...input];
  if (i >= 0) {
    const inst = instructions[switchIndices[i]];
    instructions[switchIndices[i]] = inst.startsWith('nop') ?
      inst.replace('nop', 'jmp') :
      inst.replace('jmp', 'nop');
  }
  const [halts, acc] = run(instructions, false);
  if (halts) {
    console.log('Star 2: ', acc);
    found = true;
  }
}
if (!found) throw new Error('Failed to find a working change');