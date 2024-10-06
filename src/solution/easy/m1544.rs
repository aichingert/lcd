use crate::Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut ans = Vec::with_capacity(s.len());

        for cur in s.bytes() {
            if ans
                .last()
                .map(|&prv| prv + 32 == cur || cur + 32 == prv)
                .unwrap_or(false)
            {
                ans.pop();
            } else {
                ans.push(cur);
            }
        }

        ans.into_iter().map(|c| c as char).collect::<String>()
    }
}
