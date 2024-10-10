use crate::Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut s = Vec::new();
        let mut m = 0;

        for (i, n) in nums.iter().enumerate() {
            if s.is_empty() || nums[*s.last().unwrap()] > *n {
                s.push(i);
            }
        }

        for (i, n) in nums.iter().enumerate().rev() {
            while !s.is_empty() && nums[*s.last().unwrap()] <= *n {
                m = m.max(i - s.pop().unwrap());
            }
        }

        m as i32
    }
}
