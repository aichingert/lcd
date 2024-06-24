use crate::Solution;

impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut cflip = 0;
        let mut flips = 0;
        let k = k as usize;

        for i in 0..nums.len() {
            if i >= k && nums[i - k] == 2 {
                cflip -= 1;
            }

            if cflip & 1 == 1 && nums[i] == 1 || cflip & 1 == 0 && nums[i] == 0 {
                if i + k > nums.len() {
                    return -1;
                }

                nums[i] = 2;
                flips += 1;
                cflip += 1;
            }
        }

        flips
    }
}
