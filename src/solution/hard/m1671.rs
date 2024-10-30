use crate::Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut a = i32::MAX;
        let mut lis = vec![1; nums.len()];
        let mut lds = vec![1; nums.len()];

        for i in 0..nums.len() {
            for j in (0..i).rev() {
                if nums[i] > nums[j] {
                    lis[i] = lis[i].max(lis[j] + 1);
                }
            }
        }

        for i in (0..nums.len()).rev() {
            for j in i + 1..nums.len() {
                if nums[i] > nums[j] {
                    lds[i] = lds[i].max(lds[j] + 1);
                }
            }
        }

        for i in 0..nums.len() {
            if lis[i] > 1 && lds[i] > 1 {
                a = a.min((nums.len() - (lis[i] + lds[i] - 1)) as i32);
            }
        }

        a
    }
}
