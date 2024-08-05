use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut c = HashMap::new();
        let mut d = 0;

        for a in arr.iter() {
            c.entry(a).and_modify(|n| *n += 1).or_insert(1);
        }

        for (i, a) in arr.iter().enumerate() {
            if c[&a] == 1 {
                d += 1;
            }

            if d == k {
                return arr[i].clone();
            }
        }

        "".to_string()
    }
}
