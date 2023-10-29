/* Daily problem: 2023-10-29
 * aichingert
 */

use crate::Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let t = (minutes_to_test / minutes_to_die) + 1;
        let mut x = 0;

        while t.pow(x) < buckets {
            x += 1;
        }

        x as i32
    }
}
