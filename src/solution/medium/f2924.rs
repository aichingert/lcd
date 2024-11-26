use crate::Solution;

use std::collections::HashSet;

// TODO: optimize this, I believe you should use the length of w and s
// or maybe something else but this can definetly improved
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut w = HashSet::new();
        let mut s = HashSet::new();

        for edge in edges {
            w.insert(edge[1]);
            s.remove(&edge[1]);

            if !w.contains(&edge[0]) {
                s.insert(edge[0]);
            }
        }

        for i in 0..n {
            if !w.contains(&i) {
                s.insert(i);
            }
        }

        let mut a = -1;

        if s.len() == 1 {
            a = s.into_iter().next().unwrap();
        }

        a
    }
}
