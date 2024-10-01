use crate::Solution;

use std::collections::HashMap;

// TODO: maybe figure out why this is so slow...
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut hs = HashMap::new();

        for mut cur in arr {
            cur = cur.rem_euclid(k);

            if cur == 0 {
                if hs.contains_key(&0) {
                    *hs.get_mut(&0).unwrap() -= 1;
                    if hs[&0] == 0 {
                        hs.remove(&0);
                    }

                } else {
                    hs.entry(0).and_modify(|n| *n += 1).or_insert(1);
                }
            } else {
                let key = k - cur;

                if hs.contains_key(&key) {
                    *hs.get_mut(&key).unwrap() -= 1;

                    if hs[&key] == 0 {
                        hs.remove(&key);
                    }
                } else {
                    hs.entry(cur).and_modify(|n| *n += 1).or_insert(1);
                }
            }
        }
        
        hs.is_empty()
    }
}
