use crate::Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        
        let mut ans = vec!(0; deck.len());
        let mut queue = VecDeque::new();

        for i in 0..deck.len() {
            queue.push_back(i);
        }

        for d in deck.iter() {
            ans[queue.pop_front().unwrap()] = *d;
            if let Some(nxt) = queue.pop_front() {
                queue.push_back(nxt);
            }
        }

        ans
    }
}
