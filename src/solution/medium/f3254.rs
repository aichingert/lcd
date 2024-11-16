use crate::Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }

        let mut a = vec![-1; (nums.len() + 1) - k as usize];
        let mut cnt = 1;

        for i in 0..nums.len() - 1 {
            if nums[i] + 1 == nums[i + 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }

            if cnt >= k {
                a[(i + 2) - k as usize] = nums[i + 1];
            }
        }

        a
    }
}
