use crate::Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let (mut qsize, mut gap) = (0, 0);
        let (mut start, mut ans) = (0, 0);

        for i in 0..nums.len() {
            if nums[i] & 1 == 1 {
                qsize += 1;
            }

            if qsize == k {
                gap = 0;
                while qsize == k {
                    if nums[start] & 1 == 1 {
                        qsize -= 1;
                    }
        
                    gap += 1;
                    start += 1;
                }
            }

            ans += gap
        }

        ans
    }
}
