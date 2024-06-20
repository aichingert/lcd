use crate::Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();

        let (mut start, mut end) = (0, *position.last().unwrap());
        let mut ans = 0;

        while start <= end {
            let dist = (end + start) / 2;

            if Self::place(&position, dist) >= m {
                ans = ans.max(dist);
                start = dist + 1;
            } else {
                end = dist - 1;
            }
        }

        ans
    }

    fn place(p: &[i32], d: i32) -> i32 {
        let mut ans = 1;
        let mut cur = p[0];

        for pos in p.iter().skip(1) {
            if pos - cur >= d {
                ans += 1;
                cur = *pos;
            }
        }

        ans
    }
}
