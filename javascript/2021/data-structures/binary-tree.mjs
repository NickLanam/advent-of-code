const IDENTIFIER = Symbol('BinaryTree');

export function nestedListToBinaryTree(nestedList, parent = null) {
  if (nestedList == null) {
    throw new Error('nestedListToBinaryTree expects no null/undefined values at the leaf nodes.');
  }

  if (typeof nestedList === 'object' && nestedList[IDENTIFIER] === true) {
    nestedList.parent = parent;
    return nestedList;
  }

  if (Array.isArray(nestedList)) {
    if (nestedList.length !== 2) throw new Error('nestedListToBinaryTree expects arrays to have exactly 2 elements.');
    const node = { [IDENTIFIER]: true, parent };
    node.left = nestedListToBinaryTree(nestedList[0], node);
    node.right = nestedListToBinaryTree(nestedList[1], node);
    if (node.left?.[IDENTIFIER] === true && node.left.parent !== node) {
      console.error(node);
      throw new Error('Left did not get parent assigned back up properly');
    }
    if (node.right?.[IDENTIFIER] === true && node.right.parent !== node) {
      console.error(node);
      throw new Error('Right did not get parent assigned back up properly');
    }
    return node;
  }

  return nestedList; // Looks to be a primitive, so it's a leaf node.
}

export function binaryTreeToNestedList(tree) {
  if (tree == null) {
    throw new Error('binaryTreeToNestedList called with null');
  }
  if (typeof tree !== 'object') {
    return tree; // Probably a leaf node.
  }
  if (!tree[IDENTIFIER]) {
    throw new Error('binaryTreeToNestedList called on an object that is not a tree ' + JSON.stringify(tree));
  }
  if (tree.parent == null && tree.left == null && tree.right == null) {
    throw new Error('binaryTreeToNestedList called on a node with no parent and no children');
  }
  return [binaryTreeToNestedList(tree.left), binaryTreeToNestedList(tree.right)];
}

/** Counts ancestors of the given tree node. No parent = 0; has parent = 1; parent has parent = 2; etc. */
export function depth(tree) {
  if (tree == null || typeof tree !== 'object' || !tree[IDENTIFIER]) {
    throw new Error('Called countParents on a non-tree: ' + JSON.stringify(tree));
  }
  let count = 0;
  let node = tree;
  while (node.parent != null) {
    count++;
    node = node.parent;
  }
  return count;
}

export function flatNodes(tree) {
  if (tree == null) {
    throw new Error('flatNodes called on a nullish value');
  }
  if (typeof tree !== 'object') {
    throw new Error('flatNodes was not supposed to recurse all the way to the actual values. Stop at the nodes that HAVE leaves.');
  }
  if (!tree[IDENTIFIER]) {
    throw new Error('flatNodes called on a non-tree: ' + JSON.stringify(tree));
  }
  if (tree.parent == null && tree.left == null && tree.right == null) {
    throw new Error('flatNodes called on a node with no parent and no children');
  }

  const leftIsLeaf = typeof tree.left === 'number';
  const rightIsLeaf = typeof tree.right === 'number';

  if (leftIsLeaf && rightIsLeaf) return [tree];
  else if (leftIsLeaf) return [tree, flatNodes(tree.right)].flat();
  else if (rightIsLeaf) return [flatNodes(tree.left), tree].flat();
  else return [flatNodes(tree.left), flatNodes(tree.right)].flat();
}

export function binaryTreeToString(tree, withHeritage = false) {
  if (tree == null) {
    throw new Error('binaryTreeToString called on nullish value');
  }
  if (typeof tree !== 'object') {
    return tree;
  }
  if (!tree[IDENTIFIER]) {
    throw new Error('binaryTreeToString called on a non-tree: ' + JSON.stringify(tree));
  }

  let out = JSON.stringify(binaryTreeToNestedList(tree));
  if (withHeritage && tree.parent != null) {
    out += ' < ' + binaryTreeToString(tree.parent, true);
  }
  return out;
}