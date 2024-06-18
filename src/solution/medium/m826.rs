use crate::Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut zip = difficulty.into_iter().zip(profit).collect::<Vec<_>>();
        zip.sort_by_key(|&(d, _)| d);
        let mut ans = 0;

        for w in worker.into_iter() {
            let mut g = 0;

            for &(d, p) in &zip {
                if d > w { break; }
                g = g.max(p);
            }

            ans += g;
        }

        ans
    }
}
