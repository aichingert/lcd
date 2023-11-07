use crate::Solution;

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = dist.len();
        let mut arr = dist.iter().zip(speed).map(|(d, s)| (*d as f64 / s as f64).ceil() as i32).collect::<Vec<_>>();
        arr.sort_unstable();

        for i in 0..n {
            if arr[i] <= i as i32 {
                return i as i32;
            }
        }

        n as i32
    }
}
