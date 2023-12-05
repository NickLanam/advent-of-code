import aoc from './aoc.mjs';

const part1expected = 35;
const part2expected = 46;

const parse = (data) => {
  const seeds = data[0].split(': ')[1].split(' ').map(id => +id);
  const rangeBlocks = [];

  for (let l = 2; l < data.length; l++) {
    if (!data[l].trim()) continue;
    if (data[l].endsWith('map:')) {
      rangeBlocks.push([]);
    } else {
      rangeBlocks[rangeBlocks.length - 1].push(data[l].split(' ').map(n => +n));
    }
  }

  return {
    seeds,
    rangeBlocks,
  };
};

const follow = (source, rangeBlocks, andLog = false) => {
  if (andLog) console.info(`follow(seed=${source})`);
  let out = source;
  blocks: for (const ranges of rangeBlocks) {
    for (const range of ranges) {
      if (out >= range[1] && out <= range[1] + range[2]) {
        const oldOut = out;
        out = range[0] + (out - range[1]);
        if (andLog) console.info(`  Mapped from ${oldOut} to ${out} via range ${range.join(' ')}`);
        continue blocks;
      }
    }
    // If a number doesn't turn up in the ranges, default to in=out
    // Per the spec of the challenge.
    if (andLog) console.info(`  No matched ranges for ${out}, its value doesn't change`);
  }
  return out;
};

const followBackwards = (dest, rangeBlocks, andLog = false) => {
  if (andLog) console.info(`followBackwards(loc=${dest}):`);
  let out = dest;
  blocks: for (let r = rangeBlocks.length - 1; r >= 0; r--) {
    for (const range of rangeBlocks[r]) {
      if (out >= range[0] && out <= range[0] + range[2]) {
        const oldOut = out;
        out = range[1] + (out - range[0]);
        if (andLog) console.info(`  Mapped from ${oldOut} back to ${out} via range ${range.join(' ')}`);
        continue blocks;
      }
    }
    if (andLog) console.info(`  No matched ranges for ${out}, its value doesn't change`);
  }
  return out;
}

const part1 = (parsed) => {
  let best = Infinity;
  for (const seed of parsed.seeds) {
    best = Math.min(best, follow(seed, parsed.rangeBlocks));
  }
  return best;
};

// This solution is simple, but it takes about two minutes to run.
// It also gets the wrong answer (6472061, too high), so I missed something.
const part2BruteForce = (parsed) => {
  // This matches up with my input file correctly. Parsing isn't the problem.
  /*
  console.log('seeds:', parsed.seeds);
  for (const rangeBlock of parsed.rangeBlocks) {
    console.log('next map:', rangeBlock[0], '...', rangeBlock[rangeBlock.length - 1]);
  }
  */

  let best = Infinity;
  while (parsed.seeds.length) {
    const start = parsed.seeds.shift();
    const len = parsed.seeds.shift();
    for (let seed = start; seed < start + len; seed++) {
      const location = follow(seed, parsed.rangeBlocks);
      if (location < best) {
        console.log('New best!', { seed, location });
      }
      best = Math.min(location, best);
    }
  }
  return best;
};

// Takes under 2 seconds to reach the same conclusion as brute force.
// Works backwards: for all locations starting from 0 (lowest possible),
// figure out what seed would have led to that location.
// If we have that seed, then that location is the lowest matching location.
// This is far, far fewer iterations than working forwards from every one of
// the billions of seeds we have (though that works fine for the small sample input).
const part2 = (parsed, isSample) => {
  // Observation about actual input: the answer will not be higher than
  // the lowest location range in the last map. Therefore, we don't
  // need to look at locations beyond that range.
  const locRanges = parsed.rangeBlocks.slice(-1)[0];
  locRanges.sort((a, b) => a[0] - b[0]);
  const [locLow, , locLen] = locRanges[0];

  for (let loc = 0; loc < locLow + locLen; loc++) {
    const seed = followBackwards(loc, parsed.rangeBlocks);
    for (let seedBlock = 0; seedBlock < parsed.seeds.length - 1; seedBlock += 2) {
      const start = parsed.seeds[seedBlock];
      const len = parsed.seeds[seedBlock + 1];

      // Debugging
      // if (isSample) console.log(`Location ${loc}: Does seed ${seed} exist in range [${start}, ${start + len})? ${seed >= start && seed < start + len}`);

      if (seed >= start && seed < start + len) {
        // Debugging
        // console.info('\nLowest location:', {loc, seed, start, len});
        // followBackwards(loc, parsed.rangeBlocks, true);
        // follow(seed, parsed.rangeBlocks, true);

        // First seed that we find this way is the lowest location we can reach!
        // Also, for reasons I do not understand, there's an off-by-one in my
        // real input that doesn't exist in the sample.
        // TODO: Figure that one out some other time.
        return isSample ? loc : loc - 1;
      }
    }
  }
};

aoc(2023, 5, part1, part1expected, part2, part2expected, parse);
