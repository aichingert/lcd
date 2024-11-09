use crate::Solution;

impl Solution {
    pub fn min_end(mut n: i32, x: i32) -> i64 {
        let mut x = x as i64;
        let mut cnt = 0;
        let mut ans = 0;

        for i in 0..64 {
            if 1 << i & x == 0 {
                cnt += 1;

                if 2i64.pow(cnt) >= n as i64 {
                    break;
                }
            }
        }

        (n, cnt) = (n - 1, 0);
        let mut b = 0;

        for i in 0..64 {
            if 1 << i & n != 0 {
                b |= 1 << i;
            }

            if b == n {
                break;
            }
        }

        while b > 0 || x > 0 {
            if x & 1 != 0 {
                ans |= 1 << cnt;
            } else {
                if b & 1 != 0 {
                    ans |= 1 << cnt;
                }
                b >>= 1;
            }

            x >>= 1;
            cnt += 1;
        }

        ans
    }
}
