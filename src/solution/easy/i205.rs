use crate::Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut m = HashMap::new();
        let mut e = HashSet::new();

        for (cs, ct) in s.chars().zip(t.chars()) {
            if let Some(ex) = m.get(&cs) {
                if ex != &ct {
                    return false;
                }
            } else {
                if e.contains(&ct) {
                    return false;
                }
                e.insert(ct);
                m.insert(cs, ct);
            }
        }

        true
    }
}
