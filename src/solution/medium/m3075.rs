use crate::Solution;

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable();
        let mut ans = 0;

        for i in 0..k {
            let cur = 0.max(happiness[happiness.len() - i as usize - 1] - i);
            if cur == 0 {
                break;
            }
            ans += cur as i64;
        }

        ans
    }
}
