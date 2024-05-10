use crate::Solution;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        for a in arr.iter() {
            for b in arr.iter() {
                ans.push(((*a as f64) / (*b as f64), a, b));
            }
        }

        ans.sort_by(|(da,_,_),(db,_,_)| da.partial_cmp(db).unwrap());
        vec![*ans[k as usize - 1].1, *ans[k as usize - 1].2]
    }
}
