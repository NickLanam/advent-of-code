/** Triangular numbers: n + (n - 1) + (n - 2) + ... + 1 */
export function triangulate(n) {
  return (n * (Math.abs(n) + 1)) / 2;
}

/** If this isn't a whole number, then x wasn't triangular. Still, the reference point might have use. */
export function triangularRoot(x) {
  return Math.sign(x) * (Math.sqrt(8 * Math.abs(x) + 1) - 1) / 2;
}