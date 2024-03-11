use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut m = HashMap::new();

        for c in s.chars() {
            m.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
        
        let mut ans = String::new();

        let mut i = 0;
        let chs = order.chars().collect::<Vec<_>>();

        while !m.is_empty() && i < chs.len() {
            if let Some(v) = m.get(&chs[i]) {
                for _ in 0..*v {
                    ans.push(chs[i]);
                }

                m.remove(&chs[i]);
            }

            i += 1;
        }

        for (k, v) in m {
            for _ in 0..v {
                ans.push(k);
            }
        }

        ans
    }
}
