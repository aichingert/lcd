use crate::Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut s1 = (n * (n + 1)) / 2;
        let mut s2 = 0;

        for i in 1..n + 1 {
            s2 += i;

            if s1 == s2 {
                return i;
            }

            s1 -= i;
        }

        -1
    }
}
