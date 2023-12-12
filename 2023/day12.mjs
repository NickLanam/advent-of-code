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

// Part 2 proves that this is way TOO slow
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

// Memoized recurrence relation. See Wikipedia for how these generally work.
// TODO: Turn this into an iterative function with a work queue to be a bit better on memory?
const solve = (lines) => lines.reduce((sum, { chars, arrangements }) => {
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

aoc(2023, 12, solve, part1expected, solve, part2expected, parse);
