import { sum } from './utils/array.mjs';

(await import('./aoc.mjs')).default(
  2023, 4,
  cards => sum(cards.map(card => card.points)), 13,
  (cards) => {
    for (let i = cards.length - 1; i >= 0; i--) {
      const { numWins } = cards[i];
      cards[i].totalAdd = numWins + sum(cards
        .slice(i + 1, Math.min(i + 1 + numWins, cards.length - 1))
        .map(n => n.totalAdd));
    }

    return sum(cards.map(c => c.totalAdd)) + cards.length;
  }, 30,
  lines => lines.map(line => {
    const [winners, have] = line.split(':')[1]
      .split('|')
      .map(card => card.split(' ')
        .map(c => c.trim())
        .filter(c => c)
        .map(c => +c));

    let numWins = 0;
    let points = 0;
    for (const h of have) {
      if (winners.includes(h)) {
        numWins++;
        points = (points === 0 ? 1 : points * 2);
      }
    }
    return { numWins, points };
  })
);
