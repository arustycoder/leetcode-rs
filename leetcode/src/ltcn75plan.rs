use crate::Solution;

impl Solution {
    // leetcode 1480
    #[allow(dead_code)]
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut last: Option<i32> = None;
        nums.into_iter()
            .map(|n| {
                if let Some(sum) = last {
                    let current = sum.saturating_add(n);
                    last = Some(current);
                    current
                } else {
                    last = Some(n);
                    n
                }
            })
            .collect()
    }

    // 724
    #[allow(dead_code)]
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum();
        let mut sum = 0;
        for (i, x) in nums.into_iter().enumerate() {
            if (2 * sum + x) == total {
                return i as i32;
            }
            sum += x;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works_1480() {
        let nums = vec![1, 2, 3, 4];
        let sums = Solution::running_sum(nums);
        assert_eq!(sums, vec![1, 3, 6, 10]);

        let nums = vec![1, 1, 1, 1];
        let sums = Solution::running_sum(nums);
        assert_eq!(sums, vec![1, 2, 3, 4]);
    }

    #[test]
    fn it_works_724() {
        let nums = vec![2, -1, -1];
        assert_eq!(Solution::pivot_index(nums), -1);

        let nums = vec![1, 3, 7, 6, 5, 6];
        assert_eq!(Solution::pivot_index(nums), 3);

        let nums = vec![2, 1, -1];
        assert_eq!(Solution::pivot_index(nums), 0);
    }
}
