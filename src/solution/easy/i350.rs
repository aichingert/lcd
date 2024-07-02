use crate::Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut ans = Vec::new();

        let mut i = 0i32;
        let mut j = 0i32;

        while i < nums1.len() as i32 && j < nums2.len() as i32 {
            match nums1[i as usize].cmp(&nums2[j as usize]) {
                Ordering::Equal => ans.push(nums1[i as usize]),
                Ordering::Greater => i -= 1,
                Ordering::Less => j -= 1,
            }
            i += 1;
            j += 1;
        }

        ans
    }
}
