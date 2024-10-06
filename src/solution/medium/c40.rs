use crate::Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ans = Vec::new();

        f(&candidates, target, 0, Vec::new(), &mut ans);
        ans
    }
}

fn f(c: &[i32], t: i32, i: usize, p: Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if t < 0 {
        return;
    }
    if t == 0 {
        ans.push(p);
        return;
    }

    for n in i..c.len() {
        if n > i && c[n] == c[n - 1] {
            continue;
        }

        let mut cp = p.clone();
        cp.push(c[n]);
        f(c, t - c[n], n + 1, cp, ans);
    }
}
