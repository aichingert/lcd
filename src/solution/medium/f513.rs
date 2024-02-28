use crate::Solution;
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_bottom_left_value(root: Node) -> i32 {
        sol(&root, 0).0.unwrap()
    }
}

fn sol(r: &Node, d: i32) -> (Option<i32>, i32) {
    if let Some(n) = r {
        let n = n.borrow();

        let l = sol(&n.left, d + 1);
        let r = sol(&n.right, d + 1);

        if l.0.is_none() && r.0.is_none() {
            return (Some(n.val), d)
        } else if l.1 != -1 || r.1 != -1 {
            return if l.1 >= r.1 {
                l
            } else {
                r
            };
        }
    }

    (None, -1)
}
