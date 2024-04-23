use crate::Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let n = n as usize;
        let mut g = vec![Vec::new(); n];
        let mut d = vec![0; n];

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            g[a].push(b);
            g[b].push(a);
            d[a] += 1;
            d[b] += 1;
        }

        let mut leaves = VecDeque::from_iter(d.iter().enumerate().filter(|(_, &e)| e == 1).map(|(i, _)| i));
        let mut remaining = n;

        // there can be one or two mht's
        while remaining > 2 {
            let len = leaves.len();
            remaining -= leaves.len();

            for _ in 0..len {
                if let Some(leave) = leaves.pop_front() {
                    for &ne in g[leave].iter() {
                        d[ne] -= 1;
                        if d[ne] == 1 {
                            leaves.push_back(ne);
                        }
                    }
                }
            }
        }

        leaves.into_iter().map(|e| e as i32).collect::<Vec<i32>>()
    }
}
