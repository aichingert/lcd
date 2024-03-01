use crate::Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let ns = s.bytes().map(|n| n - b'0').sum::<u8>();
        let mut ans = String::new();

        for _ in 0..ns - 1 {
            ans.push('1');
        }

        for _ in ns - 1..s.len() as u8 - 1{
            ans.push('0');
        }
        ans.push('1');
        ans
    }
}
