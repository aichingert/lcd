use crate::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let d = nums.iter().fold(0, |x, c| x ^ c);
        let mut c = 1;

        while d & c != c {
            c <<= 1;
        }

        nums.into_iter().fold(vec![0, 0], |mut x, n| {
            if n & c == 0 {
                x[0] ^= n;
            } else {
                x[1] ^= n;
            }

            x
        })
    }
}
