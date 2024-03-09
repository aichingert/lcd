use std::cmp::Ordering;

use crate::Solution;

impl Solution {
    pub fn get_common(n: Vec<i32>, m: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);

        while i < n.len() && j < m.len() {
            match n[i].cmp(&m[j]) {
                Ordering::Less => i += 1,
                Ordering::Equal => return n[i],
                Ordering::Greater => j += 1,
            }
        }

        -1
    }
}
