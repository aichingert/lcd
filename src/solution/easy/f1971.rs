use crate::Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn valid_path(_n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut s = HashSet::new();
        let mut v = vec![source];

        for edge in edges {
            g.entry(edge[0]).and_modify(|v| v.push(edge[1])).or_insert(vec![edge[1]]);
            g.entry(edge[1]).and_modify(|v| v.push(edge[0])).or_insert(vec![edge[0]]);
        }

        while let Some(c) = v.pop() {
            if c == destination {
                return true;
            }
            if !s.insert(c) {
                continue;
            }

            v.extend_from_slice(g.get(&c).unwrap());
        }

        false
    }
}
