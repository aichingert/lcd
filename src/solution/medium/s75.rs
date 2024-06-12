use crate::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let (mut z, mut t) = (0, nums.len() - 1);
        let mut i = 0;

        while i <= t {
            match nums[i] {
                0 => {
                    nums.swap(z, i);
                    z += 1;
                    i += 1;
                }
                1 => i += 1,
                2 => {
                    nums.swap(t, i);
                    if t == 0 { break; }
                    t -= 1;
                }
                _ => ()
            }
        }
    }
}
