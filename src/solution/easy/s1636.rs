use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort_lcd_dup_2(mut nums: Vec<i32>) -> Vec<i32> {
        let mut hm = HashMap::new();

        for &n in nums.iter() {
            hm.entry(n).and_modify(|x| *x += 1).or_insert(1);
        }
        
        nums.sort_unstable_by(|a, b| hm[a].cmp(&hm[b]).then_with(|| b.cmp(a)));
        nums
    }
}
