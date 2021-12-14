/**
 * Rather than trying to keep track of a string that grows 50% larger at every step...
 * - Begin knowing how many matches to each rule there were in the initial string.
 * - For each rule, find the two rules that would trigger in its stead when applying it.
 * - Keep track of how many times each rule would be triggered this way.
 * - Repeat this process at every iteration
 * - We can't reconstruct the final string this way, but that's not what we're tracking.
 */
function apply(rules, prev) {
  const next = { ...prev };
  for (const k of Object.keys(next)) next[k] = 0;

  for (const k of Object.keys(next)) {
    const add = rules[k];
    if (!add) throw new Error('Missing a rule here. Need extra logic for it. ' + k);
    const left = k.substr(0, 1) + add;
    const right = add + k.substr(1, 1);
    next[left] += prev[k];
    next[right] += prev[k];
  }
  return next;
}

/**
 * From the number of times each pair exists, we can find the number of times each letter exists.
 * This is as simple as the sum of the count of each rule that it appears in, divided by two, then rounded up if not whole.
 * The request is to then return (# most frequent letter) - (# least frequent number).
 * 
 * Why this works:
 * - String 'ABACACDCD'
 * - Pairs: { AB: 1, BA: 1, AC: 2, CA: 1, CD: 2, DC: 1 }
 * - Letters: { A: 3, B: 1, C: 3, D: 2 }
 * - A = (AB + BA + AC + CA = 1 + 1 + 2 + 1 = 5), /2 = 2.5, round up = 3
 * - B = BA / 2 = 0.5, round up = 1
 * - C = (AC + CA + CD + CD = 2 + 1 + 2 + 1 = 6), /2 = 3, round up = 3
 * - D = (CD + DC = 3), /2 = 1.5, round up = 2
 */
function score(pairings) {
  const counts = {};
  for (const [pair, count] of Object.entries(pairings)) {
    const [left, right] = pair.split('');
    counts[left] = (counts[left] ?? 0) + count;
    counts[right] = (counts[right] ?? 0) + count;
  }
  for (const k of Object.keys(counts)) counts[k] = Math.ceil(counts[k] / 2);
  
  const freq = Object.keys(counts).sort((a, b) => counts[b] - counts[a]);
  return counts[freq[0]] - counts[freq[freq.length - 1]];
}

function solve({ template, rules }, numIterations) {
  // Need to compute what pairings we started with to proceed.
  let pairings = {};
  for (const k of Object.keys(rules)) {
    pairings[k] = 0;
    for (let i = 0; i < template.length - 1; i++) {
      if (template.substr(i, 2) === k) pairings[k]++;
    }
  }

  for (let iter = 0; iter < numIterations; iter++) {
    pairings = apply(rules, pairings);
  }

  return score(pairings);
}

(await import('./aoc.mjs')).default(
  2021, 14,
  data => solve(data, 10), 1588,
  data => solve(data, 40), 2188189693529,
  lines => ({
    template: lines[0],
    rules: lines.slice(2).map(line => line.split(' -> ')).reduce((out, [a, b]) => ({...out, [a]: b }), {}),
  })
);