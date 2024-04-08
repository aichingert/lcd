use crate::Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut v = [0; 2];
        for s in students {
            v[s as usize] += 1;
        }

        for s in sandwiches {
            let cur = &mut v[s as usize];
            if cur == &0 {
                return v[1 - s as usize];
            }
            *cur -= 1;
        }

        0
    }
}
