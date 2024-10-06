use crate::Solution;

use std::collections::{HashMap, HashSet};

// TODO: not optimal fix, when I fully understand disjoint union find
impl Solution {
    pub fn remove_stones_lc_1(stones: Vec<Vec<i32>>) -> i32 {
        let mut gx: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut gy: HashMap<i32, Vec<i32>> = HashMap::new();

        for stone in &stones {
            gy.entry(stone[0])
                .and_modify(|v| v.push(stone[1]))
                .or_insert(vec![stone[1]]);
            gx.entry(stone[1])
                .and_modify(|v| v.push(stone[0]))
                .or_insert(vec![stone[0]]);
        }

        let mut v = HashSet::new();
        let mut r = 0;

        for stone in stones.iter() {
            if v.contains(&(stone[0], stone[1])) {
                continue;
            }

            let mut s = vec![(stone[0], stone[1])];
            r += 1;

            while let Some((fy, fx)) = s.pop() {
                if !v.insert((fy, fx)) {
                    continue;
                }

                for &y in gx[&fx].iter() {
                    s.push((y, fx));
                }
                for &x in gy[&fy].iter() {
                    s.push((fy, x));
                }
            }
        }

        (stones.len() - r) as i32
    }
}
