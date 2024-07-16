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
    pub fn get_directions(root: Node, start_value: i32, dest_value: i32) -> String {
        let mut s_path = Vec::new();
        let mut d_path = Vec::new();

        find(&root, start_value, &mut s_path);
        find(&root, dest_value, &mut d_path);

        while s_path[s_path.len() - 1] == d_path[d_path.len() - 1] {
            s_path.pop();
            d_path.pop();
        }

        s_path.into_iter().skip(1).map(|_| 'U').collect::<String>() + 
        &d_path.into_iter().skip(1).rev().collect::<String>()
    }
}

fn find(node: &Node, val: i32, s: &mut Vec<char>) {
    if let Some(cur) = node {
        let tref = cur.borrow();

        if tref.val == val {
            s.push('F');
            return;
        }

        find(&tref.left, val, s);

        if !s.is_empty() {
            s.push('L');
            return;
        }

        find(&tref.right, val, s);
        if !s.is_empty() {
            s.push('R');
        }
    }
}
