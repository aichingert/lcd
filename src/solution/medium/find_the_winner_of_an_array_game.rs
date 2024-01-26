use crate::Solution;

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let (mut cur, mut wins) = (arr[0], 0);

        for item in arr.into_iter().skip(1) {
            if item > cur {
                cur = item;
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
