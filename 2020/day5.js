const { getInput } = require('./utils');

function repartition(start, stop, lo) {
  const mid = Math.floor((stop - start) / 2) + start;
  return lo ? [start, mid] : [Math.min(mid + 1, stop), stop];
}

function findSeat(code) {
  const fb = code.substr(0, 7).split ``.map(c => c === 'F');
  const lr = code.substr(7).split ``.map(c => c === 'L');
  let fbp = [0, 127];
  let lrp = [0, 7];
  for (const fbs of fb) {
    fbp = repartition(...fbp, fbs);
  }
  for (const lrs of lr) {
    lrp = repartition(...lrp, lrs);
  }
  const row = fbp[0];
  const col = lrp[0];
  const sid = row * 8 + col;
  return { code, row, col, sid };
}

const day5star1 = (() => {
  const lines = getInput(5);
  const seats = lines.map(l => findSeat(l));
  const maxSid = seats.reduce((a, c) => Math.max(c.sid, a), 0);
  return maxSid;
})();

const day5star2 = (() => {
  const lines = getInput(5);
  const sids = lines.map(l => findSeat(l).sid).sort(((a, b) => a - b));
  for (let i = 1; i < sids.length - 1; i++) {
    const v = sids[i];
    if (sids[i + 1] !== v + 1) return v + 1;
  };
})();

console.log('Star 1: ', day5star1);
console.log('Star 2: ', day5star2);