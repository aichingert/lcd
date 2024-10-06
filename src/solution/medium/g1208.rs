use crate::Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let d = s
            .chars()
            .zip(t.chars())
            .map(|(si, ti)| (((si as u8) as i32) - ((ti as u8) as i32)).abs())
            .collect::<Vec<_>>();

        let (mut l, mut r, mut le) = (0, 0, 0);
        let mut co = 0;

        while r < s.len() {
            while co > max_cost && l < r {
                co -= d[l];
                l += 1;
            }

            co += d[r];
            r += 1;

            if co <= max_cost {
                le = le.max(r - l);
            }
        }

        le as i32
    }
}
