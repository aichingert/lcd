use crate::Solution;

impl Solution {
    pub fn min_steps_lc_1(n: i32) -> i32 {
        f(1, 0, 0, n)
    }
}

fn f(c: i32, p: i32, s: i32, n: i32) -> i32 {
    if c > n {
        return i32::MAX;
    }
    if c == n {
        return s;
    }

    let paste = if p > 0 {
        f(c + p, p, s + 1, n)
    } else {
        i32::MAX
    };
    let copy = if c != p { f(c, c, s + 1, n) } else { i32::MAX };

    paste.min(copy)
}
