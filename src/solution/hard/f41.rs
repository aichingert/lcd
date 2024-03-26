use crate::Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        
        for i in 0..nums.len() {
            let mut x = nums[i];

            while x >= 1 && x <= n && x as usize != i + 1 && nums[x as usize - 1] != x {
                let tmp = nums[x as usize - 1];
                nums[x as usize - 1] = nums[i];
                nums[i] = tmp;
                x = nums[i];
            }
        }

        for i in 0..nums.len() {
            let x = (i + 1) as i32;
            if nums[i] != x {
                return x;
            }
        }
        
        n + 1
    }
}
