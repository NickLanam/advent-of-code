function runMonkeyBusiness(monkeys) {
  const known = new Map(monkeys.filter(([, f]) => !Number.isNaN(+f)));
  let unknown = new Map(monkeys.filter(([, f]) => Number.isNaN(+f)).map(([n, f]) => [n, f.split(' ')]));
  let prev = unknown.size;
  while (unknown.size > 0) {
    for (const [id, [left, op, right]] of unknown) {
      const l = known.get(left);
      const r = known.get(right);
      if (l !== undefined && r !== undefined) {
        let v;
        switch (op) {
          case '+': v = l + r; break;
          case '-': v = l - r; break;
          case '*': v = l * r; break;
          case '/': v = l / r; break;
          default: throw new Error(`Impossible op in runMonkeyBusiness: ${op}`);
        }
        known.set(id, v);
        unknown.delete(id);
      }
    }

    // Stop when an attempt at reduction doesn't make any progress
    if (unknown.size > 0 && unknown.size === prev) break;
    else prev = unknown.size;
  }
  return { known, unknown };
}

function tryReplace(known, unknown, equation) {
  if (Array.isArray(equation)) {
    return equation.map(part => tryReplace(known, unknown, part));
  }
  if (!Number.isNaN(+equation)) {
    return equation;
  }

  const parts = equation.split(' ').map(p => {
    if (!Number.isNaN(+p)) return +p;
    const k = p.replaceAll(/[\(\)]+/g, '');
    if (!known.has(k)) {
      return unknown.get(k) ?? k;
    };
    const v = known.get(k);
    if (!Number.isNaN(+v)) return +v;
    return Array.isArray(v) ? v.flat(1) : v;
  });
  if (parts.length === 1) return parts[0];
  return parts.flat(1);
}

function trySolve(equation, value) {
  const i = equation.findIndex(v => !Number.isNaN(+v));
  const n = equation[i];
  const e = equation[2 - i];

  let newVal;
  switch(equation[1]) {
    case '*': newVal = value / n; break;
    case '/': newVal = (i === 0) ? n / value : value * n; break;
    case '+': newVal = value - n; break;
    case '-': newVal = (i === 0) ? n - value : value + n; break;
    default: throw new Error('Impossible op in trySolve:' + equation[1]);
  }

  // console.log({ equation, value, e, n, newVal });
  return { equation: e, value: newVal };
}

function part2(allMonkeys) {
  const monkeys = allMonkeys.filter(([n]) => !['humn', 'root'].includes(n));
  const root = allMonkeys.find(([n]) => n === 'root');

  const { known, unknown } = runMonkeyBusiness(monkeys);

  // Run one round of replacements directly to figure out which side of the equation is known.
  let [left,, right] = root[1].split(' ');
  left = tryReplace(known, unknown, left);
  right = tryReplace(known, unknown, right);
  let proc; let val;
  if (typeof left === 'number') {
    proc = right;
    val = left;
  } else {
    proc = left;
    val = right;
  }

  // Expand variables and simplify the equation until we get `humn = someNumber`.
  // console.log('Figured out where to start', { known, unknown, proc, val });
  while (proc !== 'humn') {
    proc = tryReplace(known, unknown, proc);
    ({ equation: proc, value: val } = trySolve(proc, val));
    if (Math.floor(val) !== val) throw new Error('Must have messed up, val became a fraction here');
  }

  return val;
}

(await import('./aoc.mjs')).default(
  2022, 21,
  (data) => runMonkeyBusiness(data).known.get('root'), 152,
  (data) => part2(data), 301,
  data => data.map(line => line.split(': ')).map(([id, f]) => [id, Number.isNaN(+f) ? f : +f]),
  true, false
);