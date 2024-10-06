use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut h = HashMap::new();

        for w in s1.split(' ') {
            h.entry(w).and_modify(|n| *n += 1).or_insert(1);
        }
        for w in s2.split(' ') {
            h.entry(w).and_modify(|n| *n += 1).or_insert(1);
        }

        h.into_iter()
            .filter(|(_, v)| *v == 1)
            .map(|(k, _)| k.to_string())
            .collect::<Vec<_>>()
    }
}
