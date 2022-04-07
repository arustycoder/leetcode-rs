struct Solution;

impl Solution {
    #[allow(unused)]
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            map.insert(target - n, i);
        }
        for (i, n) in nums.iter().enumerate() {
            if let Some(&j) = map.get(n) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
fn main() {
    println!("Hello, world!");
}
