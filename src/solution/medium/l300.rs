use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut long = vec![nums[0]];

        for i in 1..nums.len() {
            if *long.last().unwrap() < nums[i] {
                long.push(nums[i]);
            } else {
                if let Err(p) = long.binary_search(&nums[i]) {
                    long[p] = nums[i];
                }
            }
        }

        long.len() as i32
    }
}
