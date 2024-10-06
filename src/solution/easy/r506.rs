use crate::Solution;

impl Solution {
    pub fn find_relative_ranks(scores: Vec<i32>) -> Vec<String> {
        let mut sorted = scores.clone();
        sorted.sort_unstable();

        let mut ans = Vec::with_capacity(scores.len());

        for score in scores {
            ans.push(match sorted.binary_search(&score) {
                Ok(loc) => match sorted.len() - loc {
                    1 => "Gold Medal".to_string(),
                    2 => "Silver Medal".to_string(),
                    3 => "Bronze Medal".to_string(),
                    num => num.to_string(),
                },
                Err(_) => panic!(),
            });
        }

        ans
    }
}
