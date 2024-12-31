const { getInput, fromRaw } = require('./utils');
const input = getInput(16, false, '\n\n');
const sample1 = fromRaw(`class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12`, false, '\n\n');

const sample2 = fromRaw(`class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9`, false, '\n\n');

const which = input;

let [fields, yours, nearby] = which;
fields = fields.split('\n')
  .map(l => createField(l));
yours = yours.split('\n')
  .slice(1)[0].split(',')
    .map(c => Number(c));
nearby = nearby.split('\n')
  .slice(1)
  .map(l => l.split(',')
    .map(c => Number(c)));

function createField(l) {
  const { name, a1, a2, b1, b2 } = l.match(/^(?<name>[^:]+): (?<a1>\d+)-(?<a2>\d+) or (?<b1>\d+)-(?<b2>\d+)/)
    .groups;
  const pass = (x) => (x >= a1 && x <= a2) || (x >= b1 && x <= b2);
  return { name, pass };
}

function test(fieldName, x) {
  const pass = fields.find(f => f.name === fieldName)[0].pass;
  return pass(x);
}

function passingFields(x) {
  const results = [];
  for (const f of fields) {
    if (f.pass(x)) results.push(f.name);
  }
  return results;
}

function testAnyPass(x) {
  for (const f of fields) {
    if (f.pass(x)) return true;
  }
  return false;
}

const day16star1 = (() =>  {
  let sumInvalid = 0;
  for (const t of nearby) {
    for (const n of t) {
      if (!testAnyPass(n)) {
        sumInvalid += n;
      }
    }
  }
  return sumInvalid;
})();

const day16star2 = (() => {
  const validTickets = [yours, ...nearby].filter(t => t.every(n => testAnyPass(n)));
  const allFieldNames = fields.map(f => f.name);
  const nameToPass = new Map(fields.map(f => [f.name, f.pass]));

  // First, figure out which indices are valid candidates for each field
  const possible = new Map();
  for (let i in yours) {
    const vals = validTickets.map(t => t[i]);
    for (const f of allFieldNames) {
      if (vals.every(v => nameToPass.get(f)(v))) {
        possible.set(f, (possible.get(f) || []).concat(Number(i)));
      }
    }
  }

  // Next, pick out fields for which only one index is valid (and not already assigned) until everything is assigned
  const byCandidateCount = [...possible.entries()].sort((a, b) => a[1].length - b[1].length);
  const assignments = Array(yours.length).fill(false);
  for (const [f, opts] of byCandidateCount) {
    const remain = opts.filter(o => !assignments[o]);
    if (remain.length !== 1) throw new Error('Multiple possibilities!');
    assignments[remain[0]] = f;
  }

  // Now we get the values on the ticket which match our criteria, and multiply them together
  const departureValues = yours.filter((_, i) => assignments[i].startsWith('departure'));
  return departureValues.reduce((a, c) => a*c, 1);
})();

console.log('Star 1: ', day16star1);
console.log('Star 2: ', day16star2)