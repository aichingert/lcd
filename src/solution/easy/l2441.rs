use crate::Solution;

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let (mut i, mut j) = (0, nums.len() - 1);
        
        while i != j {
            if nums[i] > 0 {
                return - 1;
            }

            if -nums[i] == nums[j] {
                return nums[j];
            }

            if -nums[i] > nums[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        
        -1
    }
}
