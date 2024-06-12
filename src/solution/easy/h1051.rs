use crate::Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut ex = heights.clone();
        ex.sort_unstable();

        heights.into_iter().zip(ex)
            .filter(|(h, e)| h != e)
            .count() as i32
    }
}
