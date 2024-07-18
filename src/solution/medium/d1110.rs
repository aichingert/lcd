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
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, mut to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        to_delete.sort_unstable();
        let mut ans = Vec::new();
        d(&root, &mut ans, &to_delete, true);
        ans
    }
}

fn d(c: &Node, forest: &mut Vec<Node>, to_delete: &[i32], pd: bool) -> bool {
    if let Some(n) = c {
        let dc = to_delete.binary_search(&n.borrow().val).is_ok();

        if d(&n.borrow().left, forest, to_delete, dc) {
            n.borrow_mut().left = None;
        }
        if d(&n.borrow().right, forest, to_delete, dc) {
            n.borrow_mut().right = None;
        }

        if !dc && pd {
            forest.push(c.clone());
        }

        return dc;
    }

    false
}
