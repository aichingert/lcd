use crate::Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let (k, mut ans) = (k as usize, 0);

        for (i, t) in tickets.iter().enumerate() {
            let o = if i <= k { 0 } else { 1 };
            ans += (tickets[k] - o).min(*t);
        }

        ans
    }
}
