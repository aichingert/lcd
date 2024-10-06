use crate::Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }

        let mut ans = Vec::new();
        let l = intervals.len();
        let mut i = 0;

        while i < intervals.len() {
            if intervals[i][0] >= new_interval[1] || intervals[i][0] >= new_interval[0] {
                intervals.insert(i, new_interval.clone());
                break;
            } else if intervals[i][0] >= new_interval[0] {
                intervals.insert(i + 1, new_interval.clone());
                break;
            }

            i += 1;
        }

        if l == intervals.len() {
            intervals.push(new_interval);
        }

        i = 0;
        let mut j = 0;

        while i < intervals.len() {
            if ans.is_empty() {
                ans.push(intervals[i].clone());
            } else if ans[j][1] < intervals[i][0] {
                ans.push(intervals[i].clone());
                j += 1;
            } else {
                ans[j] = vec![
                    ans[j][0].min(intervals[i][0]),
                    ans[j][1].max(intervals[i][1]),
                ];
            }

            i += 1;
        }

        ans
    }
}
