struct Solution;
impl Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut exists = vec![(0usize, 0usize); 26];
        magazine
            .chars()
            .for_each(|c| exists[c as usize - 'a' as usize].0 += 1);

        for c in ransom_note.chars() {
            let idx = c as usize - 'a' as usize;
            exists[idx].1 += 1;
            if exists[idx].1 > exists[idx].0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let ransom_note = "a";
    let magazine = "b";
    println!(
        "{}",
        Solution::can_construct(ransom_note.to_owned(), magazine.to_owned())
    );

    let ransom_note = "aa";
    let magazine = "ba";
    println!(
        "{}",
        Solution::can_construct(ransom_note.to_owned(), magazine.to_owned())
    );

    let ransom_note = "aa";
    let magazine = "aba";
    println!(
        "{}",
        Solution::can_construct(ransom_note.to_owned(), magazine.to_owned())
    );
}
