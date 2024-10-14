use crate::Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut h = BinaryHeap::new();
        let mut a = 0;

        for num in nums {
            h.push(num as i64);
        }

        for _ in 0..k {
            let cur = h.pop().unwrap();
            if cur <= 0 {
                break;
            }
            a += cur;
            h.push((cur as f64 / 3.0).ceil() as i64);
        }

        a
    }
}
