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
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        sum(&root, 0)
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

fn sum(node: &Node, cur: i32) -> i32 {
    let r = node.as_ref().unwrap();
    let n = cur * 10 + r.borrow().val;
    let mut acc = 0;

    if r.borrow().left.is_some() {
        acc += sum(&r.borrow().left, n);
    }
    if r.borrow().right.is_some() {
        acc += sum(&r.borrow().right, n);
    }
    if acc == 0 {
        acc = n;
    }

    acc
}
