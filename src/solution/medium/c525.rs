use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let (mut sum, mut dis) = (0, 0);
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            if *n == 1 {
                sum += 1;
            } else {
                sum -= 1;
            }

            if sum == 0 {
                dis = dis.max(i as i32 + 1);
            }

            if let Some(l) = map.get(&sum) {
                dis = dis.max((i - l) as i32);
            } else {
                map.insert(sum, i);
            }
        }

        if sum == 0 {
            nums.len() as i32
        } else {
            dis
        }
    }
}
