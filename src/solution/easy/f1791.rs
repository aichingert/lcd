use crate::Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][1] == edges[1][0] {
            return edges[1][0];
        }

        edges[1][1]
    }
}
