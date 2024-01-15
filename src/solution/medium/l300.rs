use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut long = vec![nums[0]];

        for num in nums.into_iter().skip(1) {
            if *long.last().unwrap() < num {
                long.push(num);
            } else if let Err(p) = long.binary_search(&num) {
                long[p] = num;
            }
        }

        long.len() as i32
    }
}
