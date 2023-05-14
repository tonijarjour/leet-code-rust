class Node {
  constructor(val, children) {
    this.val = val;
    this.children = children;
  }
}

const preorder = (root) => {
  if (!root) {
    return [];
  }

  let stack = root.children.reverse();
  let res = [root.val];

  while (stack.length > 0) {
    console.log(stack)
    let curr = stack.pop();
    res.push(curr.val);
    stack = stack.concat(curr.children.reverse());
  }

  return res;
}

console.log(preorder(new Node(1, [new Node(3, [new Node(5, []), new Node(6, [])]), new Node(2, []), new Node(4, [])])))
