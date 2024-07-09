use crate::Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut ans = 0;
        let mut cur_t = 0;

        for ts in customers.iter() {
            if cur_t < ts[0] as i64 {
                cur_t = ts[0] as i64;
            } else {
                ans += cur_t - ts[0] as i64;
            }

            ans += ts[1] as i64;
            cur_t += ts[1] as i64;
        }
        
        ans as f64 / customers.len() as f64
    }
}
