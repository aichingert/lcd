use crate::Solution;

// TODO: there is a O(1) math solution
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut pos = 1;
        let mut dir = 1;

        for i in 0..time {
            if (i > 0 && pos == 1) || pos == n {
                dir *= -1;
            }

            pos += dir;
        }

        pos
    }
}
