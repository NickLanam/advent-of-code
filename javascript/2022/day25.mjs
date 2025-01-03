function snafu2dec(snafu) {
  const ufans = [...snafu].reverse();
  let sum = 0;
  for (let i = 0; i < ufans.length; i++) {
    sum += { '=': -2, '-': -1, '0': 0, '1': 1, '2': 2 }[ufans[i]] * (5 ** i);
  }
  return sum;
}

function dec2snafu(dec) {
  let s = '';
  let rem = dec;
  while (rem > 0) {
    s = ['0', '1', '2', '=', '-'][rem % 5] + s;
    rem = Math.floor((rem + 2) / 5);
  }
  return s;
}

(await import('./aoc.mjs')).default(
  2022, 25,
  (snafus) => dec2snafu(snafus.reduce((a, c) => a + snafu2dec(c), 0)), '2=-1=0', // equals 4890 in decimal
  () => 'FREEBIE', 'FREEBIE',
  data => data.map(l => l.split(''))
);