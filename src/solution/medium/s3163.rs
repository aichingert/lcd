use crate::Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut p = ' ';
        let mut c = 0u8;
        let mut a = String::new();

        for ch in word.chars() {
            if p == ' ' || p == ch {
                c += 1;

                if c == 9 {
                    a.push((c + b'0') as char);
                    a.push(p);
                    c = 0;
                }
            } else {
                if c > 0 {
                    a.push((c + b'0') as char);
                    a.push(p);
                }
                c = 1;
            }

            p = ch;
        }

        if c > 0 {
            a.push((c + b'0') as char);
            a.push(p);
        }

        a
    }
}
