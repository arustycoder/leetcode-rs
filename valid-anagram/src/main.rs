struct Solution;

impl Solution {
    fn is_anagram(s: String, t: String) -> bool {
        let mut exists = std::collections::HashMap::new();
        s.chars().for_each(|c| {
            let entry = exists.entry(c).or_insert(0);
            *entry += 1;
        });
        for c in t.chars() {
            if let Some(entry) = exists.get_mut(&c) {
                *entry -= 1;
                if *entry < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        for e in exists.into_values() {
            if e != 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let s = "anagram";
    let t = "nagaram";
    println!("{}", Solution::is_anagram(s.to_owned(), t.to_owned()));

    let s = "rat";
    let t = "car";
    println!("{}", Solution::is_anagram(s.to_owned(), t.to_owned()));

    let s = "ab";
    let t = "a";
    println!("{}", Solution::is_anagram(s.to_owned(), t.to_owned()));
}
