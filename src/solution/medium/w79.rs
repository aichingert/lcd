use crate::Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let w = word.chars().collect::<Vec<char>>();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == w[0] && bktk((i as i32, j as i32, 0), &mut board, &w) {
                    return true;
                }
            }
        }

        false
    }
}

fn bktk((i, j, k): (i32, i32, usize), b: &mut [Vec<char>], w: &[char]) -> bool {
    if k == w.len() {
        return true;
    }
    if i < 0 || j < 0 || i >= b.len() as i32 || j >= b[0].len() as i32 || b[i as usize][j as usize] != w[k] {
        return false;
    }

    let tmp = b[i as usize][j as usize];
    b[i as usize][j as usize] = '\0';
    let k = k + 1;

    if bktk((i + 1, j, k), b, w) || bktk((i, j + 1, k), b, w) 
    || bktk((i - 1, j, k), b, w) || bktk((i, j - 1, k), b, w) {
        return true;
    }

    b[i as usize][j as usize] = tmp;

    false
}
