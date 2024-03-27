use crate::Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut ans = Vec::new();

        for point in points.into_iter() {
            if !merge(&mut ans, &point) {
                ans.push(point)
            }
        }

        ans.len() as i32
    }
}

fn merge(vec: &mut [Vec<i32>], ins: &[i32]) -> bool {
    for rng in vec.iter_mut() {
        if rng[1] == ins[0] || rng[1] >= ins[0] && ins[1] >= rng[1] {
            rng[0] = rng[0].max(ins[0]);
            return true;
        } else if rng[0] == ins[1] || rng[0] >= ins[0] && rng[1] >= ins[1] && rng[0] <= ins[1] {
            rng[1] = ins[1];
            return true;
        } else if rng[0] <= ins[0] && rng[1] >= ins[1] {
            rng[0] = ins[0];
            rng[1] = ins[1];
            return true;
        }
    }

    false
}
