use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut loses = HashMap::new();

        for game in matches.into_iter() {
            loses.entry(game[0]).and_modify(|a| *a += 0).or_insert(0);
            loses.entry(game[1]).and_modify(|n| *n += 1).or_insert(1);
        }

        let (mut zero, mut one) = (Vec::new(), Vec::new());

        for (k, v) in loses.into_iter() {
            match v {
                0 => zero.push(k),
                1 => one.push(k),
                _ => (),
            }
        }

        zero.sort_unstable();
        one.sort_unstable();

        vec![zero, one]
    }
}
