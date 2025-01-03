# AdventOfCode-2017
My solutions to the 2017 [Advent of Code](http://adventofcode.com/2017) challenges, dumped into one list with explanations.

Javascript solutions are executed in the browser console of the webpage that opens when you ask Advent of Code for your puzzle input. For challenges that don't have this, there's a variable to dump your input value(s).
05AB1E solutions are executed in tio.run, with the input being pasted there.

## Day 1
### Part 1

```javascript
document.body.innerText.replace(/[^\d]+/g,'').split('').filter((d,i,a)=>d==a[(i+1)%a.length]).reduce((t,v)=>t+parseInt(v),0)
```

```05AB1E
1ôDÀ->ΘÏ1ôO
```

Takes the text from the page, keeps only the numbers, splits into an array, keeps only the items where the next one in the list (wrapping from end to front if needed) is the same, and sums what remains.

### Part 2

```javascript
document.body.innerText.replace(/[^\d]+/g,'').split('').filter((d,i,a)=>d==a[(i+a.length/2)%a.length]).reduce((t,v)=>t+parseInt(v),0)
```

Same trick, but look ahead by half of the length of the list, rather than looking one element ahead.

## Day 2
### Part 1

```javascript
document.body.innerText.split('\n').filter(l=>!!l).map(line=>line.split(/\s+/g).reduce((t,v)=>{t.max=Math.max(t.max,v);t.min=Math.min(t.min,v);return t;},{min:Number.POSITIVE_INFINITY,max:Number.NEGATIVE_INFINITY})).map(t=>t.max-t.min).reduce((t,v)=>t+v,0)
````

Turn the input into a 2D array (a matrix). Then transform each row into a {min, max} pair, transform that into max-min, and finally sum all of these values.

### Part 2

```javascript
document.body.innerText.split('\n').filter(l=>!!l).map(line=>line.trim().split(/\s+/g).map(v=>parseInt(v))).map(l=>l.sort((a,b)=>b-a)).map(l=>{ let d=0.5; for(let i=0;i<l.length-1&&Math.floor(d)!==d;i++){ for(let j=i+1;j<l.length&&Math.floor(d)!==d;j++) { d=l[i]/l[j]; } } return d; }).reduce((t,v)=>t+v,0);
````

Turn the input into a 2D array (a matrix). Clean it up and turn each row into a sorted list of numbers. On each row, iterate from largest to smallest. Divide that number by each of the later numbers in the list. Repeat this until an even division is discovered. Now that each "row" is just the first even division result, sum them up.

## Day 3
### Part 1
```javascript
manhattan=(target)=>{
  // The pattern goes: right once, up once, left twice, down twice, right three times, up three times, left four times, down four times... and so on
  // We can do some clever math with this, leveraging modular arithmetic and summation formulae.
  // First, we take the input minus one. This is how many cells to spiral. Next, we compute how far in each direction it would have to go with the above observations.
  // Then, we let left/right and up/down motion cancel out, add the absolute value of each direction, and that is our manhattan distance!
  // But before all that, let's cheat and count the slow way.
  let cell=1, step=1, dir=0, dist=[0,0,0,0]; // [right, up, left, down]
  while(cell < target) {
    let diff = Math.min(step, target-cell);
    dist[dir] += diff;
    cell += diff;
    diff = Math.min(step, target-cell);
    cell += diff;
    dir = (dir+1)%4;
    dist[dir]+=diff;
    dir = (dir+1)%4;
    step += 1;
  }
  return Math.abs(dist[0]-dist[2])+Math.abs(dist[1]-dist[3]);
};
console.log(manhattan(INPUT_NUMBER_HERE));
```

### Part 2:

```javascript
spiralFill=(target)=>{
  // The pattern goes: right once, up once, left twice, down twice, right three times, up three times, left four times, down four times... and so on
  // We can do some clever math with this, leveraging modular arithmetic and summation formulae. 
  // Each cell is the sum of its neighbors, locking once set, starting from 1 and spiraling outwards as described above.
  // The neighbors include diagonals. As such, a cell can see at most four values: the one computed before it, and up to three earlier values.
  // To make searching and storage easier, we'll keep a linear array where each "cell" knows its coordinates.
  // The final output is the first cell larger than the target value when doing this.
  // There is surely a non-brute-force method that leverages clever math based on the above observations...
  let cells=[], stepSize=1, value=1, x=0, y=0, dir=0, step=1;
  while(value<=target) {
    cells.push({value,x,y});
    x += dir%2===0? [1,-1][dir/2] : 0;
    y += (dir-1)%2===0? [-1,1][(dir-1)/2] : 0;
    step--;
    if (step === 0) {
      dir = (dir+1)%4;
      if(dir===2||dir===0) stepSize++;
      step=stepSize;
    }
    let coords = [ [x-1, y], [x-1, y-1], [x, y-1], [x+1, y-1], [x+1, y], [x+1, y+1], [x, y+1], [x-1, y+1] ];
    value = cells.filter(cell => coords.find(coord => cell.x==coord[0]&&cell.y==coord[1])).reduce((t,v)=>t+v.value,0);
  }
  return value;
};
console.log(spiralFill(INPUT_NUMBER_HERE));
```

## Day 4

### Part 1

```javascript
document.body.innerText.trim().split('\n').map(l=>l.trim().split(' ')).filter(phrase => phrase.map((w,i) => phrase.lastIndexOf(w)==i).reduce((t,v)=>t&&v,true)).length;
````

Split into lines, split lines into words, transform into a true/false value (false if any word occurs twice in the line), return how many are true.

### Part 2
```javascript
document.body.innerText.trim().split('\n').map(l=>l.trim().split(' ')).filter(phrase => phrase.map(word=>word.split('').sort().join('')).map((w,i,a) => a.lastIndexOf(w)==i).reduce((t,v)=>t&&v,true)).length
```

Same deal, but also alphabetically sort each word in each phrase before checking for duplicates.

## Day 5
### Part 1

```javascript
jumps=document.body.innerText.trim().split('\n').map(Number);index=0;steps=0;while(index<jumps.length&&index>=0) {jumps[index]++;index += jumps[index]-1;steps++;}console.log(steps);
```

Pretty self-explanatory. Direct implementation of the requirements.

### Part 2

```javascript
jumps=document.body.innerText.trim().split('\n').map(Number);index=0;steps=0;while(index<jumps.length&&index>=0) {m=jumps[index];jumps[index]=m<3?m+1:m-1;index+=m;steps++;}console.log(steps);
```

Same trick, but with the extra logic for increment/decrement in it.

## Day 6
### Part 1

```javascript
let layouts = [['your','input','numbers','here']];
let steps = 0;

let noDuplicates = layout => {
  for(let r = 0; r < layouts.length; r++) {
    let dup = true;
    for(let c = 0; c < layouts[r].length; c++) {
      dup = dup && layout[c] === layouts[r][c];
    }
    if (dup) return false;
  }
  return true;
  // Copying arrays like this works but is too slow on large inputs. Thus we do it the old fashioned way.
  //let noDuplicates = layout=>!layouts.map(c=>c.map((v,i)=>v===layout[i]).reduce((t,v)=>t&&v)).reduce((t,v)=>t&&v);
}

while(true) {
  let next=Array.apply(Array,layouts[layouts.length-1]);
  let m=Math.max.apply(null, next);
  let i=next.indexOf(m);
  next[i]=0;
  while(m>0) { next[(++i)%next.length]++; m--; }
  steps++;
  if(noDuplicates(next)) { layouts.push(next); }
  else { console.log(`GOT IT, it took ${steps} steps to find a cycle.`); break; }
  if(steps > 100000) {console.log('FAILED TO FIND A CYCLE AFTER 100,000 ITERATIONS, QUITTING.');break;}
}
console.log(steps);
```

### Part 2

```javascript
let layouts = [['your','input','values','here']];
let steps = 0;

let hasDuplicates = layout => {
  for(let r = 0; r < layouts.length; r++) {
    let dup = true;
    for(let c = 0; c < layouts[r].length; c++) {
      dup = dup && layout[c] === layouts[r][c];
    }
    if (dup) return r;
  }
  return false;
  // Copying arrays like this works but is too slow on large inputs. Thus we do it the old fashioned way.
  //let noDuplicates = layout=>!layouts.map(c=>c.map((v,i)=>v===layout[i]).reduce((t,v)=>t&&v)).reduce((t,v)=>t&&v);
}

while(true) {
  let next=Array.apply(Array,layouts[layouts.length-1]);
  let m=Math.max.apply(null, next);
  let i=next.indexOf(m);
  next[i]=0;
  while(m>0) { next[(++i)%next.length]++; m--; }
  steps++;
  let iter = hasDuplicates(next);
  if(iter === false) { layouts.push(next); }
  else { console.log(`Found a duplicate of row ${layouts.length} on row ${iter}, so the cycle has length ${layouts.length-iter}.`); break; }
  if(steps > 100000) {console.log('FAILED TO FIND A CYCLE AFTER 100,000 ITERATIONS, QUITTING.');break;}
}
console.log(steps);
```

## Day 7
### Part 1

```javascript
all=document.body.innerText.trim().split('\n') .map(t => /^([a-z]+)\s\(\d+\)( -> ([a-z, ]+))?$/.exec(t.trim())) .map(l => [l[1],...(l[3]||"").split(', ')]); held = all.reduce((t,v) => [...t,...v.slice(1)], []).sort().filter((n,i,a)=>!!n&&a.lastIndexOf(n)===i); console.log(all.filter(r=>held.indexOf(r[0])===-1).map(root=>root[0]));
```

### Part 2

```javascript
let tree = {};

document.body.innerText.trim().split('\n')
  .map(t => /^([a-z]+)\s\((\d+)\)( -> ([a-z, ]+))?$/.exec(t.trim()))
  .map(l => [l[1],Number(l[2]),...(l[4]||"").split(', ')])
  .forEach(n => tree[n[0]] = { weight:n[1],children:n.slice(2).filter(c=>!!c) });

let keys = Object.keys(tree);
let topKey = keys.filter(k => keys.filter(j => tree[j].children.indexOf(k) !== -1).length === 0)[0];
compose(topKey);
findImbalance();

function compose(k) {
  if (tree[k] && tree[k].children.length > 0) {
    tree[k].children.forEach(compose);
  }

  tree[k].sum = tree[k].weight + tree[k].children.map(j => tree[j].sum).reduce((t,v)=>t+v,0);
  tree[k].balanced = [...new Set(tree[k].children.map(j => tree[j].sum))].length <= 1;
  tree[k].output = `${tree[k].balanced?'\u2713':'\u2717'} ${k} @ ${tree[k].weight} -> ${tree[k].sum}`;
  let childOutputs = tree[k].children.map(j => tree[j].output.split('\n').map(l=>`  ${l}`).join('\n')).join('\n');
  if (childOutputs) {
    tree[k].output += '\n' + childOutputs;
  }
}

function findImbalance() {
  let inspect = topKey;
  while (!tree[inspect].balanced && tree[inspect].children.some(c=>!tree[c].balanced)) {
    inspect = tree[inspect].children.filter(c=>!tree[c].balanced)[0];
  }

  let goodValue = tree[inspect].children.map(c=>tree[c].sum).filter((v,i,a) => a.indexOf(v)!==a.lastIndexOf(v))[0];
  let badValue  = tree[inspect].children.map(c=>tree[c].sum).filter((v,i,a) => a.indexOf(v)===a.lastIndexOf(v))[0];
  let weight = tree[inspect].children.map(c=>tree[c]).filter(n=>n.sum===badValue)[0].weight;
  console.log(weight + (goodValue-badValue));
}
```

## Day 8
### Part 1

```javascript
let registers=document.body.innerText.trim().split('\n').reduce((t,v) => {
  let [,reg,incdec,delta,check,cmp,test] = /^([a-zA-Z]+) (inc|dec) ([-\d]+) if ([a-zA-Z]+) ([!<>=]+) ([-\d]+)$/.exec(v.trim());
  let cv = Number(t[check]||0), tv=Number(test);
  if(incdec === 'dec') delta = 0-Number(delta);
  if ( (cv>tv&&cmp=='>')||(cv<tv&&cmp=='<')||(cv==tv&&cmp=='==')||(cv!=tv&&cmp=='!=')||(cv<=tv&&cmp=='<=')||(cv>=tv&&cmp=='>=')) { t[reg]=Number(t[reg]||0)+Number(delta); }
  return t;
}, {});
Object.keys(registers).reduce((max,key)=>Math.max(max,registers[key]),Number.NEGATIVE_INFINITY)
```

Almost verbatim implementation of the requirements to calculate the value of every register, then a reduce to find the largest one's value.

### Part 2

```javascript
document.body.innerText.trim().split('\n').reduce((t,v) => {
  let [,reg,incdec,delta,check,cmp,test] = /^([a-zA-Z]+) (inc|dec) ([-\d]+) if ([a-zA-Z]+) ([!<>=]+) ([-\d]+)$/.exec(v.trim());
  let cv = Number(t[check]||0), tv=Number(test);
  if(incdec === 'dec') delta = 0-Number(delta);
  if ( (cv>tv&&cmp=='>')||(cv<tv&&cmp=='<')||(cv==tv&&cmp=='==')||(cv!=tv&&cmp=='!=')||(cv<=tv&&cmp=='<=')||(cv>=tv&&cmp=='>=')) { t[reg]=Number(t[reg]||0)+Number(delta); if (t[reg] > t._max) {t._max=t[reg]}; }
  return t;
}, {_max:Number.NEGATIVE_INFINITY})._max
```

## Day 9
### Part 1

```javascript
document.body.innerText.trim().replace(/!./g,'').replace(/<[^>]*>/g,'').replace(/,/g,'').split('').reduce((t,v) => ({n:t.n+(v=='{'?1:-1),a:t.a+(v=='{'?0:t.n)}), {n:0,a:0}).a
```

Remove the negated characters, remove the garbage, remove the commas, and start processing the string that is now composed entirely of `{` and `}`, from left to right. At each `{`, increment the nesting counter (starting at 0). At each `}`, add the nesting counter to the total (starting at 0) and then decrement the nesting counter.

### Part 2

```javascript
document.body.innerText.trim().replace(/!./g,'').replace(/<[^>]*>/g,g=>g.length-2).split(/[^\d]+/).filter(d=>!!d).reduce((t,v)=>t+Number(v),0);
```

Much simpler: remove the negated characters, replace the garbage with its length (not counting the `<` and `>`), sum the numbers in what remains.

## Day 10
###Part 1

```javascript
input=document.body.innerText.trim().split(',').map(Number);
c=0; s=0; ring=Array(256).fill(0).map((v,i)=>i);
while(input.length>0) {
  let l=input.shift();
  let bigRing = [...ring,...ring,...ring];
  let replacement = bigRing.slice(c,c+l).reverse();
  bigRing.splice(c,l,...replacement);
  bigRing.splice(c+256,l,...replacement);
  ring=bigRing.slice(256,512);
  c=(c+l+s)%256;
  s++;
}
console.log(ring[0]*ring[1]);
```

Make a ring [0,...,255]. Then, when doing the steps of the challenge, operate on a triplicated version of that ring. This makes wrapping easier. Taking the middle of the triplicate restores the right ring.

### Part 2

```javascript
input=document.body.innerText.trim().split('').map(c=>c.charCodeAt(0)).concat([17, 31, 73, 47, 23]);
c=0; s=0; ring=Array(256).fill(0).map((v,i)=>i);
round=input=>{
while(input.length>0) {
  let l=input.shift();
  let bigRing = [...ring,...ring,...ring];
  let replacement = bigRing.slice(c,c+l).reverse();
  bigRing.splice(c,l,...replacement);
  bigRing.splice(c+256,l,...replacement);
  ring=bigRing.slice(256,512);
  c=(c+l+s)%256;
  s++;
}
};
for(let r=0;r<64;r++) { round(input.slice(0)); }
dense = '';
for(let x=0;x<16;x++) { dense += ('0'+ring.slice(x*16,x*16+16).reduce((t,v)=>t^Number(v),0x00).toString(16)).slice(-2); }
console.log(dense);
```

First, transform the input into a sequence of character codes and tack on the predefined values from the challenge.
Then, run 256 rounds of part 1's algorithm to get the final ring.
Then take each group of 16 values, xor them together, and concatenate their two-digit hexadecimal representations.

## Day 11
### Part 1
```javascript
steps = document.body.innerText.trim().split(',').reduce((t,v)=>{t[v]++;return t;}, {n:0,ne:0,se:0,s:0,sw:0,nw:0}); dirs=['n','ne','se','s','sw','nw']; oppose = {n_s:Math.min(steps.n,steps.s), ne_sw:Math.min(steps.ne,steps.sw), nw_se:Math.min(steps.nw,steps.se)}; steps.n -= oppose.n_s; steps.s -= oppose.n_s; steps.ne -= oppose.ne_sw; steps.sw -= oppose.ne_sw; steps.nw -= oppose.nw_se; steps.se -= oppose.nw_se; middle=dirs.indexOf(dirs.filter((k,i,a)=>steps[k]>0&&steps[a[(i+5)%6]]>0&&steps[a[(i+1)%6]]>0)[0]); diff = Math.min(steps[dirs[(middle+5)%6]],steps[dirs[(middle+1)%6]]); steps[dirs[(middle+5)%6]]-=diff; steps[dirs[(middle+1)%6]]-=diff; steps[dirs[middle]]+=diff; console.log(dirs.reduce((t,k)=>t+steps[k],0));
```

### Part 2

```javascript
route = document.body.innerText.trim().split(',');
dirs=['n','ne','se','s','sw','nw'];

simplify=(given) => { let steps=Object.assign({},given); let oppose = {n_s:Math.min(steps.n,steps.s), ne_sw:Math.min(steps.ne,steps.sw), nw_se:Math.min(steps.nw,steps.se)}; steps.n -= oppose.n_s; steps.s -= oppose.n_s; steps.ne -= oppose.ne_sw; steps.sw -= oppose.ne_sw; steps.nw -= oppose.nw_se; steps.se -= oppose.nw_se; let middle=dirs.indexOf(dirs.filter((k,i,a)=>steps[k]>0&&steps[a[(i+5)%6]]>0&&steps[a[(i+1)%6]]>0)[0]); if(middle>0) {let diff = Math.min(steps[dirs[(middle+5)%6]],steps[dirs[(middle+1)%6]]); steps[dirs[(middle+5)%6]]-=diff; steps[dirs[(middle+1)%6]]-=diff; steps[dirs[middle]]+=diff;} return steps; };

distance=(given) => { let steps=simplify(given); return dirs.reduce((t,k)=>t+steps[k],0); };

currentSteps = { n:0, ne:0, se:0, s:0, sw:0, nw:0 };
maxDistance = 0;
route.forEach(step => { currentSteps[step]++; currentSteps = simplify(currentSteps); maxDistance = Math.max(maxDistance, distance(currentSteps)); });
console.log(currentSteps, maxDistance);
```

## Day 12
### Part 1

```javascript
links=document.body.innerText.trim().split('\n').map(l=>l.match(/^\d+ <-> ([\d, ]+)$/)[1].split(/[^\d]+/).map(Number));
visited=Array(links.length).fill(false);
visit=index=>{
  let reachable = [];
  links[index].forEach(l => { reachable.push(l); if(!visited[l]) {visited[l]=true; reachable = reachable.concat(visit(l)); } });
  reachable = [...new Set(reachable)];
  links[index] = reachable;
  return reachable;
};
visit(0);
console.log(links[0].length);
```

This is a common graph traversal problem with a common solution.

### Part 2

```javascript
links=document.body.innerText.trim().split('\n').map(l=>l.match(/^\d+ <-> ([\d, ]+)$/)[1].split(/[^\d]+/).map(Number));
visited=Array(links.length).fill(false);
visit=index=>{
  let reachable = [];
  links[index].forEach(l => { reachable.push(l); if(!visited[l]) {visited[l]=true; reachable = reachable.concat(visit(l)); } });
  reachable = [...new Set(reachable)];
  links[index] = reachable;
  return reachable;
};
let numGroups = 0;
let limit=100000;
while(visited.indexOf(false) !== -1 && --limit>0) {
  visit(visited.indexOf(false));
  numGroups++;
}
console.log(numGroups);
```

Similar trick, but making sure to visit every node and tracking how many times we have to manually restart.

## Day 13
### Part 1

```javascript
document.body.innerText.trim().split('\n').map(l => l.split(': ').map(Number)).map(l => l[0]%(2*(l[1]-1)) ? 0 : l[0]*l[1]).reduce((t,v)=>t+v,0)
```

Pretty straightforward with modulus arithmetic. Each layer has a cycle (it hits the "catch" line at the 0 picoseconds, then each 2*(depth-1) picoseconds thereafter). Mapping each layer to zero if it isn't on the catch line at (depth) picoseconds, or its score otherwise, then summing the results gets a working answer.

### Part 2

```javascript
m=document.body.innerText.trim().split('\n')
    .map(l => l.split(': ').map(Number));
console.log(Array(10000000).fill(0)
  .map((_,d) => m.map(l => (l[0]+d)%(2*(l[1]-1)) === 0)
    .reduce((t,v)=>t||v,false))
  .indexOf(false));
```

For each delay from 0 to a sane limit, use the technique above to detect if it was caught (no need to compute the score). Find the first one that doesn't get caught, even by index 0.

## Day 14
### Part 1

```javascript
function knot(str) {
  let input = str.split('').map(c => c.charCodeAt(0)).concat([17, 31, 73, 47, 23]);
  let c = 0,
    s = 0,
    ring = Array(256).fill(0).map((v,i)=>i);
  let round = input => {
    while(input.length>0) {
      let l = input.shift();
      let bigRing = [...ring,...ring,...ring];
      let replacement = bigRing.slice(c,c+l).reverse();
      bigRing.splice(c,l,...replacement);
      bigRing.splice(c+256,l,...replacement);
      ring = bigRing.slice(256,512);
      c = (c+l+s)%256;
      s++;
    }
  };

  for(let r = 0; r < 64; r++) {
    round(input.slice(0));
  }

  let dense = '';
  for(let x = 0; x < 16; x++) {
    dense += ring.slice(x*16,x*16+16).reduce((t,v)=>t^Number(v),0).toString(2).padStart(8,'0');
  }
  return dense;
}

console.log(Array(128).fill(0).map((_,i) => knot(`YOUR_INPUT_VALUE-${i}`).split('').filter(c => c==='1').length).reduce((t,v)=>t+v,0));
```

Recycles Day 10, Part 2, returning the hash in binary instead of hexadecimal. It then uses this to generate the entire grid, and count the ones.

### Part 2

```javascript
function knot(str) {
  let input = str.split('').map(c => c.charCodeAt(0)).concat([17, 31, 73, 47, 23]);
  let c = 0,
    s = 0,
    ring = Array(256).fill(0).map((v,i)=>i);
  let round = input => {
    while(input.length>0) {
      let l = input.shift();
      let bigRing = [...ring,...ring,...ring];
      let replacement = bigRing.slice(c,c+l).reverse();
      bigRing.splice(c,l,...replacement);
      bigRing.splice(c+256,l,...replacement);
      ring = bigRing.slice(256,512);
      c = (c+l+s)%256;
      s++;
    }
  };

  for(let r = 0; r < 64; r++) {
    round(input.slice(0));
  }

  let dense = '';
  for(let x = 0; x < 16; x++) {
    dense += ring.slice(x*16,x*16+16).reduce((t,v)=>t^Number(v),0).toString(2).padStart(8,'0');
  }
  return dense;
}

function tagReachableGroups(grid) {
  let tagged = Array(128).fill(0).map((_,r)=>Array(128).fill(0).map((_,c)=>({set: grid[r][c], group: false})));
  let groupId = 0;

  let tagFill = (row, col) => {
    if (row >= 0 && row < 128 && col >= 0 && col < 128 && tagged[row][col].set && tagged[row][col].group === false) {
      tagged[row][col].group = groupId;
      tagFill(row-1, col);
      tagFill(row+1, col);
      tagFill(row, col-1);
      tagFill(row, col+1);
    }
  };

  while(true) {
    let entryRow = tagged.findIndex(row => row.some(v => v.set && v.group === false));
    if (entryRow === -1) break; // Found them all already
    let entryCol = tagged[entryRow].findIndex(cell => cell.set && cell.group === false);
    groupId++;
    tagFill(entryRow, entryCol);
  }

  return groupId;
}

let grid = Array(128).fill(0).map((_,i) => knot(`uugsqrei-${i}`).split('').map(v=>v==='1'));
let groups = tagReachableGroups(grid);
console.log(groups);
```

After generating the grid, use the same idea as Day 12, Part 2 in order to count distinct groups.

## Day 15
### Part 1

```javascript
function* gen(seed,factor){let v=seed; while(true){ v=(v*factor)%2147483647; yield v; }};
a = gen(INPUT_A, 16807);
b = gen(INPUT_B, 48271);
matches = 0;
for(let test=0; test < 40000000; test++) {
  matches += (a.next().value&0xFFFF)===(b.next().value&0xFFFF) ? 1 : 0;
}
console.log(matches);
```

A good excuse to use generator functions and bit mashing. Even though the input is on its own page for this one, it's two numbers, so no point parsing that out.

### Part 2

```javascript
function* gen(seed,factor,mod){let v=seed; while(true){ do { v=(v*factor)%2147483647; } while(v%mod !== 0); yield v; }};
a = gen(INPUT_A, 16807, 4);
b = gen(INPUT_B, 48271, 8);
matches = 0;
for(let test=0; test < 5; test++) {
  matches += (a.next().value&0xFFFF)===(b.next().value&0xFFFF) ? 1 : 0;
}
console.log(matches);
```

Only need to change the generator to keep going until it finds a suitable value before yielding, rather than yielding every time.

## Day 16
### Part 1

```javascript
document.body.innerText.trim().split(',').reduce(
  (line, command) => {
    command = [command.slice(0,1), command.slice(1)];
    line = line.split('');
    if (command[0] === 's') {
      let rot = Number(command[1]);
      for (;rot>0;rot--) {
        line.unshift(line.pop());
      }
    } else if (command[0] === 'x') {
      let [_, i1, i2] = /^(\d+)\/(\d+)$/.exec(command[1]);
      let tmp = line[i1];
      line[i1] = line[i2];
      line[i2] = tmp;
    } else {
      let [_, n1, n2] = /^([a-z])\/([a-z])$/.exec(command[1]);
      let i1 = line.indexOf(n1), i2 = line.indexOf(n2);
      line[i1] = n2;
      line[i2] = n1;
    }
    return line.join('');
  },
  'abcdefghijklmnop'
);
```

Split the commands into a list, then perform them in sequence to mutate the line.

### Part 2

```javascript
// Since we're doing this one billion times, we want to minimize duplicate computation as much as possible.
let commands = document.body.innerText.trim().split(',').map(command => {
  command = [command.slice(0, 1), command.slice(1)];
  if (command[0] === 's') {
    return function(line) {
      for (let rot = Number(command[1]); rot > 0; rot--) {
        line.unshift(line.pop());
      }
      return line;
    };
  } else if (command[0] === 'x') {
    let [_, i1, i2] = /^(\d+)\/(\d+)$/.exec(command[1]);
    return function(line) {
      let tmp = line[i1];
      line[i1] = line[i2];
      line[i2] = tmp;
      return line;
    };
  } else {
    let [_, n1, n2] = /^([a-z])\/([a-z])$/.exec(command[1]);
    return function(line) {
      let i1 = line.indexOf(n1), i2 = line.indexOf(n2);
      line[i1] = n2;
      line[i2] = n1;
      return line;
    };
  }
});
let line = 'abcdefghijklmnop'.split('');
for(let group=0;group<100000;group++) {
  console.log(`After ${group*100000}, we're at ${line.join('')}`);
  for (let iter = 0; iter < 10000; iter++) {
    line = commands.reduce(
      (line, command) => {
        return command(line);
      },
      line
    );
  }
}
console.log(line.join(''));
```

One billion iterations is a LOT for string manipulation in JS. Spitting out the progress every 100k will start showing patterns.

In my case, I found that there was a cycle every 300,000 iterations. 1bil%300k=100k, so the result at 100k iterations is the answer.

## Day 17
### Part 1

```javascript
let jump = YOUR_PUZZLE_INPUT; // Example uses 3
let stop = 2017;
let buffer = ['0'];
let current = 0;
for (let tag = 1; tag <= stop; tag++) {
  current = ((current + jump) % buffer.length) + 1;
  buffer = [...buffer.slice(0, current), String(tag), ...buffer.slice(current)];
}

console.log(buffer[(buffer.indexOf(String(stop))+1)%buffer.length]);
```

A better way to state the problem's algorithm: start with a ring that only has '0'. Step forward YOUR_PUZZLE_INPUT times. Insert the value '1 in front of you. Step forward YOUR_PUZZLE_INPUT times. Insert the value '2' in front of you. Repeat until you insert '2017'. Return the value that is now two steps in front of you.

In code, use the modulus operator to simulate the ring. Javascript's `splice` could be used in place of the spread concatenation syntax, this is just easier to understand at first glance than considering index arithmetic.

### Part 2

```javascript
let jump = YOUR_PUZZLE_INPUT;
let stop = 50000000;
let valueAfterZero = -1;
let current = 0;
for (let tag = 1; tag <= stop; tag++) {
  current = ((current + jump) % tag) + 1;
  if (current === 1) {
    valueAfterZero = tag;
  }
  if (tag % 100000 === 0) { console.log(`At ${tag}`); }
}

console.log(valueAfterZero);
```

Using an array to run this algorithm out to 50 million steps would take too long. Implementing a linked list to do it would be faster, but use plenty of memory.

The value we care about is the one immediately after the '0'. Since that's a constant position, we only care about insertions that would go there. As such, we don't need to generate the ring, we only need to check for insertions that would take over the position after '0', and track that value.

## Day 18
### Part 1

```javascript
let program = document.body.innerText.trim().split('\n');
let registers = {freq: 0};
let pc = 0;
let limit = 10000; // Just in case
run: while (pc >= 0 && pc < program.length && --limit >= 0) {
  let [_, instruction, lval, rval] = /([a-z]{3}) ([a-z\d-]+) ?([a-z\d-]+)?/.exec(program[pc]);
  rval = !isNaN(+rval) ? +rval : registers[rval] || 0;
  switch (instruction) {
    case 'snd': {
      registers.freq = !isNaN(+lval) ? +lval : registers[lval] || 0;
      break;
    }
    case 'set': {
      registers[lval] = rval;
      break;
    }
    case 'add': {
      registers[lval] = (registers[lval] || 0) + rval;
      break;
    }
    case 'mul': {
      registers[lval] = (registers[lval] || 0) * rval;
      break;
    }
    case 'mod': {
      registers[lval] = (registers[lval] || 0) % rval;
      break;
    }
    case 'rcv': {
      if (registers[lval]) {
        registers[lval] = registers.freq;
        console.log(`Recovered ${registers.freq}`);
        break run;
      }
      break;
    }
    case 'jgz': {
      if (registers[lval]) {
        pc = pc - 1 + rval;
      }
      break;
    }
    default: break;
  }
  pc += 1;
}
```

A trivial assembly interpreter that halts when the program counter goes out of bounds, and when the challenge answer is found (the first rcv instruction that actually runs).

### Part 2

```javascript
let reg = [
  {p: 0, pc: 0, snd: [], sent: 0, rcv: false, iter: 0},
  {p: 1, pc: 0, snd: [], sent: 0, rcv: false, iter: 0}
];
let pid = false; // false=0, true=1, cheap trick for other places to toggle.

let commands = {
  opSnd(lval) {
    reg[+pid].snd.push(reg[+pid][lval]);
    reg[+pid].sent++;
    reg[+pid].pc += 1;
  },

  opRcv(lval) {
    let o = +!pid;
    if (reg[o].snd.length > 0) {
      reg[+pid][lval] = reg[o].snd.shift();
      reg[+pid].rcv = false;
      reg[+pid].pc += 1;
      pid = o;
    } else {
      reg[+pid].rcv = true;
      pid = o;
    }
  },

  opJgz(lval, rval) {
    if ((!isNaN(lval) && +lval > 0) || reg[+pid][lval] > 0) {
      reg[+pid].pc = reg[+pid].pc + (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    } else {
      reg[+pid].pc += 1;
    }
  },

  opSet(lval, rval) {
    reg[+pid][lval] = !isNaN(rval) ? +rval : reg[+pid][rval] || 0;
    reg[+pid].pc += 1;
  },

  opAdd(lval, rval) {
    reg[+pid][lval] = (reg[+pid][lval] || 0) + (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    reg[+pid].pc += 1;
  },

  opMul(lval, rval) {
    reg[+pid][lval] = (reg[+pid][lval] || 0) * (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    reg[+pid].pc += 1;
  },

  opMod(lval, rval) {
    reg[+pid][lval] = (reg[+pid][lval] || 0) % (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    reg[+pid].pc += 1;
  }
};

function command(instruction) {
  let [_, op, lval, rval] = /([a-z]{3}) ([a-z\d-]+) ?([a-z\d-]+)?/.exec(instruction);
  switch(op) {
    case 'snd':
      return commands.opSnd.bind(null, lval);
    case 'set':
      return commands.opSet.bind(null, lval, rval);
    case 'add':
      return commands.opAdd.bind(null, lval, rval);
    case 'mul':
      return commands.opMul.bind(null, lval, rval);
    case 'mod':
      return commands.opMod.bind(null, lval, rval);
    case 'rcv':
      return commands.opRcv.bind(null, lval);
    case 'jgz':
      return commands.opJgz.bind(null, lval, rval);
  }
}

let program = document.body.innerText.trim().split('\n').map(instruction => command(instruction));

while (
  (
    (reg[+pid].pc >= 0 && reg[+pid].pc < program.length - 1)
    || (reg[+!pid].pc >= 0 && reg[+!pid].pc < program.length - 1)
  )
  && !(
    reg[0].rcv && reg[1].snd.length === 0
    && reg[1].rcv && reg[0].snd.length === 0
  )
  && ++reg[+pid].iter < 1e8 // Sanity limit
) {
  // Termination test
  if (program[reg[+pid].pc]) {
    program[reg[+pid].pc]();
  } else {
    pid = !pid;
  }
}

console.log(reg[1].sent);
```

Similar technique, but avoiding re-computing commands for speed. Lots of little subtleties in what the commands are allowed to do!

## Day 19
### Part 1

```javascript
let met = [], map = document.body.innerText.split('\n').map(l=>l.split(''));
console.log(map.map(l=>l.join('')).join('\n'));
let x = map[0].indexOf('|'), y = 0, dir = 2; // 0=up, 1=right, 2=down, 3=left
while (x >= 0 && x < map[0].length && y >= 0 && y < map.length) {
  if (/[A-Z]/.test(map[y][x])) {
    met.push(map[y][x]);
  } else if (map[y][x] === '+') {
    if (dir % 2 === 1) {
      if (map[y-1] !== undefined && map[y-1][x] !== ' ') { dir = 0; }
      else if (map[y+1] !== undefined && map[y+1][x] !== ' ') { dir = 2; }
      else { break; }
    } else {
      if (map[y][x-1] !== ' ') { dir = 3; }
      else if (map[y][x+1] !== ' ') { dir = 1; }
      else { break; }
    }
  } else if (map[y][x] === ' ') {
    break;
  }

  x -= dir % 2 === 1 ? dir - 2 : 0;
  y += dir % 2 === 0 ? dir - 1 : 0;
}
console.log(met.join(''));
```

Transform input to a grid, find the location of the '|' on the first line, start going down from there. Track every letter seen. Rotate when seeing a '+'. Stop if there's no direction to rotate or if going forward would not be in the route anymore.

### Part 2

```javascript
let met = [], map = document.body.innerText.split('\n').map(l=>l.split(''));
console.log(map.map(l=>l.join('')).join('\n'));
let steps = 0, x = map[0].indexOf('|'), y = 0, dir = 2; // 0=up, 1=right, 2=down, 3=left
while (x >= 0 && x < map[0].length && y >= 0 && y < map.length) {
  if (/[A-Z]/.test(map[y][x])) {
    met.push(map[y][x]);
  } else if (map[y][x] === '+') {
    if (dir % 2 === 1) {
      if (map[y-1] !== undefined && map[y-1][x] !== ' ') { dir = 0; }
      else if (map[y+1] !== undefined && map[y+1][x] !== ' ') { dir = 2; }
      else { break; }
    } else {
      if (map[y][x-1] !== ' ') { dir = 3; }
      else if (map[y][x+1] !== ' ') { dir = 1; }
      else { break; }
    }
  } else if (map[y][x] === ' ') {
    break;
  }

  x -= dir % 2 === 1 ? dir - 2 : 0;
  y += dir % 2 === 0 ? dir - 1 : 0;
  steps++;
}
console.log(steps);
```

Same solution, just keeping track of steps taken and spitting that out at the end instead.

## Day 20
### Part 1

```javascript
document.body.innerText.trim().split('\n')
  .map((line, index) => {
    let [_, p, v, a] = /^p=<([\d,-]+)>, v=<([\d,-]+)>, a=<([\d,-]+)>$/.exec(line);
    let particle = {
      i: index,
      p: p.split(',').map(Number),
      v: v.split(',').map(Number),
      a: a.split(',').map(Number)
    };
    particle.s_p = particle.p.reduce((t,v)=>t+Math.abs(v),0);
    particle.s_v = particle.v.reduce((t,v)=>t+Math.abs(v),0);
    particle.s_a = particle.a.reduce((t,v)=>t+Math.abs(v),0);
    return particle;
  })
  .sort((p1, p2) => {
    let diff = p1.s_a - p2.s_a;
    if (diff === 0) {
      diff = p1.s_v - p2.s_v;
    }
    if (diff === 0) {
      diff = p1.s_p - p2.s_p;
    }
    return diff;
  });
```

This one is cheating: long term means as time approaches infinity. The smallest acceleration will remain closest to the origin point, regardless of initial positions and velocities, given infinite time.

For each particle, we take the manhattan distance for its acceleration, velocity, and position. We sort on acceleration (closest to zero), break ties with velocity, and break those ties with initial position.

### Part 2

```javascript
let particles = document.body.innerText.trim().split('\n')
  .map((line, index) => {
    let [_, p, v, a] = /^p=<([\d,-]+)>, v=<([\d,-]+)>, a=<([\d,-]+)>$/.exec(line);
    return {
      i: index,
      p: p.split(',').map(Number),
      v: v.split(',').map(Number),
      a: a.split(',').map(Number)
    };
  });

// Track the most recent length of the list, and how many iterations it has stayed that length
let lastLength = particles.length, stableIterations=1;

// Keep going until the list stays the same length for 1000 iterations
while (stableIterations < 1000) {
  // Drop any particles for which another has the same position (and make sure not to check for particles against themselves)
  particles = particles.filter((v,i,a) => !a.some(w => v.i!==w.i && v.p[0]===w.p[0] && v.p[1]===w.p[1] && v.p[2]===w.p[2]));

  // Track as described above
  if (particles.length === lastLength) {
    stableIterations++;
  } else {
    stableIterations = 1;
    lastLength = particles.length;
  }

  // Take a step forward in the simulation: add acceleration to velocity, then velocity to position, for each particle.
  particles.forEach(particle => {
    particle.v[0] += particle.a[0];
    particle.v[1] += particle.a[1];
    particle.v[2] += particle.a[2];

    particle.p[0] += particle.v[0];
    particle.p[1] += particle.v[1];
    particle.p[2] += particle.v[2];
  });
}

console.log(lastLength);
```

Abstractly, we're looking for position curves that do not collide with any others over infinite time. Normally, this would mean solving a system of equations like in grade school algebra.

However, there's a catch making this harder: a particle colliding with another will eliminate both and create an end condition for their curves. We could deal with this by sorting collisions by lowest t-value and ignoring collisions involving particles that already did, but it's less code to just simulate each time step until the remaining particle list remains stable for a reasonable number of iterations.

## Day 21
### Part 1

```javascript
class Pattern {
  constructor(line) {
    let [, search, replace] = /^([.\/#]+) => ([.\/#]+)$/.exec(line);
    let baseSearch = new Grid(search.split('/').map(row => row.split('').map(col => col === '#' ? 1 : 0)));
    this.replace = new Grid(replace.split('/').map(row => row.split('').map(col => col === '#' ? 1 : 0)));
    this.patterns = baseSearch.permutations();
  }

  execute(grid) {
    if (grid.size === this.patterns[0].size && this.patterns.some(pattern => pattern.equals(grid))) {
      return new Grid(this.replace.grid);
    } else {
      return false;
    }
  }
}

class Grid {
  constructor(input) {
    this.grid = input.slice(0);
    this.size = this.grid.length;
  }

  enhance(patterns) {
    if (this.size === 2 || this.size === 3) {
      let enhanced = patterns.map(pattern => pattern.execute(this)).find(g => !!g);
      if (!enhanced) {
        console.error(`Failed to find a match for ${this.grid.join('/')}`);
        process.exit(0);
      }
      this.grid = enhanced.grid;
      this.size = this.grid.length;
    } else {
      let partitions = this.split();
      partitions.forEach(partition => partition.enhance(patterns));
      this.merge(partitions);
    }
  }

  split() {
    let partitionSize = this.size % 2 === 0 ? 2 : 3;
    let partitions = [];
    for (let r = 0; r < this.size; r += partitionSize) {
      for (let c = 0; c < this.size; c += partitionSize) {
        let partition = this.grid.slice(r, r + partitionSize).map(row => row.slice(c, c + partitionSize));
        partitions.push(new Grid(partition));
      }
    }
    return partitions;
  }

  merge(partitions) {
    let metaSize = Math.sqrt(partitions.length);
    let partitionSize = partitions[0].size;
    this.size = Math.ceil(this.size % 2 === 0 ? (3/2) * this.size : (4/3) * this.size);
    this.grid = Array(this.size).fill(0).map((_,r) => Array(this.size).fill(0).map((_,c) => {
      let partition = partitions[Math.floor(r/partitionSize)*metaSize + Math.floor(c/partitionSize)];
      return partition.grid[r%partitionSize][c%partitionSize];
    }));
  }

  equals(other) {
    return this.grid.reduce((t, row, r) => t && row.reduce((t, col, c) => t && col === other.grid[r][c], true), true);
  }

  /**
   * No matter how many operations you do, only eight states can be reached (at most - some patterns have fewer):
   * 0,-  1,-  2,-  3,-    -,v  v,1  v,2  v,3
   *           h,v         2,h  1,h  -,h  3,h
   *           v,h         v,-  3,v  2,v  1,v
   *                       h,2  h,3  h,-  h,1
   * ...  ##.  ..#  .#.    #..  .##  ...  .#.
   * ###  .#.  ###  .#.    ###  .#.  ###  .#.
   * #..  .#.  ...  .##    ...  .#.  ..#  ##.
   *
   * Thus, we'll return the unmodified grid, its three rotations, the vertically flipped grid, and ITS three rotations.
   */
  permutations() {
    let result = [new Grid(this.grid), new Grid(this.grid.reverse())];
    for (let rot = 0; rot < 3; rot++) {
      result.push(result[rot*2].rotate());
      result.push(result[rot*2 + 1].rotate());
    }
    return result;
  }

  rotate() {
    return new Grid(
      new Array(this.size).fill(0).map(
        (_, r) => Array(this.size).fill(0).map(
          (_, c) => this.grid[this.size-1-c][r]
        )
      )
    );
  }
}

let patterns = document.body.innerText.trim().split('\n').map(line => new Pattern(line));
let grid = new Grid([[0,1,0],[0,0,1],[1,1,1]]);

for(let iteration = 0; iteration < 5; iteration++) {
  grid.enhance(patterns);
}

console.log(grid.grid.reduce((t,row) => t + row.reduce((t,col) => t+col, 0), 0));
```

No special tricks this time - just a verbatim implementation and gnarly matrix manipulation math. A programming language that has native matrices would have a huge advantage (Mathematica for example).

We start with initial grid and the input. Each row of the input is transformed into a pair of grids (search pattern and replace pattern). The search pattern is replaced with a list of all eight permutations of rotation and mirroring.

Then, if the grid is small enough, we check it against all patterns and their permutations. The one that matches returns the replacement, and the grid becomes that. If the grid was NOT small enough, it is split into grids that are, those get mutated, and the grid becomes the result of stitching those back together.

We repeat this process five times per the challenge requirements.

### Part 2

Repeat of part 1, but iterate to 18 instead of 5 in the loop at the end. Logging the final pattern (1536x1536) does, in fact, look vaguely artistic.

It could be interesting to do fractal art with this technique.

## Day 22
### Part 1

```javascript
let grid = document.body.innerText.trim().split('\n').map(row => row.split('').map(cell => cell === '#'));
let dir = 0; // 0=up, 1=right, 2=down; 3=left
let x = Math.floor(grid[0].length/2); // Odds land in the center, evens land on the right edge of center
let y = Math.floor(grid.length/2);    // Same, except bottom edge for evens

let infections = 0;
for (let burst = 0; burst < 10000; burst++) {
  if (grid[y][x]) {
    dir = (dir+1)%4;
  } else {
    dir = (dir+3)%4;
  }

  grid[y][x] = !grid[y][x];
  if (grid[y][x]) {
    infections++;
  }

  x -= dir%2 === 1 ? dir-2 : 0;
  y += dir%2 === 0 ? dir-1 : 0;

  if (y >= grid.length) {
    grid.push(Array(grid[0].length).fill(false));
  } else if (y < 0) {
    grid.unshift(Array(grid[0].length).fill(false));
    y++;
  } else if (x >= grid[0].length) {
    grid = grid.map(row => [...row, false]);
  } else if (x < 0) {
    grid = grid.map(row => [false, ...row]);
    x++;
  }
}

console.log(infections);
```

Like the other grid walking challenges, but with extending the grid when going out-of-bounds instead of wrapping around.

### Part 2

```javascript
let grid = document.body.innerText.trim().split('\n').map(row => row.split('').map(cell => cell === '#' ? 'I' : 'C'));
let dir = 0; // 0=up, 1=right, 2=down; 3=left
let x = Math.floor(grid[0].length/2); // Odds land in the center, evens land on the right edge of center
let y = Math.floor(grid.length/2);    // Same, except bottom edge for evens

let infections = 0;
for (let burst = 0; burst < 10000000; burst++) {
  if (grid[y][x] === 'C') {
    grid[y][x] = 'W';
    dir = (dir+3)%4;
  } else if (grid[y][x] === 'W') {
    grid[y][x] = 'I';
    infections++;
  } else if (grid[y][x] === 'I') {
    grid[y][x] = 'F';
    dir = (dir+1)%4;
  } else if (grid[y][x] === 'F') {
    grid[y][x] = 'C';
    dir = (dir+2)%4;
  }

  x -= dir%2 === 1 ? dir-2 : 0;
  y += dir%2 === 0 ? dir-1 : 0;

  if (y >= grid.length) {
    grid.push(Array(grid[0].length).fill('C'));
  } else if (y < 0) {
    grid.unshift(Array(grid[0].length).fill('C'));
    y++;
  } else if (x >= grid[0].length) {
    grid = grid.map(row => [...row, 'C']);
  } else if (x < 0) {
    grid = grid.map(row => ['C', ...row]);
    x++;
  }
}

console.log(infections);
```

Slightly more complex decision engine and node state, with more iterations, but otherwise the same challenge/code.

## Day 23
### Part 1

```javascript
let reg = [
  {p: 0, pc: 0, snd: [], sent: 0, rcv: false, iter: 0},
  {p: 1, pc: 0, snd: [], sent: 0, rcv: false, iter: 1e8} // Disabled, only need one core for this one
];
let pid = false; // false=0, true=1, cheap trick for other places to toggle.
let numMul=0;

let commands = {
  opSnd(lval) {
    reg[+pid].snd.push(reg[+pid][lval]);
    reg[+pid].sent++;
    reg[+pid].pc += 1;
  },

  opRcv(lval) {
    let o = +!pid;
    if (reg[o].snd.length > 0) {
      reg[+pid][lval] = reg[o].snd.shift();
      reg[+pid].rcv = false;
      reg[+pid].pc += 1;
      pid = o;
    } else {
      reg[+pid].rcv = true;
      pid = o;
    }
  },

  opJgz(lval, rval) {
    if ((!isNaN(lval) && +lval > 0) || reg[+pid][lval] > 0) {
      reg[+pid].pc = reg[+pid].pc + (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    } else {
      reg[+pid].pc += 1;
    }
  },

  opJnz(lval, rval) {
    if ((!isNaN(lval) && +lval !== 0) || (reg[+pid][lval] || 0) !== 0) {
      reg[+pid].pc = reg[+pid].pc + (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    } else {
      reg[+pid].pc += 1;
    }
  },

  opSet(lval, rval) {
    reg[+pid][lval] = !isNaN(rval) ? +rval : reg[+pid][rval] || 0;
    reg[+pid].pc += 1;
  },

  opAdd(lval, rval) {
    reg[+pid][lval] = (reg[+pid][lval] || 0) + (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    reg[+pid].pc += 1;
  },

  opSub(lval, rval) {
    reg[+pid][lval] = (reg[+pid][lval] || 0) - (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    reg[+pid].pc += 1;
  },

  opMul(lval, rval) {
    reg[+pid][lval] = (reg[+pid][lval] || 0) * (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    reg[+pid].pc += 1;
    numMul++;
  },

  opMod(lval, rval) {
    reg[+pid][lval] = (reg[+pid][lval] || 0) % (!isNaN(rval) ? +rval : reg[+pid][rval] || 0);
    reg[+pid].pc += 1;
  }
};

function command(instruction) {
  let [_, op, lval, rval] = /([a-z]{3}) ([a-z\d-]+) ?([a-z\d-]+)?/.exec(instruction);
  switch(op) {
    case 'snd':
      return commands.opSnd.bind(null, lval);
    case 'set':
      return commands.opSet.bind(null, lval, rval);
    case 'add':
      return commands.opAdd.bind(null, lval, rval);
    case 'sub':
      return commands.opSub.bind(null, lval, rval);
    case 'mul':
      return commands.opMul.bind(null, lval, rval);
    case 'mod':
      return commands.opMod.bind(null, lval, rval);
    case 'rcv':
      return commands.opRcv.bind(null, lval);
    case 'jgz':
      return commands.opJgz.bind(null, lval, rval);
    case 'jnz':
      return commands.opJnz.bind(null, lval, rval);
  }
}

let program = document.body.innerText.trim().split('\n').map(instruction => command(instruction));

while (
  (
    (reg[+pid].pc >= 0 && reg[+pid].pc < program.length - 1)
    || (reg[+!pid].pc >= 0 && reg[+!pid].pc < program.length - 1)
  )
  && !(
    reg[0].rcv && reg[1].snd.length === 0
    && reg[1].rcv && reg[0].snd.length === 0
  )
  && ++reg[+pid].iter < 1e8 // Sanity limit
  ) {
  // Termination test
  if (program[reg[+pid].pc]) {
    program[reg[+pid].pc]();
  } else {
    pid = !pid;
  }
}

console.log(numMul);
```

Very similar to day 18, with some slightly different commands (jump had a bug - it wasn't checking for uninitialized registers before).

### Part 2

```javascript
h = 0;
for(let b=105700; b<=122700; b+= 17) {
  for(let e=2; e<b; e++) {
    if(b%e === 0) { h++; break; }
  }
}
console.log(h);
```

Manually translating the assembly into higher level code yields a brute force primality test. Very convoluted and roundabout. Also, it appears that everyone has the same answer this time - but the input is a random obfuscation of the same "real" code.