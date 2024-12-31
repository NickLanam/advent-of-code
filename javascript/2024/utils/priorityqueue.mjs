/** @typedef {<T>(first: T, second: T) => number} Comparator<T> */

export class PriorityQueue {
  /** @type T[] */
  #items;
  /** @type Comparator<T> */
  #compare;
  /** @type number */
  #maxSize;

  constructor(items, comparator, maxSize) {
    this.#maxSize = maxSize ?? 0;
    this.#items = items ?? [];
    this.#compare = comparator ?? ((first, second) => (first <= second ? -1 : 1));
    this.#heapify();
  }

  withComparator(comparator) {
    this.#compare = comparator;
    return this;
  }

  capMaxSize(maxSize) {
    this.#maxSize = maxSize;
    return this;
  }

  add(item) {
    this.#items.push(item);
    this.#heapifyUp();
    while (this.maxSize && this.size > this.maxSize) this.#items.pop();
  }
  clear() {
    this.#items = [];
  }
  peek() {
    return this.#items[0] ?? null;
  }
  remove() {
    if (this.#items.length === 0) return null;

    this.#swap(0, this.#items.length - 1);
    const head = this.#items.pop();
    this.#heapifyDown();
    return head;
  }
  get size() {
    return this.#items.length;
  }
  get maxSize() {
    return this.#maxSize;
  }
  toArray() {
    return [...this.#items];
  }
  forEachRemaining(callbackFn) {
    while (this.#items.length > 0) {
      callbackFn(this.remove());
    }
  }

  #heapify() {
    for (let i = Math.floor((this.#items.length - 1) / 2); i >= 0; i--) {
      this.#heapifyDown(i);
    }
  }
  #heapifyUp(i = this.#items.length - 1) {
    if (this.#hasParent(i) && this.#compare(this.#parent(i), this.#items[i]) > 0) {
      this.#swap(i, this.#parentIndex(i));
      this.#heapifyUp(this.#parentIndex(i));
    }
  }
  #heapifyDown(i = 0) {
    if (this.#hasLeftChild(i)) {
      let minIndex = i;
      if (this.#compare(this.#items[minIndex], this.#leftChild(i)) > 0) {
        minIndex = this.#leftChildIndex(i);
      }
      if (this.#hasRightChild(i) && this.#compare(this.#items[minIndex], this.#rightChild(i)) > 0) {
        minIndex = this.#rightChildIndex(i);
      }
      if (minIndex !== i) {
        this.#swap(i, minIndex);
        this.#heapifyDown(minIndex);
      }
    }
  }

  #swap(i, j) {
    const temp = this.#items[i];
    this.#items[i] = this.#items[j];
    this.#items[j] = temp;
  }

  #parent(i) {
    return this.#items[this.#parentIndex(i)];
  }
  #leftChild(i) {
    return this.#items[this.#leftChildIndex(i)];
  }
  #rightChild(i) {
    return this.#items[this.#rightChildIndex(i)];
  }

  #hasParent(i) {
    return i > 0;
  }
  #hasLeftChild(i) {
    return this.#leftChildIndex(i) < this.#items.length;
  }
  #hasRightChild(i) {
    return this.#rightChildIndex(i) < this.#items.length;
  }

  #parentIndex(i) {
    return Math.floor((i - 1) / 2);
  }
  #leftChildIndex(i) {
    return i * 2 + 1;
  }
  #rightChildIndex(i) {
    return i * 2 + 2;
  }
}