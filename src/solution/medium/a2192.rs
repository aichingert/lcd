use crate::Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![Vec::new(); n as usize];
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in edges {
            g.entry(edge[1])
                .and_modify(|v| v.push(edge[0]))
                .or_insert(vec![edge[0]]);
        }

        for i in 0..n {
            let mut bfs = vec![i];
            let mut vis = HashSet::new();

            while let Some(key) = bfs.pop() {
                if !vis.insert(key) {
                    continue;
                }

                if let Some(res) = g.get(&key) {
                    ans[i as usize].extend_from_slice(res);
                    bfs.extend_from_slice(res);
                }
            }

            ans[i as usize].sort_unstable();
            ans[i as usize].dedup();
        }

        ans
    }
}
