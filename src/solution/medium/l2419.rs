use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn longest_subarray_lc_1(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut l = 0;
        let mut b = 0;

        for num in nums {
            match num.cmp(&b) {
                Ordering::Less => {
                    l = l.max(a);
                    a = 0;
                }
                Ordering::Equal => a += 1,
                Ordering::Greater => {
                    a = 1;
                    l = 1;
                    b = num;
                }
            }
        }

        l.max(a)
    }
}
