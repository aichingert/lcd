use crate::Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut zip = names.into_iter().zip(heights).collect::<Vec<_>>();
        zip.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        zip.into_iter().map(|a| a.0).collect()
    }
}
