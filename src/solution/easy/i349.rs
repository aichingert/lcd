use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        HashSet::<i32>::from_iter(nums1)
            .intersection(&HashSet::<i32>::from_iter(nums2)).copied()
            .collect()
    }
}

