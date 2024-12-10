use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let c = s.chars().collect::<Vec<_>>();
        let mut h: HashMap<char, Vec<i32>> = HashMap::new();
        let mut a = -1;
        let mut l = 1;

        for i in 0..c.len() - 1 {
            if c[i] == c[i + 1] {
                l += 1;
            } else {
                h.entry(c[i]).and_modify(|v| v.push(l)).or_insert(vec![l]);
                l = 1;
            }
        }
        h.entry(c[c.len() - 1])
            .and_modify(|v| v.push(l))
            .or_insert(vec![l]);

        for v in h.values_mut() {
            let l = v.len();
            v.sort_unstable();

            if l > 2 {
                a = a.max(v[l - 3]);
            }
            if l > 1 && v[l - 1] > 1 {
                a = a.max(v[l - 2].min(v[l - 1] - 1));
            }
            if v[l - 1] > 2 {
                a = a.max(v[l - 1] - 2);
            }
        }

        a
    }
}
