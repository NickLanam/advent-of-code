const MAX_SEARCH_SIZE = 100_000;
const MAX_SPACE_ON_DISK = 70_000_000;
const MIN_FREE_AFTER_DELETE = 30_000_000;

function du(dirMap, dir) {
  let total = 0;
  [...Object.entries(dirMap[dir])].forEach(([key, what]) => {
    if (typeof what === 'number') total += what;
    else {
      const path = (dir + '/' + key).replace(/\/+/g, '/');
      total += du(dirMap, path);
    }
  });
  return total;
}

(await import('./aoc.mjs')).default(
  2022, 7,
  ({ dirMap }) => {
    let grand = 0;
    for (const dirName in dirMap) {
      const total = du(dirMap, dirName);
      if (total <= MAX_SEARCH_SIZE) grand += total;
    }
    return grand;
  }, 95437,
  ({ dirMap }) => {
    const grandTotal = du(dirMap, '/');
    const limit = MAX_SPACE_ON_DISK - MIN_FREE_AFTER_DELETE;
    const needToFree = Math.max(grandTotal - limit, 0);
    const candidates = [];
    for (const dirName in dirMap) {
      const total = du(dirMap, dirName);
      if (total >= needToFree) candidates.push(total);
    }
    return Math.min(...candidates);
  }, 24933642,
  (data) => {
    let dirMap = { '/': {} };
    let cwd = '/';
    for (let lineNo = 0; lineNo < data.length; lineNo++) {
      const line = data[lineNo];
      if (line.startsWith('$ cd')) {
        const relPath = line.substr(5);
        cwd = cwd + '/' + relPath;
        cwd = cwd.replace(/\/+/g, '/').replace(/\/[^\/]+\/\.\.$/, '');
        if (!cwd) cwd = '/';
      } else if (line.startsWith('$ ls')) {
        let res = data.slice(lineNo + 1);
        let last = res.findIndex(l => l.startsWith('$'));
        if (last === -1) last = res.length;
        res = res.slice(0, last);

        for (const content of res) {
          dirMap[cwd] = dirMap[cwd] ?? {}; // Don't actually need to build the nesting for this to work.
          if (content.startsWith('dir')) {
            dirMap[cwd][content.substring(4)] = dirMap[cwd][content.substring(4)] ?? {};
          } else {
            let [size, fname] = content.split(' ');
            size = +size;
            dirMap[cwd][fname] = size;
          }
        }
      } // Lines that aren't commands get skipped because the `ls` part already consumes them
    }
    return { cwd, dirMap };
  }
);