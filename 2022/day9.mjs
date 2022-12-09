function adj(head, tail) {
  return (Math.abs(head[0] - tail[0]) <= 1) && (Math.abs(head[1] - tail[1]) <= 1);
}

function getMoveCloser(target, start) {
  if (adj(target, start)) return start;

  const xd = target[0] - start[0];
  const yd = target[1] - start[1];

  return [start[0] + Math.sign(xd), start[1] + Math.sign(yd)];
}

function solve(moves, numTails) {
  const headVisits = [[0, 0]];
  const tails = Array(numTails).fill(0).map(() => [[0, 0]]);
  for (const { dir, count } of moves) {
    for (let step = 0; step < count; step++) {
      const top = headVisits[headVisits.length - 1];
      let next;
      switch (dir) {
        case 'L': next = [top[0] - 1, top[1]]; break;
        case 'R': next = [top[0] + 1, top[1]]; break;
        case 'U': next = [top[0], top[1] - 1]; break;
        case 'D': next = [top[0], top[1] + 1]; break;
        default: throw new Error('wat');
      }
      headVisits.push(next);

      for (let t = 0; t < tails.length; t++) {
        let prev = (t === 0) ? headVisits : tails[t - 1];
        const follow = prev[prev.length - 1];
        let me = tails[t][tails[t].length - 1];
        while (!adj(follow, me)) {
          me = getMoveCloser(follow, me);
          tails[t].push(me);
        }
      }
    }
  }
  return (new Set(tails[tails.length - 1].map(d => JSON.stringify(d)))).size;
}

(await import('./aoc.mjs')).default(
  2022, 9,
  (moves) => solve(moves, 1), 88,
  (moves) => solve(moves, 9), 36,
  data => data.map(line => line.split(' ')).map(([d, c]) => ({ dir: d, count: +c }))
);