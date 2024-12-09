use crate::Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut a = vec![false; queries.len()];
        let mut p = vec![0; nums.len()];

        for i in 1..nums.len() {
            if nums[i - 1] & 1 == nums[i] & 1 {
                p[i] = p[i - 1] + 1;
            } else {
                p[i] = p[i - 1];
            }
        }

        for (i, q) in queries.into_iter().enumerate() {
            if p[q[1] as usize] - p[q[0] as usize] == 0 {
                a[i] = true;
            }
        }

        a
    }
}
