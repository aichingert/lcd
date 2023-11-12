use std::collections::{HashSet, VecDeque};

use crate::Solution;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut bfs = VecDeque::new();
        let mut q = HashSet::new();

        for i in 0..routes.len() {
            if routes[i].binary_search(&source).is_ok() {
                bfs.push_front((i, 1));
                q.insert(i);
            }
        }

        if source == target {
            return if bfs.len() > 0 { 0 } else { -1 };
        }

        while let Some((idx, d)) = bfs.pop_front() {
            if routes[idx].binary_search(&target).is_ok() {
                return d;
            }
            let d = d + 1;

            'outer: for i in 0..routes.len() {
                if !q.contains(&i) {
                    for j in 0..routes[idx].len() {
                        if routes[i].binary_search(&routes[idx][j]).is_ok() {
                            q.insert(i);
                            bfs.push_back((i, d));
                            continue 'outer;
                        }
                    }

                }
            }
        }

        -1
    }
}
