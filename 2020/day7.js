const { getInput } = require('./utils');
const raw = getInput(7);

const dag = raw.map(l => {
  l = l.replace(/ bags?/g, '').replace(/.$/, '');
  [_, top, contents] = l.match(/^(.+) contain (.+)$/);
  contents = contents.split(', ') //.map(c => c.split(' ').slice(1).join(' '));
  return { top, contents };
}).reduce((a, c) => {
  a[c.top] = c.contents;
  return a;
}, {});

let deep = {...dag };
for (const [top, contents] of Object.entries(dag)) {
  let reachable = [...contents];
  for (let depth = 0; depth < 5; depth++) {
    for (const next of reachable) {
      reachable.push(...(dag[next.split(' ').slice(1).join(' ')] || []));
    }
    reachable = [...new Set(reachable)];
  }
  deep[top] = reachable;
}

function sumBags(outer) {
  const count = Number(outer.split(' ')[0].replace('no', '0'));
  const next = outer.split(' ').slice(1).join(' ');
  if (count === 0) {
    return 0; // have to count the outer bag too
  }
  const contents = dag[next];
  const result = contents.map(l => count * sumBags(l)).reduce((a, c) => a + c, count);
  return result;
}

const day7star1 = Object.values(deep).filter(v => v.some(c => c.endsWith('shiny gold'))).length;
const day7star2 = dag['shiny gold'].map(n => sumBags(n)).reduce((a, c) => a + c, 0);

console.log('Star 1: ', day7star1);
console.log('Star 2: ', day7star2);