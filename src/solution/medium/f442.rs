use crate::Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();

        for i in 0..nums.len() {
            let j = (nums[i].abs() - 1) as usize;
            nums[j] = -nums[j];

            if nums[j]>0 {
                ans.push(nums[i].abs());
            }
        }

        ans
    }
}
