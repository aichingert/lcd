use crate::Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if bloom_day.len() as i32 / k < m {
            return -1;
        }

        let (mut start, mut end) = (0, *bloom_day.iter().max().unwrap());
        let mut ans = -1;

        while start <= end {
            let mid = (start + end) / 2;

            if Self::boquets(&bloom_day, mid, k) >= m {
                ans = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        ans
    }

    fn boquets(b: &[i32], mid: i32, k: i32) -> i32 {
        let (mut bq, mut ct) = (0, 0);

        for &c in b {
            if c > mid {
                ct = 0;
            } else {
                ct += 1;
            }

            if ct == k {
                bq += 1;
                ct = 0;
            }
        }

        bq
    }
}
