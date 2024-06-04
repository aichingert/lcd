use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let cnt = s.bytes().fold([0; 52], |mut acc, n| {if n > 92 { acc[(n - b'a') as usize] += 1; } else { acc[(n - b'A' + 26) as usize] += 1; } acc});
        let mut ans = 0;
        let mut odd = false;

        for i in 0..cnt.len() {
            if cnt[i] & 1 != 0 {
                if odd {
                    ans += cnt[i] - 1;
                } else {
                    ans += cnt[i];
                    odd = true;
                }
            } else {
                ans += cnt[i];
            }
        }

        ans
    }
}
