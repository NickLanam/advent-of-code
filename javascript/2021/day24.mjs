/*
 * Creating the ALU is a red herring.
 * Input to the ALU: fourteen values, all 1-9 (single digit no zero).
 *
 * The final instruction puts a 0 in register Z if the input was "valid" (model number validated). Otherwise, it'll be non-0 (could even be negative).
 * 
 * Part 1 challenge: find the LARGEST 14-digit model number that gets a 0 in register Z at the end.
 * Part 2: SMALLEST value instead.
 * 
 * The trick (and the reason the ALU is a red herring) is the pattern in the input program...
 * - Exactly 14 chunks of 18 instructions starting with `inp a` and ending in `add a b`
 * - Only the arguments to three lines differ between chunks:
 *   - Line 5 (0-indexed): an integer in [-25?, -1] or [9, 25?]. Never within [0..8].
 *   - Line 15: another integer in [1, 25?].
 *   - Line 4: A 1 if line 5 was positive; a 26 if it was negative.
 * - Lots of 26's in the arithmetic, and modulo / intdiv on 26.
 *
 * The variable names and line order are likely shuffled between different inputs, but the code ultimately does the same thing.
 * Here is what the first chunk of my input looks like (lines preceded with # are the same in every chunk):
 *
 * # inp w    <- w = read a single digit, 1-9 (never 0)
 * # mul x 0  <- x = (z % 26)
 * # add x z  <- ...
 * # mod x 26 <- ...
 *   div z 1  <- Always a 1 if the next value is POSITIVE, always a 26 if the next value is NEGATIVE
 *   add x 12 <- x += something within [-25?, -1], or [9, 25?]. Never within [0..8].
 * # eql x w  <- x = !(w === x)
 * # eql x 0  <- ...
 * # mul y 0  <- y = x ? 26 : 1
 * # add y 25 <- ...
 * # mul y x  <- ...
 * # add y 1  <- ...
 * # mul z y  <- z = z*y (z was divmodded by either 1 or 26 earlier)
 * # mul y 0  <- y = w + something within [1, 14?]
 * # add y w  <- ...
 *   add y 4  <- ...
 * # mul y x  <- y = y * x
 * # add z y  <- z = z + y
 *
 * With a bit of hand-decompilation, the code is actually doing this:
 *
 * ------
 *
 * // These are my unique values; they change for each person.
 * const xs = [12, 11, 14, -6, 15, 12, -9, 14, 14, -5, -9, -5, -2, -7];
 * const ys = [ 4, 10, 12, 14,  6, 16,  1,  7,  8, 11,  8,  3,  1,  8];
 *
 * // Assembly version uses a base 26 number, using div and mod to treat it like a stack.
 * const stack = [];
 * for (const i in digits) {
 *   const w = digits[i]
 *   const prev = stack[-1]
 *   if (xs[i] < 0) {
 *     stack.pop();
 *   }
 *   if ((prev ?? 0) + xs[i] !== w) {
 *     stack.push(w + ys[i]);
 *   }
 * }
 * return stack.length === 0 ? 'valid' : 'not valid';
 *
 * ------
 *
 * In plain English:
 * 
 * Stack starts empty.
 * 
 * On each input:
 * - If xs[i] < 0, pop.
 * - If digits[i] !== (top element before popping) + xs[i],
 *   push digits[i] + ys[i]
 *   When xs[i] >= 9, this condition is always met because digits range [1..9].
 * 
 * There are some noteworthy properties on the inputs that make this a lot more clear:
 * - xs are never in [0..8]. A negative value pops off the stack; a value >= 9 pushes.
 *   A value in [0..8] would potentially do both. Not having those present is suspect.
 * - Neither xs nor ys go outside of [-16..16]. Since digits are 1-9, adding/subtract
 *   will never result in a value over 25 in any digit of the output. Thus, base 26!
 * - There are 7 xs values less than 0, and 7 greater than 8. Perfectly balanced.
 * 
 * Extrapolating further with that code and those restrictions on the input:
 * - MONAD is a checksum algorithm.
 * - It uses a stack to add and remove pending work. An empty stack at the end means a valid model#.
 * - In order for a model number to pass, every digit where xs > 8 _must_ fail to push to the stack.
 * - With this knowledge, we can start to create some rules based entirely on the input xs and ys.
*/

