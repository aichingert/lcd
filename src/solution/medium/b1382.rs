use crate::Solution;
use crate::TreeNode;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn balance_bst(root: Node) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vec = Vec::new();
        collect(&root, &mut vec);
        build(&vec)
    }
}

fn build(v: &[i32]) -> Node {
    if v.is_empty() {
        return None;
    }
    let mut tn = TreeNode::new(v[v.len() / 2]);

    tn.left = build(&v[..v.len() / 2]);
    tn.right = build(&v[v.len() / 2 + 1..]);

    Some(Rc::new(RefCell::new(tn)))
}

fn collect(cur: &Node, vec: &mut Vec<i32>) {
    if let Some(node) = cur {
        let brw = node.borrow();
        collect(&brw.left, vec);
        vec.push(brw.val);
        collect(&brw.right, vec);
    }
}
