import { sum } from './utils/array.mjs';

(await import('./aoc.mjs')).default(
  2023, 1,
  (data) => {
    return sum(data.map(l => (l.split('').filter(c => !Number.isNaN(+c))
    .reduce((_, __, ___, arr) => `${arr[0]}${arr[arr.length - 1]}`, ''))).map(c => +c));
  }, 209,
  (data) => {
    const words = ['zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine'];
    const nums = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    const numsOfLine = (line) => {
      const options = [];
      for (let i = 0; i < line.length; i++) {
        for (const w in words) {
          if (line.substr(i).startsWith(words[w])) {
            options.push(w);
            // Would skip overlaps (eightwo would be [8] rather than [8, 2]).
            // The correct answer expects that to come out as [8, 2], so overlap it is!
            // i += words[w].length - 1;
          }
        }
        for (const n in nums) {
          if (line.substr(i).startsWith(nums[n])) {
            options.push(n);
          }
        }
      }
      if (options.length === 0) throw new Error('ERROR no matches');
      // If there is only one match, this doubles it up.
      // The correct answer expects this to be how it's done.
      // Very literal interpretation of the spec!
      return +`${options[0]}${options[options.length - 1]}`;
    }
    return sum(data.map(line => numsOfLine(line)));
  }, 281,
  data => data
);
