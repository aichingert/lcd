use crate::Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 { return 0; }
    
        let mut ans = 0;

        let (mut l, mut r) = (0, 0);
        let mut p = 1;

        while r < nums.len() {
            p *= nums[r];

            while l <= r && p >= k {
                p /= nums[l];
                l += 1;
            }

            ans += r - l + 1;
            r += 1;
        }

        ans as i32
    }
}
