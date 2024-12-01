use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut n = HashSet::new();
        let mut d = HashSet::new();

        for a in arr {
            if d.contains(&a) || n.contains(&(2 * a)) {
                return true;
            }

            n.insert(a);
            d.insert(2 * a);
        }

        false
    }
}
