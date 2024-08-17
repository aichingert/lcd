use crate::Solution;

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_unstable();
        let mut b = vec![0; (nums[nums.len() - 1] - nums[0]) as usize + 1];

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                b[(nums[i] - nums[j]).unsigned_abs() as usize] += 1;
            }
        }

        for (i, c) in b.into_iter().enumerate() {
            k -= c;
            if k <= 0 {
                return i as i32;
            }
        }

        0
    }
}
