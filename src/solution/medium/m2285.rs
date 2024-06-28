use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut g = (0..n).map(|i| (i, 0)).collect::<Vec<_>>();
        let mut m = HashMap::new();
        let mut a = 0i64;

        for i in 0..roads.len() {
            g[roads[i][0] as usize].1 += 1;
            g[roads[i][1] as usize].1 += 1;
        }

        g.sort_unstable_by_key(|(_, b)| *b);

        for (i, e) in g.into_iter().enumerate() {
            m.insert(e.0, i as i64 + 1);
        }

        for road in roads {
            a += m[&road[0]] + m[&road[1]];
        }

        a
    }
}
