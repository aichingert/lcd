use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut hm = HashMap::new();

        for t in target {
            hm.entry(t).and_modify(|n| *n += 1).or_insert(1);
        }
        for a in arr {
            let Some(e) = hm.get_mut(&a) else {
                return false;
            };

            if *e == 0 {
                return false;
            }

            *e -= 1;
        }

        true
    }
}
