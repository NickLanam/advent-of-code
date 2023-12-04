import { sum } from './utils/array.mjs';

(await import('./aoc.mjs')).default(
  2023, 4,
  (data) => {
    let points = 0;
    for (const line of data) {
      const [winners, have] = line.split(':')[1].split('|').map(card => card.split(' ').map(c => c.trim()).filter(c => c).map(c => +c));
      let cp = 0;
      for (const h of have) {
        if (winners.includes(h)) {
          if (cp === 0) cp = 1;
          else cp *= 2;
        }
      }
      points += cp;
    }
    return points;
  }, 13,
  (data) => {
    let originalCards = [];
    for (const line of data) {
      const cardId = line.match(/^Card\s+(\d+):/)[1];
      const [winners, have] = line.split(':')[1].split('|').map(card => card.split(' ').map(c => c.trim()).filter(c => c).map(c => +c));
      const numWins = sum(have.map(h => +winners.includes(h)));
      originalCards.push({ id: +cardId, numWins });
    }

    // Each card always adds the same number of cards.
    // Don't need to actually run the simulation to compute the answer.
    for (let i = originalCards.length - 1; i >= 0; i--) {
      const { numWins } = originalCards[i];
      originalCards[i].totalAdd = numWins + sum(originalCards.slice(i + 1, Math.min(i + 1 + numWins, originalCards.length - 1)).map(n => n.totalAdd));
    }

    // Remember, have to count every single card in the pile.
    // That means the original cards themselves, plus the entire explosion
    // that each one adds.
    return sum(originalCards.map(c => c.totalAdd)) + originalCards.length;
  }, 30,
  data => data
);
