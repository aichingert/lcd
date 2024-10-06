use crate::Solution;
// NOTE: this is not a good solution (=

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut set = HashSet::new();
        let mut bfs = VecDeque::from_iter([("0000".to_string(), 0)]);

        while let Some((cur, spins)) = bfs.pop_front() {
            if cur == target {
                return spins;
            }
            if set.contains(&cur) || deadends.contains(&cur) {
                continue;
            }

            let nxt = cur.chars().collect::<Vec<_>>();

            for (i, c) in nxt.iter().enumerate() {
                let mut nps = nxt.clone();

                nps[i] = (*c as u8 + 1) as char;
                if (nps[i] as u8) > b'9' {
                    nps[i] = '0';
                }
                bfs.push_back((nps.iter().collect::<String>(), spins + 1));
                nps[i] = (*c as u8 - 1) as char;
                if (nps[i] as u8) < b'0' {
                    nps[i] = '9';
                }
                bfs.push_back((nps.iter().collect::<String>(), spins + 1));
            }

            set.insert(cur);
        }

        -1
    }
}
