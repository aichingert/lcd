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
    pub fn evaluate_tree(root: Node) -> bool {
        Solution::solve(&root)
    }

    pub fn solve(cur: &Node) -> bool {
        if let Some(tr_node) = cur {
            let node = tr_node.borrow();
            if node.val <= 1 {
                return node.val == 1;
            }

            let lhs = Solution::solve(&node.left);
            if node.val == 2 && lhs {
                return true;
            }

            let rhs = Solution::solve(&node.right);
            if node.val == 2 && rhs {
                return true;
            }
            if node.val == 3 && rhs && lhs {
                return true;
            }
            return false;
        }

        false
    }
}
