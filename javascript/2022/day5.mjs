(await import('./aoc.mjs')).default(
  2022, 5,
  ({ moves, stacks }) => {
    while (moves.length) {
      const move = moves.shift();
      for (let i = 0; i < move.count; i++) {
        const top = stacks[move.source - 1].shift();
        if (!top) throw new Error('Took too many');
        stacks[move.dest - 1].unshift(top);
      }
    };
    return stacks.map(s => s.shift()).join('');
  }, 'CMZ',
  ({ moves, stacks }) => {
    while (moves.length) {
      const move = moves.shift();
      const top = stacks[move.source - 1].slice(0, move.count);
      for (let i = 0; i < move.count; i++) stacks[move.source - 1].shift();
      stacks[move.dest - 1].unshift(...top)
    };
    return stacks.map(s => s.shift()).join('');
  }, 'MCD',
  (data) => {
    const moves = data.filter(line => line.startsWith('move')).map(line => {
      const [, count, source, dest] = line.match(/move (\d+) from (\d+) to (\d+)/);
      return { count: +count, source: +source, dest: +dest };
    });
    const empty = data.findIndex(line => !line);
    const stackRows = data.slice(0, empty - 1);
    const colCount = Math.ceil(stackRows[stackRows.length - 1].length / 4);

    const stacks = Array(colCount).fill(0).map(() => []);
    for (let r = 0; r < stackRows.length; r++) {
      const rowRaw = stackRows[r];
      for (let ch = 0; ch < colCount * 4; ch += 4) {
        const c = rowRaw.slice(ch + 1, ch + 2);
        if (c.trim()) stacks[Math.ceil(ch / 4)].push(c);
      }
    }
    return { moves, stacks };
  },
  false
);