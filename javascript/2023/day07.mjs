import aoc from './aoc.mjs';
import { sum } from './utils/array.mjs';

const part1expected = 6440;
const part2expected = 5905;

const CARD_VALUES = '123456789TJQKA'.split('');

const parse = (data) => (
  data.map(line => {
    let [original, stringBid] = line.split(' ');
    const cards = original.split('').map((v) => CARD_VALUES.indexOf(v) + 1);
    const bid = +stringBid;
    return { original, cards, bid };
  })
);

function rankHand({ cards }) {
  let mod = 0;

  const uniques = new Set(cards);

  const fiveOfAKind = uniques.size === 1;
  const twoTypes = uniques.size === 2; // Can be either four-of-a-kind or full house
  const threeTypes = uniques.size === 3; // Only way to have three unique cards
  const onePair = uniques.size === 4; // Only way to have four unique cards
  const highCard = uniques.size === 5; // Only way to have five unique cards

  const fullHouse = twoTypes && cards.some(c => cards.filter(c2 => c2 === c).length === 3);
  const fourOfAKind = twoTypes && !fullHouse;
  const threeOfAKind = threeTypes && cards.some(c => cards.filter(c2 => c2 === c).length === 3);
  const twoPair = threeTypes && !threeOfAKind;

  if (fiveOfAKind) {
    mod = 7;
  } else if (fourOfAKind) {
    mod = 6;
  } else if (fullHouse) {
    mod = 5;
  } else if (threeOfAKind) {
    mod = 4;
  } else if (twoPair) {
    mod = 3;
  } else if (onePair) {
    mod = 2;
  } else if (highCard) {
    mod = 1;
  } else {
    throw new Error(`Hand should have gotten SOMETHING, but did not: ${cards.join(', ')}`);
  }

  return mod;
}

/*
 * Useful input quirk: no straights to handle, and four-of-a-kind beats full house.
 * This makes figuring out what the jokers need to be a bit easier.
 */
function rankHandWithJokers(hand) {
  const { cards } = hand;

  const baseRank = rankHand(hand);
  const numJokers = cards.reduce((a, c) => a + (c === 1), 0);

  // High card but there's a joker -> it's now one pair
  if (baseRank === 1 && numJokers === 1) {
    return 2;
  }
  // One pair, and there's either one or two jokers -> it's now three-of-a-kind
  if (baseRank === 2 && numJokers >= 1) {
    return 4;
  }
  // Two pair, and one pair is jokers -> it's now four-of-a-kind
  if (baseRank === 3 && numJokers === 2) {
    return 6;
  }
  // Two pair, and neither is jokers but there IS a joker -> it's now full house
  if (baseRank === 3 && numJokers === 1) {
    return 5;
  }
  // Three of a kind, and a separate joker -> now four of a kind
  if (baseRank === 4 && numJokers === 1) {
    return 6;
  }
  // Three of a kind, and two separate jokers -> now five of a kind
  if (baseRank === 4 && numJokers === 2) {
    return 7;
  }
  // Three of a kind, and it's the jokers -> now either four or five depending on the other cards
  if (baseRank === 4 && numJokers === 3) {
    if ((new Set(cards)).size === 2) return 7;
    else return 6;
  }
  // Full house or better with jokers -> all of those become five of a kind
  if (baseRank >= 5 && numJokers > 0) {
    return 7;
  }
  // None of the above scenarios apply, probably there were no jokers
  return baseRank;
}

const part1 = (hands) => {
  const scored = hands.map(hand => ({
    ...hand,
    score: [rankHand(hand), ...hand.cards],
  }));
  scored.sort(({ score: a }, { score: b }) => {
    const comps = a.map((v, i) => Math.sign(v - b[i]));
    return comps.find(c => c !== 0);
  });
  const winnings = scored.map((s, i) => (i + 1) * s.bid);
  return sum(winnings);
};

const part2 = (hands) => {
  const jokered = hands.map(h => ({ ...h, cards: h.cards.map(c => c === 11 ? 1 : c) }));
  const scored = jokered.map(hand => ({
    ...hand,
    score: [rankHandWithJokers(hand), ...hand.cards],
  }));
  scored.sort(({ score: a }, { score: b }) => {
    const comps = a.map((v, i) => Math.sign(v - b[i]));
    return comps.find(c => c !== 0);
  });
  const winnings = scored.map((s, i) => (i + 1) * s.bid);
  return sum(winnings);
};

aoc(2023, 7, part1, part1expected, part2, part2expected, parse);
