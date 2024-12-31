function majorityBit(index, lines) {
  const sum = lines.reduce((a, c) => (
    a + (((c & (2 ** index)) > 0) ? 1 : 0)
  ), 0);
  return sum / lines.length >= 0.5 ? 1 : 0; // Ones win in a tie
}

function getSignal(LEN, lines, cond) {
  let keep = [...lines], i = LEN;
  while (keep.length > 1 && (--i) >= 0) {
    const k = majorityBit(i, keep);
    keep = keep.filter(n => cond(n & (2 ** i)) ? 1 - k : k);
  }
  return keep[0];
}

(await import('./aoc.mjs')).default(
  2021, 3,
  (data) => {
    let gamma = 0, epsilon = 0;
    for (let i = 0; i < data.LEN; i++) {
      const m = majorityBit(i, data.lines);
      gamma += m << i;
      epsilon += (1 - m) << i;
    }
    return gamma * epsilon;
  }, 198,
  (data) => (
    getSignal(data.LEN, data.lines, n => n === 0)
    * getSignal(data.LEN, data.lines, n => n !== 0)
  ), 230,
  data => ({
    LEN: data[0].length,
    lines: data.map(l => parseInt(l, 2))
  })
);