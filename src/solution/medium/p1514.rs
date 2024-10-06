use crate::Solution;

use std::collections::{HashMap, HashSet};

// TODO: use binary heap instead of searching the smallest in the vec
impl Solution {
    pub fn max_probability(
        _n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let mut g: HashMap<i32, Vec<(i32, f64)>> = HashMap::new();

        for (i, edge) in edges.into_iter().enumerate() {
            g.entry(edge[0])
                .and_modify(|v| v.push((edge[1], succ_prob[i])))
                .or_insert(vec![(edge[1], succ_prob[i])]);
            g.entry(edge[1])
                .and_modify(|v| v.push((edge[0], succ_prob[i])))
                .or_insert(vec![(edge[0], succ_prob[i])]);
        }

        let mut q = Vec::from([(start_node, 1.0)]);
        let mut s = HashSet::new();

        while !q.is_empty() {
            let u = {
                let (mut pos, mut cos): (usize, f64) = (0, 0.);

                for (i, p) in q.iter().enumerate() {
                    if cos < p.1 {
                        cos = p.1;
                        pos = i;
                    }
                }

                q.remove(pos)
            };

            if u.0 == end_node {
                return u.1;
            }

            if !g.contains_key(&u.0) {
                continue;
            }

            for e in g[&u.0].iter() {
                if !s.insert((e.0, u.0)) {
                    continue;
                }
                q.push((e.0, e.1 * u.1));
            }
        }

        0.
    }
}
