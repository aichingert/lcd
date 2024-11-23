use crate::Solution;

impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut a = vec![vec!['.'; b.len()]; b[0].len()];

        for (i, chs) in b.iter().enumerate() {
            for (j, am) in a.iter_mut().enumerate().take(chs.len()) {
                let len = am.len();
                am[len - i - 1] = chs[j];

                //let len = a[j].len();
                //a[j][len - i - 1] = b[i][j];
            }
        }

        for i in (0..a.len()).rev() {
            for j in 0..a[i].len() {
                if a[i][j] == '#' {
                    let mut p = i;
                    while p + 1 < a.len() && a[p + 1][j] == '.' {
                        a[p][j] = '.';
                        p += 1;
                        a[p][j] = '#';
                    }
                }
            }
        }

        a
    }
}
