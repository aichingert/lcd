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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut v = Vec::new();
        s(&root, &mut v, 0);

        if (k - 1) >= v.len() as i32 {
            return -1;
        }

        v.sort_unstable();
        v[v.len() - k as usize]
    }
}

fn s(n: &Node, v: &mut Vec<i64>, lvl: usize) {
    let Some(c) = n else {
        return;
    };

    if lvl >= v.len() {
        v.push(0);
    }

    v[lvl] += c.borrow().val as i64;
    s(&c.borrow().left, v, lvl + 1);
    s(&c.borrow().right, v, lvl + 1);
}
