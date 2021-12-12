function traverse(links, allowExtraVisit = false) {
  const paths = [];
  const pendingPaths = [];
  pendingPaths.push(['start']);
  let LIM = 1_000_000; // Actually needs to be this high - the real input can get upwards of 100k results.
  while (pendingPaths.length && --LIM > 0) {
    const look = pendingPaths.pop();
    const options = links[look[look.length - 1]].filter(k => {
      if (allowExtraVisit) {
        const existingExtra = look.filter((e, i) => look.lastIndexOf(e) !== i && e.toUpperCase() !== e);
        if (existingExtra.length > 1) throw new Error(`A path broke the rules: [${newPath.join(', ')}]`);
        if (existingExtra.length && look.includes(k) && k.toUpperCase() !== k) return false;
        return k !== 'start' && !existingExtra.includes(k);
      } else {
        return k !== 'start' && (k.toUpperCase() === k || !look.includes(k));
      }
    });
    for (const next of options) {
      const newPath = [...look, next];
      const doubles = newPath.filter((e, i) => newPath.lastIndexOf(e) !== i && e.toUpperCase() !== e);
      if (doubles.length > +allowExtraVisit) {
        throw new Error(`A path broke the rules: [${newPath.join(', ')}]`);
      }
      if (next === 'end') {
        paths.push(newPath);
      } else {
        pendingPaths.push(newPath);
      }
    }
  }
  if (LIM < 1) throw new Error('LIMIT REACHED');
  return paths.length;
}

(await import('./aoc.mjs')).default(
  2021, 12,
  links => traverse(links, false), 10,
  links => traverse(links, true), 36,
  raw => {
    const links = {};
    for (const [a, b] of raw.map(line => line.split('-'))) {
      if (!links[a]) links[a] = [b];
      else links[a].push(b);
      if (!links[b]) links[b] = [a];
      else links[b].push(a);
    }
    return links;
  }
);