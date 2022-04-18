struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let ss = s.as_bytes();
        let len = ss.len();
        let mut max = (0usize, 0usize);
        for (i, c) in ss.iter().enumerate() {
            if i == 0 || i == len - 1 {
                continue;
            }
            // odd case: bcb
            for l in 1..=i {
                if i + l < len && ss[i - l] == ss[i + l] {
                    if max.1 < l {
                        max.0 = i;
                        max.1 = l;
                    }
                    break;
                }
            }
            // even case: bb
        }
        println!("{:?}", max);
        String::from_utf8(ss[max.0 - max.1..max.0 + max.1 + 1].to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let s = "babad".to_string();
        assert_eq!(Solution::longest_palindrome(s), "bab".to_string());

        let s = "cbbd".to_string();
        assert_eq!(Solution::longest_palindrome(s), "bb".to_string());
    }
}
