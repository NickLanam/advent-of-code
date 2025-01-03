function runRound(monkeys, gcd, relief) {
  for (const monkey of monkeys) {
    while (monkey.items.length) {
      const item = monkey.items.shift();
      const operand = monkey.operand === 'old' ? item : BigInt(monkey.operand)
      let score = monkey.operator === '+' ? (item + operand + 0n) : (item * operand * 1n);

      // This is the magic that makes it work for part 2. This one line.
      // BigInt is still needed as the final score can be tens of billions, though.
      if (score > gcd) score = score % gcd;
      
      if (relief) {
        score = score / 3n;
      }

      if ((score % monkey.divTest) === 0n) {
        monkeys[monkey.trueThrow].items.push(score);
      } else {
        monkeys[monkey.falseThrow].items.push(score);
      }
      monkey.numInspections++;
    }
  }
  return monkeys;
}

function score(monkeys) {
  const scores = monkeys.map(monkey => monkey.numInspections);
  // BigInt isn't a valid return value for sort (has to be Number), so can't just do `b - a`.
  scores.sort((a, b) => {
    if (a > b) return -1;
    else if (a < b) return 1;
    else return 0;
  })
  return scores[0] * scores[1];
}

(await import('./aoc.mjs')).default(
  2022, 11,
  ({ monkeys, gcd }) => {
    for (let round = 0; round < 20; round++) {
      runRound(monkeys, gcd, true);
    }
    return score(monkeys);
  }, BigInt('10605'),
  ({ monkeys, gcd }) => {
    for (let round = 0; round < 10_000; round++) {
      runRound(monkeys, gcd, false);
    }
    return score(monkeys);
  }, BigInt('2713310158'),
  data => {
    const raws = data.join('\n').split('\n\n');
    const monkeys = raws.map((raw) => {
      const [idRaw, startRaw, opRaw, testRaw, trueRaw, falseRaw] = raw.split('\n');
      const id = Number(idRaw.split('Monkey ')[1].split(':')[0]);
      const items = startRaw.split(': ')[1].split(',').map(n => BigInt(n.trim()));
      let [,,,,operator, operand] = opRaw.trim().split(' ');
      const divTest = BigInt(testRaw.split('divisible by ')[1]);
      const trueThrow = Number(trueRaw.split('monkey ')[1]);
      const falseThrow = Number(falseRaw.split('monkey ')[1]);
      return { id, items, operator, operand, divTest, trueThrow, falseThrow, numInspections: 0n };
    });
    const gcd = monkeys.reduce((a, m) => a * m.divTest, 1n);
    return { monkeys, gcd };
  }
);