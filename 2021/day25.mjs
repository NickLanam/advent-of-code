(await import('./aoc.mjs')).default(
  2021, 25,
  (orig) => {
    let h = orig.length;
    let w = orig[0].length;
    let prev;
    let next = [...orig];
    let turns = 0;

    let LIM = 1_000;

    do {
      prev = [...next];
      const mid = [];
      // First, horizontal moves happen
      for (let r = 0; r < h; r++) {
        let line = '';
        for (let c = 0; c < w; c++) {
          const lv = prev[r][c < 1 ? w - 1 : c - 1];
          const mv = prev[r][c];
          const rv = prev[r][(c + 1) % w];
          if (lv === '>' && mv === '.') line += '>';
          else if (mv === '>' && rv === '.') line += '.';
          else line += mv;
        }
        mid.push(line);
      }

      // Then, vertical moves happen
      next = [];
      for (let r = 0; r < h; r++) {
        let line = '';
        for (let c = 0; c < w; c++) {
          const uv = mid[r < 1 ? h - 1 : r - 1][c];
          const mv = mid[r][c];
          const dv = mid[(r + 1) % h][c];
          if (uv === 'v' && mv === '.') line += 'v';
          else if (mv === 'v' && dv === '.') line += '.';
          else line += mv;
        }
        next.push(line);
      }
      turns++;
    } while(--LIM && prev.some((l, i) => next[i] !== l));
    return turns;
  }, 58,
  // The solution for star 50 is "collect all 49 other stars". That is, it's the Platinum Trophy.
  _ => 'Freebie', 'Freebie'
);