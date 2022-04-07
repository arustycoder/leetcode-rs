struct Solution;

impl Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hm = std::collections::HashSet::new();
        for num in nums {
            if !hm.insert(num) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let nums = [1, 2, 3, 1];
    assert!(Solution::contains_duplicate(nums.to_vec()));

    let nums = [1, 2, 3, 4];
    assert!(!Solution::contains_duplicate(nums.to_vec()));
}
