use crate::Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct S {
    p: i32,
    s: i32,
}

impl Ord for S {
    fn cmp(&self, other: &Self) -> Ordering {
        other.s.cmp(&self.s).then_with(|| other.p.cmp(&self.p))
    }
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut p = HashMap::new();
        let mut ans = vec![0; queries.len()];

        for i in 0..=n - 2 {
            p.insert(i, vec![i + 1]);
        }

        for (i, q) in queries.into_iter().enumerate() {
            p.get_mut(&q[0]).unwrap().push(q[1]);
            ans[i] = f(&p, n - 1);
        }

        ans
    }
}

fn f(g: &HashMap<i32, Vec<i32>>, t: i32) -> i32 {
    let mut vis = HashSet::new();
    let mut bfs = BinaryHeap::from([S { p: 0, s: 0 }]);

    while let Some(S { p, s }) = bfs.pop() {
        if !vis.insert(p) {
            continue;
        }

        if p == t {
            return s;
        }

        for &j in &g[&p] {
            bfs.push(S { p: j, s: s + 1 });
        }
    }

    unreachable!()
}
