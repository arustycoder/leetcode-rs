struct Solution;

impl Solution {
    fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sub_arrays = std::collections::HashSet::<Vec<i32>>::new();
        let max_len = nums.len();
        let mut seq = (0..max_len).map(|s| vec![s]).collect::<Vec<_>>();
        for _ in 2..=max_len {
            // len : the length of current subsequence
            let mut new_seq = Vec::<Vec<usize>>::new();
            for v in seq.iter() {
                let last = *v.last().unwrap();
                for idx in last + 1..max_len {
                    if nums[idx] >= nums[last] {
                        let mut v1 = v.clone();
                        v1.push(idx);
                        let array = v1.iter().map(|s| nums[*s]).collect::<Vec<_>>();
                        if !sub_arrays.contains(&array) {
                            sub_arrays.insert(array);
                        }
                        new_seq.push(v1);
                    }
                }
            }
            seq = new_seq;
        }
        sub_arrays.into_iter().collect()
    }
}

fn main() {
    let nums = [4, 6, 7, 7];
    println!("{:?}", Solution::find_subsequences(nums.to_vec()));

    let nums = [4, 4, 3, 2, 1];
    println!("{:?}", Solution::find_subsequences(nums.to_vec()));
}
