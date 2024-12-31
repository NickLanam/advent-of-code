import {
  nestedListToBinaryTree as toTree,
  depth,
  binaryTreeToString as treeString,
  flatNodes
} from './data-structures/binary-tree.mjs';

function addToNearestLeft(start) {
  if (!start.parent) return false; // Shortcut

  const val = start.left;

  // Find the entire ancestry of the start node for easier checks.
  const ancestors = [start];
  let ancestor = start.parent;
  while (ancestor) {
    ancestors.push(ancestor);
    ancestor = ancestor.parent;
  }

  // Climb until we find a node whose RIGHT child is an ancestor of the start node.
  // That is, keep climbing until we can go left.
  let check = start.parent;
  while (check && ancestors.includes(check.left)) {
    check = check.parent;
  }
  if (check == null) {
    return false; // We went all the way up the tree but there's nothing further left.
  }

  // Now descent left once. If we already found a plain number, add val to that and we're done.
  if (typeof check.left === 'number') {
    check.left += val;
    return true;
  }   {
    check = check.left;
  }

  // We still haven't found a plain number, so descend to the right until we do.
  while (check) {
    if (typeof check.right === 'number') {
      check.right += val;
      return true;
    }
    check = check.right;
  }

  // return false;
  throw new Error('Tried to find a left neighbor, and fell through every check. Tree is likely built wrong.');
}

function addToNearestRight(start) {
  if (!start.parent) return false; // Shortcut

  const val = start.right;

  // Find the entire ancestry of the start node for easier checks.
  const ancestors = [start];
  let ancestor = start.parent;
  while (ancestor) {
    ancestors.push(ancestor);
    ancestor = ancestor.parent;
  }

  // Climb until we find a node whose LEFT child is an ancestor of the start node.
  // That is, keep climbing until we can go right.
  let check = start.parent;
  while (check && ancestors.includes(check.right)) {
    check = check.parent;
  }
  if (check == null) {
    return false; // We went all the way up the tree but there's nothing further right.
  }

  // Now descent right once. If we already found a plain number, add val to that and we're done.
  if (typeof check.right === 'number') {
    check.right += val;
    return true;
  }   {
    check = check.right;
  }

  // We still haven't found a plain number, so descend to the left until we do.
  while (check) {
    if (typeof check.left === 'number') {
      check.left += val;
      return true;
    }
    check = check.left;
  }

  // return false;
  throw new Error('Tried to find a right neighbor, and fell through every check. Tree is likely built wrong.');
}

// Returns true if an explosion happened; false if there was no explosion to do.
function tryExplode(tree) {
  
  // Find a [l, r] pair that is at least four levels deep (as in, it has a great-great-grandparent).
  // If there isn't one, we're done.
  // If such a pair is found:
  // - Replace the [l, r] pair with a 0 (no pair).
  // - Follow left siblings up the parent tree. If one is found, add l to it.
  // - Follow right siblings up the parent tree. If one is found, add r to it.
  // Only do this to the first match (first deep pair; nearest left cousin; nearest right cousin)

  const stack = [tree];
  while (stack.length) {
    const node = stack.pop();
    const numParents = depth(node);
    if (numParents >= 4 && typeof node.left === 'number' && typeof node.right === 'number') {
      // First, add the left side of the exploding node to its nearest neighbor on the left (if possible).
      // Do the same on the right.
      const didLeft = addToNearestLeft(node);
      const didRight = addToNearestRight(node);

      if (!didLeft && !didRight) throw new Error(`Adding to left failed and same for right. Technically possible but something went wrong more likely. ${treeString(node, true)}`);

      // Then, replace the original node with a 0 (instead of a pair).
      if (node.parent.left === node) node.parent.left = 0;
      else if (node.parent.right === node) node.parent.right = 0;
      else {
        throw new Error('Linkage is broken');
      }

      return true;
    }
    // We're pushing the right child first, so that the left child is popped first.
    // This way, we make sure to explode from left to right (for some inputs, order changes result).
    if (typeof node.right !== 'number') {
      stack.push(node.right);
    }
    if (typeof node.left !== 'number') {
      stack.push(node.left);
    }
  }

  return false;
}

// Returns true if a split happened; false if there was no split to do.
function trySplit(tree) {
  // Find any value >= 10 anywhere in the tree.
  // If one is found, replace that node with [Math.floor(val / 2), Math.ceil(val / 2)]
  // Only do this to the first match.

  const leavesLeftToRight = flatNodes(tree);

  if (leavesLeftToRight.length === 0) return; // Shortcut

  for (const leaf of leavesLeftToRight) {
    if (typeof leaf.left === 'number' && leaf.left >= 10) {
      leaf.left = toTree([Math.floor(leaf.left / 2), Math.ceil(leaf.left / 2)], leaf);
      return true;
    }
    if (typeof leaf.right === 'number' && leaf.right >= 10) {
      leaf.right = toTree([Math.floor(leaf.right / 2), Math.ceil(leaf.right / 2)], leaf);
      return true;
    }
  }

  return false;
}

function reduce(tree) {
  for (let LIM = 10_000; LIM > 0; LIM--) {
    const didExplode = tryExplode(tree);
    if (!didExplode) {
      const didSplit = trySplit(tree);
      if (!didSplit) {
        return tree;
      }
    }
  }
  if (LIM === 0) throw new Error('Stopped after 10k simplification attempts... tree so far: ' + treeString(tree));
  return tree;
}

function merge(left, right) {
  return reduce(toTree([toTree(left), toTree(right)]));
}

function mergeAll(...lines) {
  let out = toTree(lines.shift());
  while (lines.length) out = merge(out, lines.shift());
  out = reduce(out); // In case there was only one line in the input, reduce it anyway
  return out;
}

function magnitude(tree) {
  if (typeof tree === 'number') return tree;
  return 3 * magnitude(tree.left) + 2 * magnitude(tree.right);
}

(await import('./aoc.mjs')).default(
  2021, 18,
  all => magnitude(mergeAll(...all)), 4140,
  (all) => {
    // x+y may not equal y+x, so we have to try every permutation of two elements.
    // This is, of course, kinda slow.
    const combinations = all.map(
      (list, i) => all.filter((_, j) => j !== i).map(other => magnitude(merge(list, other)))
    ).flat().sort(((a, b) => b - a));
    return combinations[0];
  }, 3993,
  lines => lines.map(line => JSON.parse(line))
);