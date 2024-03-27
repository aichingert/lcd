use crate::Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        
        for i in 0..nums.len() {
            let mut x = nums[i];

            while x >= 1 && x <= n && x as usize != i + 1 && nums[x as usize - 1] != x {
                nums.swap(x as usize - 1, i);
                x = nums[i];
            }
        }

        for (i, num) in nums.iter().enumerate() {
            let x = (i + 1) as i32;
            if *num != x {
                return x;
            }
        }
        
        n + 1
    }
}
