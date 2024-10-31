use crate::Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable_by_key(|k| k[0]);

        let mut fp = Vec::new();

        for f in factory {
            for _ in 0..f[1] {
                fp.push(f[0]);
            }
        }

        let mut dp = vec![vec![-1; fp.len()]; robot.len()];
        sol(&mut dp, &robot, &fp)
    }
}

fn sol(dp: &mut Vec<Vec<i64>>, r: &[i32], fp: &[i32]) -> i64 {
    if r.is_empty() {
        return 0;
    }
    if fp.is_empty() {
        return 1e12 as i64;
    }
    let (rl, fl) = (r.len() - 1, fp.len() - 1);

    if dp[rl][fl] != -1 {
        return dp[rl][fl];
    }

    let o = (fp[0] - r[0]).abs() as i64 + sol(dp, &r[1..], &fp[1..]);
    let s = sol(dp, r, &fp[1..]);

    dp[rl][fl] = s.min(o);
    dp[rl][fl]
}
