/* Daily problem: 2023-11-02
 * aichingert
 */

use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::TreeNode;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Solution::find_avg_subtrees(&root, &mut ans);
        ans
    }

    fn find_avg_subtrees(node: &Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32) -> (i32, i32) {
        if let Some(tn) = node {
            let tnr = tn.borrow();

            let (avg_l, branches_l) = Solution::find_avg_subtrees(&tnr.left, cnt);
            let (avg_r, branches_r) = Solution::find_avg_subtrees(&tnr.right, cnt);

            let (avg, branches) = (avg_l + avg_r + tnr.val, branches_l + branches_r + 1);

            if avg / branches == tnr.val {
                *cnt += 1;
            }

            (avg, branches)
        } else {
            (0, 0)
        }
    }
}
