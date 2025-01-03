export function unique(arr, keyFunc = x => x) {
  return arr.filter((v, i, a) => a.indexOf(keyFunc(v)) === i);
}

export function* range(start = 0, end = Infinity, step = undefined) {
  if (end === Infinity) {
    end = start;
    start = 0;
  }

  if (start === end) return;

  if (step === 0) throw new Error('Step cannot be 0');
  else if (step === undefined) step = Math.sign(end - start);
  else if (Math.sign(step) !== Math.sign(end - start)) throw new Error('Stepping the wrong way');

  let i = start;
  while ((step > 0 && i < end) || (step < 0 && i > end)) {
    yield i;
    i += step;
  }
}

export function sum(arr, keyFunc = x => +x) {
  return arr.reduce((a, c) => a + keyFunc(c), 0);
}

export function productSum(arr, keyFunc = x => +x) {
  return arr.reduce((a, c) => a * keyFunc(c), 1);
}

export function cross(a, b) {
  if (!Array.isArray(a) || !Array.isArray(b)) throw new Error('Zip takes two arrays');
  return a.map(x => b.map(y => [x, y])).flat(1);
}