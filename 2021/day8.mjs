(await import('./aoc.mjs')).default(
  2021, 8,
  (lines) => {
    let found = [0, 0, 0, 0]; // Ones, fours, sevens, and eights (which have 2, 4, 3, and 7 segments lit respectively)
    for (const { values } of lines) {
      const search = [2, 4, 3, 7];
      for (let i in search) {
        found[i] += values.filter(v => v.length === search[i]).length;
      }
    }
    return found.reduce((a, c) => a + c, 0);
  }, 26,
  (lines) => {
    const sums = [];

    for (const { patterns, values } of lines) {
      // Start determining which pattern is which digit, beginning with the ones that have unique lengths.
      const maps = {
        0: '', // Ambiguous - 6 segments
        1: patterns.find(p => p.length === 2),
        2: '', // Ambiguous - 5 segments
        3: '', // Ambiguous - 5 segments
        4: patterns.find(p => p.length === 4),
        5: '', // Ambiguous - 5 segments
        6: '', // Ambiguous - 6 segments
        7: patterns.find(p => p.length === 3),
        8: patterns.find(p => p.length === 7),
        9: '', // Ambiguous - 6 segments
      };

      // To find the rest, we'll need to identify four of the seven segments.
      // We don't need to know top, mid, bot, or topLeft to finish this task.
      const poss = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

      // 0, 6, 9 have six segments. 9 and 0 have both segments that 1 does; 6 does not.
      maps[6] = patterns.find(p => p.length === 6 && !maps[1].split('').every(v => p.includes(v)));

      // Bottom right is set in every pattern EXCEPT 2.
      const botRight = poss.find(v => patterns.filter(p => !p.includes(v)).length === 1)
      maps[2] = patterns.find(p => !p.includes(botRight));

      // Top right is the only segment missing in 6.
      const topRight = poss.find(v => !maps[6].includes(v));

      // 2, 3, and 5 each have five segments. Among them, 5 doesn't have the top right segment.
      maps[5] = patterns.find(p => p.length === 5 && !p.includes(topRight));

      // We know 2 and 5 already, so 3 must be the only remaining five-segment pattern.
      maps[3] = patterns.find(p => p.length === 5 && p !== maps[2] && p !== maps[5]);

      // Bottom left exists in 2 and 0, but not 3 or 9. That lets us finish the map. 
      const botLeft = poss.find(v => maps[2].includes(v) && !maps[3].includes(v));
      maps[9] = patterns.find(p => p.length === 6 && !p.includes(botLeft));
      maps[0] = patterns.find(p => p.length === 6 && p !== maps[6] && p !== maps[9]);

      // Now we could find mid, top, bottom, and top left; but can already decode everything.
      // For each line we decode its values, then concatenate them and turn that into a number.
      let sum = '';
      for (const value of values) {
        const sanitized = value;
        for (let i = 0; i <= 9; i++) {
          if (maps[i] == sanitized) {
            sum += String(i);
            break;
          }
        }
      }
      sums.push(+sum);

    }
    // Final answer is the sum of each line's result.
    return sums.reduce((a, c) => a + c, 0);
  }, 61229,
  data => data.map(line => {
    let [patterns, values] = line.split(' | ').map(half => half.split(' ').map(pv => pv.split('').sort().join('')));
    return { patterns, values };
  })
);