/*

One data type: integer in [Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER].

Four registers: w, x, y, z. They hold integers, including negative values. Initialized to 0

One-off: no jmp instruction. Runs from top to bottom, once.

Six instructions. First arg is a register, second is either a register or a literal.
 inp r : READ a single integer 1-9 into register r
 add r s : r = r + s
 mul r s : r = r * s
 div r s : r = Math.floor(r / s) ; crash if s === 0. (negative values okay).
 mod r s : r = r % s ; crash if r < 0 || s <= 0; that is, don't bother dealing with negatives for mod.
 eql r s : r = +(r === s)

Example:
 inp x
 mul x -1
 > x = -1 * read()

Another:
 inp z
 inp x
 mul z 3
 eql z x
 > z = (read() * 3) === read();

THE SAMPLE INPUT:
 inp w
 add z w
 mod z 2
 div w 2
 add y w
 mod y 2
 div w 2
 add x w
 mod x 2
 div w 2
 mod w 2
 > w = read(); z = w % 2; w = Math.floor(w / 2); y = w % 2; w = Math.floor(w / 2); x = w % 2; w = Math.floor(w / 2); w = w % 2;
 > z = lowest bit; y = next bit; x = next bit; w = highest bit.
 > [w, x, y, z] = Number(read()).toString(2).split('').slice(-4);
 > This is a hint of what the real input does, but in a different base.

The first challenge is stripping all the junk from the input to find what it really does. Creating the ALU is a red herring.

Input to the ALU: fourteen values, all 1-9 (single digit no zero).

The final instruction puts a 0 in register Z if the input was "valid" (model number validated). Otherwise, it'll be non-0 (could even be negative).

Part 1 challenge: find the LARGEST 14-digit serial that gets a 0 in register Z at the end.
Part 2: SMALLEST value instead.

Now real input...

- Exactly 14 chunks of 18 instructions starting with `inp` and ending in `add z y`
- Lots of 26's in the arithmetic
- Hashed lines are the same in each chunk.

# inp w    <- w = read()
# mul x 0  <- x = (z % 26)
# add x z  <- ...
# mod x 26 <- ...
div z 1    <- Always a 1 if the next value is POSITIVE, always a 26 if the next value is NEGATIVE
add x 12   <- x += something within [-9, -1], or [9, 15]. Never within [0..8].
# eql x w  <- x = !(w === x)
# eql x 0  <- ...
# mul y 0  <- y = x ? 26 : 1
# add y 25 <- ...
# mul y x  <- ...
# add y 1  <- ...
# mul z y  <- z = z*y (z was divmodded by either 1 or 26 earlier)
# mul y 0  <- y = w + something within [1, 14] (specific to my input?)
# add y w  <- ...
add y 4    <- ...
# mul y x  <- y = y * x
# add z y  <- z = z + y

We see only three lines change, and the first of them depends on the second.
With that in mind, turning it into code goes like this:

---
w = read()
x = z % 26
if (xs is negative) {
  z = floor(z/26)
}
x += xs
x = !(x == w)
y = (25 * x) + 1
z = z * y
y = w + ys
y = y * x
z += y

---
w = read()
prev = z % 26
if (xs is negative) {
  z = floor(z/26)
}
if ((prev + xs) !== w) {
  z = (z * 26) + w + ys
}

---
const xs = [12, 11, 14, -6, 15, 12, -9, 14, 14, -5, -9, -5, -2, -7]; // Changes for each person
const ys = [ 4, 10, 12, 14,  6, 16,  1,  7,  8, 11,  8,  3,  1,  8]; // Changes for each person

const stack = [];
for (const i in digits) {
  const w = digits[i]
  const prev = stack[-1]
  if (xs[i] < 0) {
    stack.pop();
  }
  if ((prev ?? 0) + xs[i] !== w) {
    stack.push(w + ys[i]);
  }
}
return sum(stack) === 0 ? 'valid' : 'not valid';


---------------------------------------
Simplify even further in plain english:

Stack starts empty.

On each input:
- If xScalars[i] < 0, pop.
- If digits[i] !== (top element before popping) + xScalars[i],
  push digits[i] + yScalars[i]
  When xScalars[i] >= 9, this condition is always met because digits range [1..9].

What this means:
- We can "afford" to push at most as many digits as there are negative scalars for X.
- X scalars > 8 guarantee a push.
- So, #negatives-#>8 = "budget"
- X scalars in [0..8]: only as many as budget may fail; any more and the check can't be valid.
- In my input (guessing the rule is the same for others?), [0..8] doesn't occur.
- Therefore, we need to fail the push condition every time X is negative or the whole thing fails.

-------------------------------------
MANUAL CHECK AT EACH DIGIT GOES LIKE:

We have seven negative X scalars, but also seven that are >= 9.
Therefore, the budget is actually ZERO.
ZERO digits can be allowed to push where X < 9.
- X < 9 on the same seven where it's negative. It's never in [0..8] in my input. Convenient.


const xs = [12, 11, 14, -6, 15, 12, -9, 14, 14, -5, -9, -5, -2, -7]; // Changes for each person
const ys = [ 4, 10, 12, 14,  6, 16,  1,  7,  8, 11,  8,  3,  1,  8]; // Changes for each person

One digit at a time...
- Digit 1 [12,  4] -> PUSH [d1, 5..13] -> [[d1, 5..13]]
- Digit 2 [11, 10] -> PUSH [d2, 11..19] -> [[d1, 5..13],[d2, 11..19]]
- Digit 3 [14, 12] -> PUSH [d3, 13..21] -> [[d1, 5..13],[d2, 11..19],[d3, 13..21]]
- Digit 4 [-6, 14] -> POP [d3, 13..21] -> [[d1, 5..13],[d2, 11..19]]
  - [d3, 13..21] - 6 = [7..9]; Digit3 had to be [1..3]
  - Digit4 = Digit3 + 6
  - Digit3 must be [1..3]
  -> What we popped was pushed by digit 3. 9-xs = max value of digit 3. -1*xs = the rule (digit3 + 6 = digit4). This can be coded!!!
  - Also, the upper bound being pushed is irrelevant. We just need to push (digit it came from).
  - Rules: currentDigit = digitFrom - xs (xs is negative); digitFromMin = 1; digitFromMax = 9 + xs
  - NOTICE: we don't even use ys. That ... is likely wrong, but the math does make it fall out actually.
- Digit 5 [15,  6] -> PUSH [d5, 7..15] -> [[d1, 5..13],[d2, 11..19],[d5, 7..15]]
- Digit 6 [12, 16] -> PUSH [d6, 17..25] -> [[d1, 5..13],[d2, 11..19],[d5, 7..15],[d6, 17..25]]
- Digit 7 [-9,  1] -> POP [d6, 17..25] -> [[d1, 5..13],[d2, 11..19],[d5, 7..15]]
  - [d6, 17..25] - 9 = [8..9]; Digit6 had to be [1..2]
  - Digit7 = Digit6 + 7
  > We totally needed the prev min to subtract xs from. prevY (16) - curX (9) = 7; 9 - 7 = 2 = max val of prev.
  - And the constant to add to Digit7 is prevY - |curX| = 9 - max of Digit6.
- Digit 8 [14,  7] -> PUSH [d8, 8..16] -> [[d1, 5..13],[d2, 11..19],[d5, 7..15],[d8, 8..16]]
- Digit 9 [14,  8] -> PUSH [d9, 9..17] -> [[d1, 5..13],[d2, 11..19],[d5, 7..15],[d8, 9..17],[d9, 9..17]]
- Digit 10 [-5, 11] -> POP [d9, 9..17] -> [[d1, 5..13],[d2, 11..19],[d5, 7..15],[d8, 8..16]]
  - [d9, 9..17] - 5 = [4..9]; Digit 9 had to be [1..6]
  - Digit10 = Digit9 + 3
- Digit 11 [-9,  8] -> POP [d8, 8..16] -> [[d1, 5..13],[d2, 11..19],[d5, 7..15]]
  - [d8, 8..16] - 9 = [1..7]; Digit8 had to be [3..9]
  - Digit11 = Digit8 - 2
  > offset here is NEGATIVE. And maxVal of Digit8 is 9 but its minVal is 3. Hrm.

Putting those rules together:
- Digit4 = Digit3 + 6
  - Digit3 in [1..3]
- Digit7 = Digit6 + 7
  - Digit6 in [1..2]
- Digit10 = Digit9 + 3
  - Digit9 in [1..6]


So code that answers this question could do this:

- First, extract xs, ys from the input. Zip them together.
- Create a stack that is Array<number>, where the number is the digit ID
- Create a set of rules[]
- Loop d = 0..14
  - const prevIndex = stack.peek();
  - if (xs[d] < 0) stack.pop();
    - rules.maxValues[prevIndex] = Math.min(rules.maxValues[prevIndex], 9 + xs[d]);
    - rules.addition[d] = [prevIndex, Math.abs(xs[d])];
  - else
    - stack.push(d)
- If stack has length, we screwed up. Throw an error.
- Now rules give us two kinds of constraints:
  - Some constraints set the max value of a digit
  - Some constraints set the value of a digit to a different digit, plus a constant (9 minus that constant defines the first rule)
  - Those rules can be derived from each other, we _technically_ only need to remember the latter.
- So to get the minimum 'legal' value:
  - 1s for digits that don't have a rule.
  - If the digit has the second rule type, then it's 1 + the constant
  - If the digit has the first rule type, it's just 1
- To get the maximum 'legal' value:
  - 9s for digits that don't have a rule
  - If the digit has the second rule type, it's just 9
  - If the digit has the first rule type, it's 9 - the constant
*/

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

      relRules.set(d, { dPrev, offset });
      if (offset >= 0) {
        maxRules.set(dPrev, Math.min(maxRules.get(dPrev) ?? Infinity, 9 - offset));
      } else {
        minRules.set(dPrev, Math.max(minRules.get(dPrev) ?? -Infinity, Math.abs(offset) + 1 ));
      }
    } else {
      stack.push(d);
    }
  }

  if (stack.length) {
    throw new Error('ERR: Stack was not empty at the end. Likely made a mistake with rule implementation.');
  }
  if ((maxRules.size + minRules.size) !== 7 || relRules.size !== 7) {
    throw new Error(`ERR: Imbalanced: Got ${maxRules.size} upper bound constraints, ${minRules.size} lower, and ${relRules.size} relation constraints. Both should be 7.`);
  }

  // For my input, this is what happens:
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

// This time around, there was no sample to use. So I just put my real input there after solving by hand, and tested that this code gets the same result.
// I should grab someone else's input and check that I get the right answers for them, too, to verify my assumptions.
(await import('./aoc.mjs')).default(
  2021, 24,
  digitOffsets => rulesToMinMax(buildRules(digitOffsets)).max, 91398299697996,
  digitOffsets => rulesToMinMax(buildRules(digitOffsets)).min, 41171183141291,
  // Input is 14 sets of instructions, always the same except a number on lines 5 and 15 (zero-indexed).
  data => data.join('\n').split('inp').filter(l=>l).map(l => l.split('\n').filter((_,i) => i === 5 || i === 15).map(ins => +ins.match(/([-\d]+)$/)[1])),
);