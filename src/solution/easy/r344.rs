use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut [char]) {
        let n = s.len();
        for i in 0..n / 2 {
            s.swap(i, n - i - 1);
        }
    }
}
