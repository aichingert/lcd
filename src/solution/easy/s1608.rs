use crate::Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        for i in 0..=nums.len() as i32 {
            let mut cnt = 0;
            for &n in nums.iter() {
                if n >= i {
                    cnt += 1;
                }
            }

            if cnt == i {
                return i;
            }
        }

        -1
    }
}
