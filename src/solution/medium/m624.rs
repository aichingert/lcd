use crate::Solution;

impl Solution {
    pub fn max_distance_lc_1(arrays: Vec<Vec<i32>>) -> i32 {
        let (mut s, mut b) = (arrays[0][0], *arrays[0].last().unwrap());
        let mut ans = 0;

        for arr in arrays.iter().skip(1) {
            ans = ans.max(b - arr[0]).abs().max(arr[arr.len() - 1] - s);

            b = arr[arr.len() - 1].max(b);
            s = arr[0].min(s);
        }

        ans
    }
}
