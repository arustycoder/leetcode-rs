struct Solution;
impl Solution {
    fn first_uniq_char(s: String) -> i32 {
        let mut exists = vec![(0usize, 0usize); 26];
        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 'a' as usize;
            if exists[idx].0 == 0 {
                exists[idx].1 = i;
            }
            exists[idx].0 += 1;
        }

        let mut min = None;
        for e in exists.iter() {
            if e.0 == 1 && (min.is_none() || min.unwrap() > e.1 as i32) {
                min = Some(e.1 as i32);
            }
        }
        min.unwrap_or(0)
    }
}
fn main() {
    let s = "leetcode";
    println!("{}", Solution::first_uniq_char(s.to_owned()));
    let s = "loveleetcode";
    println!("{}", Solution::first_uniq_char(s.to_owned()));
}
