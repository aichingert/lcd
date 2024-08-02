use crate::Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut total = 0;

        for &n in nums.iter() {
            if n == 1 {
                total += 1;
            }
        }

        let mut i = 0;
        let mut c = 0;

        for &n in nums.iter().take(total) {
            if n == 1 {
                c += 1;
            }
        }

        let mut ans = i32::MAX;

        while i < nums.len() {
            ans = ans.min((total - c) as i32);

            if nums[(i + total) % nums.len()] == 1 {
                c += 1;
            }
            if nums[i] == 1 {
                c -= 1;
            }
            i += 1;
        }

        ans
    }
}
