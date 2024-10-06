use crate::Solution;

impl Solution {
    pub fn longest_palindrome_dup(s: String) -> i32 {
        let cnt = s.bytes().fold([0; 52], |mut acc, n| {
            if n > 92 {
                acc[(n - b'a') as usize] += 1;
            } else {
                acc[(n - b'A' + 26) as usize] += 1;
            }
            acc
        });
        let mut ans = 0;
        let mut odd = false;

        for &c in cnt.iter() {
            if c & 1 != 0 {
                if odd {
                    ans += c - 1;
                } else {
                    ans += c;
                    odd = true;
                }
            } else {
                ans += c;
            }
        }

        ans
    }
}
