use crate::Solution;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|a, b| {
            (format!("{a}{b}").parse::<usize>().unwrap())
                .cmp(&format!("{b}{a}").parse::<usize>().unwrap())
        });

        let mut res = nums.into_iter().fold(Vec::new(), |mut acc, n| {
            acc.push(n.to_string());
            acc
        });

        while res.len() > 1 && &res[res.len() - 1] == "0" {
            res.pop();
        }

        res.into_iter().rev().fold(String::new(), |mut acc, s| {
            acc.push_str(&s);
            acc
        })
    }
}
