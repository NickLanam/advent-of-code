export function unique(arr, keyFunc = x => x) {
  return arr.filter((v, i, a) => a.indexOf(keyFunc(v)) === i);
}

export default function applyPolyfills() {
  Array.prototype.unique = keyFunc => unique(this, keyFunc);
}