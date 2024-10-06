use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut m = HashMap::new();

        for c in s.chars() {
            m.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut v = m.into_iter().collect::<Vec<_>>();
        v.sort_by(|a, b| b.1.cmp(&a.1));

        let mut ans = String::new();

        for (k, va) in v {
            for _ in 0..va {
                ans.push(k);
            }
        }

        ans
    }
}
