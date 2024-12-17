import aoc from './aoc.mjs';

/** @typedef {string[]} ParsedInput */
/** @typedef {string} Part1Solution */
/** @typedef {number} Part2Solution */

/** @type Part1Solution */
const part1expected = '5,7,3,0';

/** @type Part2Solution */
const part2expected = 117440;

/**
 * @param {string[]} lines Unparsed input lines
 * @returns {ParsedInput}
 */
const parse = (lines) => {
  const A = +lines[0].match(/\d+$/)[0];
  const B = +lines[1].match(/\d+$/)[0];
  const C = +lines[2].match(/\d+$/)[0];
  const program = lines[4].substring('Program: '.length).split(',').map(n => +n);
  return { A, B, C, program };
};

/**
 * 
 * @param {ParsedInput} parsed 
 * @returns {Part1Solution}
 */
const samplePart1 = ({ A, B, C, program, expect }) => {
  let pc = 0;
  const COMBO = (arg) => [0, 1, 2, 3, A, B, C, 'RESERVED'][arg];

  let consoleOutput = [];
  while (pc <= program.length - 2) {
    let inst = program[pc];
    let arg = program[pc + 1];
    switch (inst) {
      case 0: { // adv
        A = A >> COMBO(arg);
        pc += 2;
        break;
      }
      case 1: { // bxl
        B = B ^ arg;
        pc += 2;
        break;
      }
      case 2: { // bst
        B = COMBO(arg) & 0b111;
        pc += 2;
        break;
      }
      case 3: { // jnz
        if (A === 0) {
          pc += 2;
        } else {
          pc = arg;
        }
        break;
      }
      case 4: { // bxc
        B = B ^ C;
        pc += 2;
        break;
      }
      case 5: { // out
        const out = COMBO(arg) & 0b111;
        if (expect && expect[consoleOutput.length] !== out) {
          // For part 2: we prune if we output the wrong thing ever
          return '';
        }
        consoleOutput.push(out);
        pc += 2;
        break;
      }
      case 6: { // bdv
        B = Math.floor(A / (2**COMBO(arg)));
        pc += 2;
        break;
      }
      case 7: { // cdv
        C = Math.floor(A / (2**COMBO(arg)));
        pc += 2;
        break;
      }
    }
  }

  return consoleOutput.join(',');
};

/**
 * This method will technically work on anyone's input, but too slow to be useful.
 * I kept it because the sample is a different program for which this returns in under 1ms.
 *
 * @param {ParsedInput} param0 
 * @returns {Part2Solution}
 */
const samplePart2 = ({ B, C, program }) => {
  const expected = program.join(',');
  for (let newA = 0; newA < Number.MAX_SAFE_INTEGER; newA++) {
    const o = samplePart1({ A: newA, B, C, program, expect: program });
    if (expected === o) {
      return newA;
    }
  }
  return '\x1b[31mFAIL\x1b[0m';
};

/**
 * Only works for my input, not the sample (two different programs).
 * 
 * @param {BigInt} inA 
 * @returns {Part1Solution}
 */
const fastPart1 = (inA) => {
  let A = inA;
  const out = [];
  while (true) {
    const b = A & 7n ^ 2n;
    out.push((b ^ (A >> b) ^ 3n) & 7n);
    A = A >> 3n;
    if (A === 0n) break;
  }
  return out.join(',');
};

/**
 * The idea is to work backwards using what I learned about a faster part 1 solution.
 * The function in the input just outputs a pseudo-random 3-bit window from A, then drops the last 3 bits.
 * It repeats until A=0.
 * So, we just do a depth-first search of possible 3-bit blocks to append until we get the program output,
 * and keep the one that had the lowest integer after adding all of those parts together.
 * All in all this takes under a millisecond.
 *
 * @param {ParsedInput} parsed 
 * @returns {Part2Solution}
 */
const fastPart2 = ({ B, C, program }, isSample) => {
  let bestAnswer = Infinity;
  const queue = [[program.length - 1, 0n]]; // [digit, A]
  while (queue.length) {
    const [e, A] = queue.pop();
    if (e < 0) {
      bestAnswer = Math.min(bestAnswer, Number(A));
      continue;
    }
    for (let i = 0n; i < 8n; i += 1n) {
      const a = (A * 8n) + i;
      const b = (a & 7n) ^ 2n;
      const n  = (b ^ (a >> b) ^ 3n) & 7n;
      if (n === BigInt(program[e])) {
        queue.push([e - 1, a]);
      }
    }
  }

  return bestAnswer;
};

aoc({
  year: 2024,
  day: 17,
  part1: (args, isSample) => isSample ? samplePart1(args) : fastPart1(BigInt(args.A)),
  part1expected,
  part2: (args, isSample) => isSample ? samplePart2(args) : fastPart2(args),
  part2expected,
  parse,
});
