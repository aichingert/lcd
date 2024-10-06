use crate::Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let s = *nums.iter().max().unwrap();
        let (mut ans, mut sen) = (0, 0);

        let (mut l, mut r) = (0, 0);

        while r < nums.len() {
            if nums[r] == s {
                sen += 1;
            }

            while l <= r && sen >= k {
                if nums[l] == s {
                    sen -= 1;
                }
                l += 1;
            }

            ans += l;
            r += 1;
        }

        ans as i64
    }
}
