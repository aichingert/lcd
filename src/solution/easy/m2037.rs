use crate::Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();

        (0..seats.len())
            .map(|i| (seats[i] - students[i]).abs())
            .sum()
    }
}
