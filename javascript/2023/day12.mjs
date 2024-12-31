import aoc from './aoc.mjs';

const part1expected = 21;
const part2expected = 525152;

const parse = (data, part) => data.map(line => {
  let [chars, arrangements] = line.split(' ');
  if (part === 1) {
    chars = (chars + '.').split('');
    arrangements = arrangements.split(',').map(n => +n);
  } else {
    chars = ((chars + '?').repeat(4) + chars + '.').split('');
    arrangements = String(arrangements + ',').repeat(5).split(',').map(n => +n).filter(n => n > 0);
  }
  return { chars, arrangements };
});

// Brute force solution, with no memoization or dynamic programming.
// This is decently fast for part 1, but part 2 has too huge a search space.
const solveSlowly = (lines) => {
  // Gnarly, but should do the trick for testing if a line fits a pattern.
  const fits = (fixedLine, arrangements) => {
    let found = [];
    let active = null;
    for (const c of fixedLine) {
      switch (c) {
        case '?': throw new Error('There was still an unknown when testing a line');
        case '#': {
          if (active != null) active++;
          else active = 1;
          break;
        }
        case '.': {
          if (active != null) {
            found.push(active);
            active = null;
          }
        }
      }
    }
    if (active != null) {
      found.push(active);
    }
    return found.length === arrangements.length && found.every((f, i) => f === arrangements[i]);
  };

  return lines.map(({ chars, arrangements }) => {
    // Naive slow approach: just try every combination.
    let matches = 0;
    const numUnknowns = chars.filter(c => c === '?').length;
    for (let combo = 0; combo < 2**numUnknowns; combo++) {
      let fixed = [...chars];
      let iu = -1;
      for (let c = 0; c < fixed.length; c++) {
        if (fixed[c] === '?') {
          iu++;
          fixed[c] = ((combo >> (numUnknowns - (iu) - 1)) & 0b1) === 0 ? '.' : '#';
        }
      }
      if (fits(fixed, arrangements)) matches++;
    }
    return matches;
  }).reduce((a, c) => a + c, 0);
};

// Dynamic programming, memoized recurrence relation. Haven't done this since college...
// This version explains what's going on, but the recursive rewrite below is easier to read.
// The order of operations is exactly the same; the difference is how the call stack is tracked.
const solveIteratively = (lines) => lines.reduce((sum, { chars, arrangements }) => {
  const memo = new Map();
  const stack = ['0/0/0'];

  const copyOrRetrieve = (sourceKey, destKey, plusScalar = 0) => {
    if (memo.has(sourceKey)) {
      stack.pop();
      memo.set(destKey, memo.get(sourceKey) + plusScalar);
    } else {
      stack.push(sourceKey);
    }
  };

  while (stack.length) {
    const key = stack[stack.length - 1];
    const [charIndex, count, arrangementIndex] = key.split('/').map(n => +n);

    const countUpKey = `${charIndex + 1}/${count + 1}/${arrangementIndex}`;
    const nextArrangementKey = `${charIndex + 1}/0/${arrangementIndex + 1}`;
    const retryArrangementKey = `${charIndex + 1}/0/${arrangementIndex}`;

    // No need to recalculate a known result.
    if (memo.has(key)) {
      stack.pop();
      continue;
    }

    if (charIndex === chars.length) {
      // Whole string has been read, this configuration works if it matched all arrangements (and no extras).
      const pass = arrangementIndex === arrangements.length;
      stack.pop();
      memo.set(key, pass ? 1 : 0);
    } else if (chars[charIndex] === '#') {
      copyOrRetrieve(countUpKey, key);
    } else if (chars[charIndex] === '.' || arrangementIndex === arrangements.length) {
      if (arrangementIndex < arrangements.length && count === arrangements[arrangementIndex]) {
        // We've found a group of broken springs and then a dot.
        // The pass/fail result will be the same as that of the next arrangement's pass/fail.
        // THIS ONE IS WHY WE ADD A SPARE DOT TO THE END OF THE INPUT IN parse()
        // (We add a ? instead of a dot for part 2 to find cases of overlap between copies)
        copyOrRetrieve(nextArrangementKey, key);
      } else if (count === 0) {
        // We failed to match an arrangement, so keep looking for it further down the string.
        copyOrRetrieve(retryArrangementKey, key);
      } else {
        // This configuration failed to match all arrangements, so it's not a viable config.
        stack.pop();
        memo.set(key, 0);
      }
    } else {
      if (memo.has(countUpKey)) {
        const numBrokenSprings = memo.get(countUpKey);
        if (count === arrangements[arrangementIndex]) {
          // If the rest of the string passes for remaining arrangements, so does this one.
          // Sum the number of passing arrangements so far together.
          copyOrRetrieve(nextArrangementKey, key, numBrokenSprings);
        } else if (count === 0) {
          // Same as above, but we just processed a dot.
          copyOrRetrieve(retryArrangementKey, key, numBrokenSprings);
        } else {
          // We're in the middle of a group, so whether this one passes depends on the rest of the string.
          stack.pop();
          memo.set(key, numBrokenSprings);
        }
      } else {
        // We don't know yet whether the rest of the string looks good, learn that first.
        stack.push(countUpKey);
      }
    }
  }

  // All results bubble up to this one.
  // Remember that we're doing all of this for every input line, then summing the results!
  return sum + memo.get('0/0/0');
}, 0);

// Same thing as above, but much easier to read and runs way faster because
// the native call stack is already optimized for this type of usage.
const solveRecursively = (lines) => lines.reduce((sum, { chars, arrangements }) => {
  const memo = new Map();
  const recurse = (charIndex, count, arrangementIndex) => {
    const key = `${charIndex}/${count}/${arrangementIndex}`;
    let out;
    if (memo.has(key)) {
      return memo.get(key);
    } else if (charIndex === chars.length) {
      out = arrangements.length === arrangementIndex ? 1 : 0;
    } else if (chars[charIndex] === '#') {
      out = recurse(charIndex + 1, count + 1, arrangementIndex); 
    } else if (chars[charIndex] === '.' || arrangementIndex === arrangements.length) {
      if (arrangementIndex < arrangements.length && count === arrangements[arrangementIndex]) {
        out = recurse(charIndex + 1, 0, arrangementIndex + 1);
      } else if (count === 0) {
        out = recurse(charIndex + 1, 0, arrangementIndex);
      } else {
        out = 0;
      }
    } else {
      const numHashes = recurse(charIndex + 1, count + 1, arrangementIndex);
      let numDots = 0;
      if (count === arrangements[arrangementIndex]) {
        numDots = recurse(charIndex + 1, 0, arrangementIndex + 1);
      } else if (count === 0) {
        numDots = recurse(charIndex + 1, 0, arrangementIndex);
      }
      out = numHashes + numDots;
    }
    memo.set(key, out);
    return out;
  };
  return sum + recurse(0, 0, 0);
}, 0);

aoc(2023, 12, solveRecursively, part1expected, solveRecursively, part2expected, parse);
