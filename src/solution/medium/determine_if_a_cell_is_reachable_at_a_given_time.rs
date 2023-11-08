use crate::Solution;

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if sx == fx && sy == fy && t == 1 {
            return false;
        }

        let x = (fx - sx).abs();
        let y = (fy - sy).abs();

        x >= y && x <= t || x <= y && x + (y - x) <= t
    }
}
