use crate::Solution;

impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        for i in 0..nums.len() - 1 {
            if nums[i] <= nums[i + 1] {
                continue;
            }

            if nums[i].count_ones() != nums[i + 1].count_ones() {
                return false;
            }

            (nums[i], nums[i + 1]) = (nums[i + 1], nums[i]);
        }

        for i in (1..nums.len()).rev() {
            if nums[i] >= nums[i - 1] {
                continue;
            }

            if nums[i].count_ones() != nums[i - 1].count_ones() {
                return false;
            }

            (nums[i], nums[i - 1]) = (nums[i - 1], nums[i]);
        }

        true
    }
}
