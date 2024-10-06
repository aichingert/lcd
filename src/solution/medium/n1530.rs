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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut ans = 0;
        fnd(&root, &mut ans, distance);
        ans
    }
}

fn fnd(c: &Node, ans: &mut i32, dis: i32) -> Option<Vec<i32>> {
    let nb = c.as_ref()?.borrow();

    Some(match (fnd(&nb.left, ans, dis), fnd(&nb.right, ans, dis)) {
        (Some(l), Some(r)) => {
            for a in &l {
                for b in &r {
                    if a + b <= dis {
                        *ans += 1;
                    }
                }
            }

            l.into_iter()
                .chain(r)
                .map(|n| n + 1)
                .filter(|&n| n < dis)
                .collect()
        }
        (None, Some(r)) => r.into_iter().map(|n| n + 1).filter(|&n| n < dis).collect(),
        (Some(l), None) => l.into_iter().map(|n| n + 1).filter(|&n| n < dis).collect(),
        (None, None) => vec![1],
    })
}
