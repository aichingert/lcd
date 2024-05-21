use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = sub(&nums, vec![]);
        ans.push(vec![]);
        ans
    }
}

fn sub(xs: &[i32], cur: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();

    for i in 0..xs.len() {
        let mut cl = cur.clone();
        cl.push(xs[i]);
        ans.push(cl.clone());
        for rem in sub(&xs[i + 1..], cl) {
            ans.push(rem);
        }
    }

    ans
}
