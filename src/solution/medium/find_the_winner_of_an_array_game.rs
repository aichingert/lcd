use crate::Solution;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let (mut cur, mut wins) = (arr[0], 0);

        for i in 1..arr.len() {
            if arr[i] > cur {
                cur = arr[i];
                wins = 0;
            }

            wins += 1;
            if wins == k {
                return cur;
            }
        }

        cur
    }
}
