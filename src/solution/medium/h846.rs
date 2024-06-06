use crate::Solution;
use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut b = BTreeMap::new();

        for h in hand {
            b.entry(h).and_modify(|n| *n += 1).or_insert(1);
        }

        while !b.is_empty() {
            let (&k, _) = b.first_key_value().unwrap();

            for i in 0..group_size {
                let key = k + i;

                if let Some(&v) = b.get(&key) {
                    if v == 1 {
                        b.remove(&key);
                    } else {
                        b.insert(key, v - 1);
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }
}
