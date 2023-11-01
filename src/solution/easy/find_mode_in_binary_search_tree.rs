/* Daily problem: 2023-11-01
 * aichingert
 */

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::Solution;
use crate::TreeNode;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut max = 0;

        Solution::find_leaf_mode(&root, &mut map, &mut max);

        map
            .into_iter()
            .filter(|(_, v)| *v == max)
            .map(|(k, _)| k)
            .collect::<Vec<_>>()
    }

    fn find_leaf_mode(node: &Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>, max: &mut i32) {
        if let Some(tree_node) = node {
            let tree_node_ref = tree_node.borrow();
            map.entry(tree_node_ref.val).and_modify(|val| *val += 1).or_insert(1);
            *max = *max.max(map.get_mut(&tree_node_ref.val).unwrap());

            Solution::find_leaf_mode(&tree_node_ref.left, map, max);
            Solution::find_leaf_mode(&tree_node_ref.right, map, max);
        }
    }
}
