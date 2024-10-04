use crate::Solution;

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        skill.sort_unstable();
        let check = skill[0] + skill.last().unwrap();
        let mut chm = 0;

        for i in 0..skill.len() / 2 {
            if skill[i] + skill[skill.len() - i - 1] != check {
                return -1;
            }

            chm += (skill[i] * skill[skill.len() - i - 1]) as i64;
        }

        chm
    }
}
