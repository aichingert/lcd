use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut m: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            let k = ((n as f32).sqrt()) as i32;

            if k * k != n {
                m.insert(n, 1);
                continue;
            }

            if let Some(c) = m.get(&((n as f32).sqrt() as i32)) {
                m.insert(n, c + 1);
            } else {
                m.insert(n, 1);
            }
        }

        let Some(ans) = m
            .values()
            .max()
            .and_then(|&n| if n == 1 { None } else { Some(n) })
        else {
            return -1;
        };

        ans
    }
}