// Note: We only need to retain the relative rules, the min/max rules could be computed from them.
// It would be less code, but harder to read that way.
function buildRules(digitOffsets) {
  const stack = [];
  const maxRules = new Map();
  const minRules = new Map();
  const relRules = new Map();

  // When testing with someone else's input, these will tell me if I made assumptions that only work on my input.
  if (digitOffsets.length !== 14) {
    throw new Error(`Got a set of ${digitOffsets.length} digit offsets, should be exactly 14 pairs.`);
  }
  if (digitOffsets[0][0] < 0) {
    throw new Error('This setup does not handle the first X rule being negative');
  }
  if (digitOffsets.some(([xs]) => xs >= 0 && xs <= 8)) {
    throw new Error('x offsets should be outside of [0, 8], lest they create an imbalanced special case');
  }
  if (digitOffsets.some(([,ys]) => ys < 1 || ys > 25)) {
    throw new Error('y offsets less than 1 or greater than 25 would create invalid offsets or leak across entries in the stack');
  }

  for (let d = 0; d < 14; d++) {
    const [xs] = digitOffsets[d];
    if (xs < 0 && stack.length) {
      const dPrev = stack.pop();
      const yPrev = digitOffsets[dPrev][1];
      const offset = yPrev - Math.abs(xs);

      // Relative rule: a digit whose value is some earlier digit plus or minus an offset.
      relRules.set(d, { dPrev, offset });
      if (offset >= 0) {
        // Said earlier digit can take on values from [1..(9 - offset)] if the offset is positive.
        maxRules.set(dPrev, Math.min(maxRules.get(dPrev) ?? Infinity, 9 - offset));
      } else {
        // If the offset is negative, said earlier digit ranges instead from [(|offset|+1)..9].
        minRules.set(dPrev, Math.max(minRules.get(dPrev) ?? -Infinity, Math.abs(offset) + 1 ));
      }
    } else {
      stack.push(d);
    }
  }

  if (stack.length) {
    throw new Error('Stack was not empty at the end. Likely made a mistake with rule implementation.');
  }
  if ((maxRules.size + minRules.size) !== 7 || relRules.size !== 7) {
    // Given the assumptions made above, we SHOULD end up with exactly half of the digits being relative to the rest,
    // and the rest having EITHER a minumum or a maximum value (the other defaulting to 1 or 9 respectively).
    throw new Error(`Imbalanced: Got ${maxRules.size} upper bound constraints, ${minRules.size} lower, and ${relRules.size} relation constraints.`);
  }

  // For my input, this is what happens.
  // I wrote the code AFTER doing this by hand and verifying the answers.
  // - Digit 0 has minVal 4, no maxVal -> [4, 9]
  // - Digit 1 has maxVal 1, no minVal -> [1, 1]
  // - Digit 2 has maxVal 3, no minVal -> [1, 3]
  // - Digit 3 = digit2 + 6 =             [7, 9]
  // - Digit 4 has maxVal 8, no minVal -> [1, 8]
  // - Digit 5 has maxVal 2, no minVal -> [1, 2]
  // - Digit 6 = digit5 + 7 =             [8, 9]
  // - Digit 7 has minVal 3, no maxVal -> [3, 9]
  // - Digit 8 has maxVal 6, no minVal -> [1, 6]
  // - Digit 9 = digit8 + 3 =             [4, 9]
  // - Digit 10 = digit7 - 2 =            [1, 7]
  // - Digit 11 = digit4 + 1 =            [2, 9]
  // - Digit 12 = digit1 + 8 =            [9, 9]
  // - Digit 13 = digit0 - 3 =            [1, 6]
  return { maxRules, minRules, relRules, stack };
}

function rulesToMinMax({ maxRules, minRules, relRules }) {
  let minOut = [];
  let maxOut = [];

  for (let d = 0; d < 14; d++) {
    if (relRules.has(d)) {
      const { dPrev, offset } = relRules.get(d);
      if (minOut.length < dPrev || maxOut.length < dPrev) throw new Error('Rules are out of order');
      maxOut.push(maxOut[dPrev] + offset);
      minOut.push(minOut[dPrev] + offset);
    } else if (maxRules.has(d)) {
      maxOut.push(maxRules.get(d));
      minOut.push(1);
    } else if (minRules.has(d)) {
      minOut.push(minRules.get(d));
      maxOut.push(9);
    } else {
      throw new Error(`There was no rule for digit ${d}`);
    }
  }

  return { min: +minOut.join(''), max: +maxOut.join('') };
}

// This time around, there was no sample input. Since I checked my answers by hand before coding,
// my "sample" is my real input.
(await import('./aoc.mjs')).default(
  2021, 24,
  digitOffsets => rulesToMinMax(buildRules(digitOffsets)).max, 91398299697996,
  digitOffsets => rulesToMinMax(buildRules(digitOffsets)).min, 41171183141291,
  // Input is 14 sets of instructions, always the same except a number on lines 5 and 15 (zero-indexed). Line 4 is determiend by line 5.
  data => data.join('\n').split('inp').filter(l=>l).map(l => l.split('\n').filter((_,i) => i === 5 || i === 15).map(ins => +ins.match(/([-\d]+)$/)[1])),
);