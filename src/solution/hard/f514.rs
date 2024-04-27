use crate::Solution;

use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut s = HashSet::<(i32, i32)>::new();
        let r = ring.chars().collect::<Vec<_>>();
        let k = key.chars().collect::<Vec<_>>();
        
        let mut bfs = VecDeque::from([(0, 0, 0)]);

        while let Some((rp, kp, ans)) = bfs.pop_front() {
            if !s.insert((rp, kp)) {
                continue;
            }
            if kp >= k.len() as i32 {
                return ans;
            }

            if r[rp as usize] == k[kp as usize] {
                bfs.push_back((rp, kp + 1, ans + 1));
            } else {
                bfs.push_back(((rp + 1i32).rem_euclid(r.len() as i32), kp, ans + 1));
                bfs.push_back(((rp - 1i32).rem_euclid(r.len() as i32), kp, ans + 1));
            }
        }

        -1
    }
}
