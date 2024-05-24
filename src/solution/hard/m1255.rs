use crate::Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut l = [0; 26];
        for c in letters {
            l[(c as u8 - b'a') as usize] += 1;
        }

        cnt(&words, 0, l, &score, 0)
    }
}

fn cnt(w: &[String], i: usize, mut c: [i32; 26], l: &[i32], mut s: i32) -> i32 {
    if i >= w.len() {
        return s;
    }

    let mut ans = cnt(w, i + 1, c, l, s);
    let mut val = true;

    for ch in w[i].chars().map(|c| (c as u8 - b'a') as usize) {
        if c[ch] == 0 {
            val = false;
            break;
        }

        c[ch] -= 1;
        s += l[ch];
    }

    if val {
        ans = ans.max(cnt(w, i + 1, c, l, s));
    }

    ans
}
