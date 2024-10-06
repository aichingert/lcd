use crate::Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .filter(|s| s[11..=12].parse::<i32>().unwrap() > 60)
            .count() as i32
    }
}
