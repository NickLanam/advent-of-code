const IDENTIFIER = Symbol('Point3D');

export class Point3D {
  [IDENTIFIER];

  static #expectComparable(a, b) {
    if (!(a instanceof Point3D) || !(b instanceof Point3D)) {
      throw new Error(`Expected two points, got ${typeof a} and ${typeof b}`);
    }
  }

  static fromString(s) {
    if (typeof s !== 'string') {
      throw new Error(`Point3D cannot convert from ${typeof s}: ${JSON.stringify(s)}`);
    }

    const v = s.trim().split(',');
    const [x, y, z] = v.map(n => +n.trim());

    if (v.length !== 3 || Number.isNaN(x) || Number.isNaN(y) || Number.isNaN(z)) {
      throw new Error(`Point3D.fromString expects three comma-separated numbers, instead saw "${s}"`);
    }

    return new Point3D(x, y, z);
  }

  static manhattanDistance(a, b) {
    Point3D.#expectComparable(a, b);
    return Math.abs(b.x - a.x) + Math.abs(b.y - a.y) + Math.abs(b.z - a.z);
  }

  static squaredDistance(a, b) {
    Point3D.#expectComparable(a, b);
    return (b.x - a.x)**2 + (b.y - a.y)**2 + (b.z - a.z)**2;
  }

  static distance(a, b) {
    return Math.sqrt(this.squaredDistance(a, b));
  }

  static difference(a, b) {
    Point3D.#expectComparable(a, b);
    return [b.x - a.x, b.y - a.y, b.z - a.z];
  }

  constructor(x, y, z) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  roll(x, y, z) {
    return new Point3D( y, -x,  z); // Roll 90 degrees right
  }

  roll2(x, y, z) {
    return new Point3D(-x, -y,  z); // Roll 180 degrees
  }

  roll3(x, y, z) {
    return new Point3D(-y,  x,  z); // Roll 90 degrees left
  }

  pitch(x, y, z) {
    return new Point3D( x, -z, y); // Look 90 degrees up
  }

  pitch2(x, y, z) {
    return new Point3D( x, -y, -z); // Flip upside-down
  }

  pitch3(x, y, z) {
    return new Point3D( x,  z, -y); // Look 90 degrees down
  }

  yaw(x, y, z) {
    return new Point3D( z,  y, -x); // Look 90 degrees right
  }

  yaw2(x, y, z) {
    return new Point3D(-x,  y, -z); // Look 180 degrees behind (turn around)
  }

  yaw3(x, y, z) {
    return new Point3D(-z,  y,  x); // Look 90 degrees left
  }

  /**
   * Choose one of the 48 possible orientations of a point3D.
   * This sets which way a vector starting at that point3D is facing, and at what roll.
   * For example, point3Ding towards +z with "up" being +y is one orientation.
   * There are six cardinal directions to point3D (-x, +x, -y, +y, -z, +z).
   * There are four ways that can be "up" relative to the facing direction.
   * And, the third axis can be mirrored, doubling the possible orientations.
   *
   * In total, that's 6*4*2 = 48 possible orientations.
   *
   * @param {number} which 0-47 inclusive.
   */
  orient(which) {
    const flip = which >= 24 ? -1 : 1;
    let [x, y, z] = [flip * this.x, flip * this.y, flip * this.z];
    switch (which % 24) {
      // Facing: +z (default).
      case 0:  [x, y, z] = [ x,  y,  z]; break;
      case 1:  [x, y, z] = [ y, -x,  z]; break;
      case 2:  [x, y, z] = [-x, -y,  z]; break;
      case 3:  [x, y, z] = [-y,  x,  z]; break;

      // Facing: +x (yaw).
      case 4:  [x, y, z] = [ z,  y, -x]; break;
      case 5:  [x, y, z] = [ y, -z, -x]; break;
      case 6:  [x, y, z] = [-z, -y, -x]; break;
      case 7:  [x, y, z] = [-y,  z, -x]; break;

      // Facing: -z (yaw2)
      case 8:  [x, y, z] = [-x,  y, -z]; break;
      case 9:  [x, y, z] = [ y,  x, -z]; break;
      case 10: [x, y, z] = [ x, -y, -z]; break;
      case 11: [x, y, z] = [-y, -x, -z]; break;

      // Facing: -x (yaw3)
      case 12: [x, y, z] = [-z,  y,  x]; break;
      case 13: [x, y, z] = [ y,  z,  x]; break;
      case 14: [x, y, z] = [ z, -y,  x]; break;
      case 15: [x, y, z] = [-y, -z,  x]; break;

      // Facing: +y (pitch)
      case 16: [x, y, z] = [ x, -z,  y]; break;
      case 17: [x, y, z] = [-z, -x,  y]; break;
      case 18: [x, y, z] = [-x,  z,  y]; break;
      case 19: [x, y, z] = [ z,  x,  y]; break;

      // Facing: -y (pitch3)
      case 20: [x, y, z] = [ x,  z, -y]; break;
      case 21: [x, y, z] = [ z, -x, -y]; break;
      case 22: [x, y, z] = [-x, -z, -y]; break;
      case 23: [x, y, z] = [-z,  x, -y]; break;

      default: throw new Error('Unknown rotation: ' + which);
    }
    return new Point3D(x, y, z);
  }

  /** Undoes whatever orient(which) does. */
  orientReverse(which) {
    let [x, y, z] = [this.x, this.y, this.z];
    switch (which % 24) {
      case 0:  [x, y, z] = [ x,  y,  z]; break;
      case 1:  [y, x, z] = [ x, -y,  z]; break;
      case 2:  [x, y, z] = [-x, -y,  z]; break;
      case 3:  [y, x, z] = [-x,  y,  z]; break;
      case 4:  [z, y, x] = [ x,  y, -z]; break;
      case 5:  [y, z, x] = [ x, -y, -z]; break;
      case 6:  [z, y, x] = [-x, -y, -z]; break;
      case 7:  [y, z, x] = [-x,  y, -z]; break;
      case 8:  [x, y, z] = [-x,  y, -z]; break;
      case 9:  [y, x, z] = [ x,  y, -z]; break;
      case 10: [x, y, z] = [ x, -y, -z]; break;
      case 11: [y, x, z] = [-x, -y, -z]; break;
      case 12: [z, y, x] = [-x,  y,  z]; break;
      case 13: [y, z, x] = [ x,  y,  z]; break;
      case 14: [z, y, x] = [ x, -y,  z]; break;
      case 15: [y, z, x] = [-x, -y,  z]; break;
      case 16: [x, z, y] = [ x, -y,  z]; break;
      case 17: [z, x, y] = [-x, -y,  z]; break;
      case 18: [x, z, y] = [-x,  y,  z]; break;
      case 19: [z, x, y] = [ x,  y,  z]; break;
      case 20: [x, z, y] = [ x,  y, -z]; break;
      case 21: [z, x, y] = [ x, -y, -z]; break;
      case 22: [x, z, y] = [-x, -y, -z]; break;
      case 23: [z, x, y] = [-x,  y, -z]; break;

      default: throw new Error('Unknown rotation: ' + which);
    }
    return which >= 24 ? new Point3D(-x, -y, -z) : new Point3D(x, y, z);
  }

  add(other) {
    Point3D.#expectComparable(this, other);
    return new Point3D(this.x + other.x, this.y + other.y, this.z + other.z);
  }

  subtract(other) {
    Point3D.#expectComparable(this, other);
    return new Point3D(this.x - other.x, this.y - other.y, this.z - other.z);
  }

  translate(dx, dy, dz) {
    return new Point3D(this.x + dx, this.y + dy, this.z + dz);
  }

  equals(other) {
    Point3D.#expectComparable(this, other);
    return this.x === other.x && this.y === other.y && this.z === other.z;
  }

  // A trick to check if two points are orientations of each other.
  toNormal() {
    return new Point3D(...this.toArray().map(n => Math.abs(n)).sort((a, b) => b - a));
  }

  toArray() {
    return [this.x, this.y, this.z];
  }

  toString() {
    return [this.x, this.y, this.z].join(',');
  }

  [Symbol.toPrimitive](hint) {
    if (hint === 'string') return this.toString();
    return null;
  }
}