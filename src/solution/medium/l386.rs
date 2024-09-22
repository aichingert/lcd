use crate::Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut c = 1;
        let mut v = Vec::with_capacity(n as usize);

        for _ in 0..n {
            v.push(c);

            if c * 10 <= n {
                c *= 10;
            } else {
                while c % 10 == 9 || c >= n {
                    c /= 10;
                }

                c += 1;

            }

        }

        v
    }
}
