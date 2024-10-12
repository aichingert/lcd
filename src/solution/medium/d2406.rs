use crate::Solution;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut n = Vec::new();
        let mut a = 0;
        let mut c = 0;

        for int in intervals {
            n.push((int[0], 1));
            n.push((int[1] + 1, -1));
        }

        n.sort_unstable();

        for (_, v) in n {
            c += v;
            a = a.max(c);
        }

        a
    }
}
