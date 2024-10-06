use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::Solution;
use crate::TreeNode;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // This could be better with a bfs and a previous variable
        let mut m = HashMap::new();
        let res = sol(&root, 0, &mut m);

        if !res {
            return false;
        }

        for (k, v) in m {
            for i in 0..v.len() - 1 {
                if k & 1 == 0 {
                    if v[i] >= v[i + 1] {
                        return false;
                    }
                } else if v[i] <= v[i + 1] {
                    return false;
                }
            }
        }

        true
    }
}

fn sol(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, ans: &mut HashMap<i32, Vec<i32>>) -> bool {
    if let Some(n) = root {
        let n = n.borrow();
        ans.entry(l)
            .and_modify(|v| v.push(n.val))
            .or_insert(vec![n.val]);

        if l & 1 == 1 && n.val & 1 == 1 || l & 1 == 0 && n.val & 1 == 0 {
            return false;
        }

        sol(&n.left, l + 1, ans) && sol(&n.right, l + 1, ans)
    } else {
        true
    }
}
