struct Solution;

impl Solution {
    #[allow(unused)]
    fn max_sub_array(nums: Vec<i32>) -> (usize, usize, i32) {
        let mut biggest = (0, 0, nums[0]);
        let mut current = biggest;
        for (i, v) in nums.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if v >= &0 {
                if current.2 > 0 {
                    current = (current.0, i, current.2 + v);
                } else {
                    current = (i, i, *v);
                }
            } else if nums[current.0] < 0 {
                current = (current.0 + 1, i, current.2 - nums[current.0] + v);
            } else {
                current = (current.0, i, current.2 + v);
            }
            println!("==={:?}", current);
            if biggest.2 < current.2 {
                biggest = current;
            }
        }
        biggest
    }
}

fn main() {
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{:?}", Solution::max_sub_array(nums.to_vec()));
    let nums = [1];
    println!("{:?}", Solution::max_sub_array(nums.to_vec()));
    let nums = [5, 4, -1, 7, 8];
    println!("{:?}", Solution::max_sub_array(nums.to_vec()));
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{:?}", Solution::max_sub_array(nums.to_vec()));
}
