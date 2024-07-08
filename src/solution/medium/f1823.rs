use crate::Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut ans = 0;

        for people in 1..=n {
            ans = (ans + k) % people
        }

        ans + 1
    }
}
