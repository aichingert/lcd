use crate::Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let ans = if let Some(max) = left.iter().max() { *max } else { 0 };

        if let Some(max) = right.iter().map(|x| (n - x).abs()).max() {
            ans.max(max)
        } else {
            ans
        }
    }
}
