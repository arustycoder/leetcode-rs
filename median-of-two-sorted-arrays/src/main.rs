fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let total = m + n;
        let (odd, mut k) = if total & 1 != 0 {
            // 奇数个
            (true, total / 2 + 1)
        } else {
            // 偶数
            (false, total / 2)
        };
        let mut slice1 = nums1.as_slice();
        let mut slice2 = nums2.as_slice();
        // 偶数 (k-1, k), 奇数 k-1
        loop {
            let half = k / 2;
            if slice1.is_empty() {
                return if odd {
                    slice2[k - 1] as f64
                } else {
                    (slice2[k - 1] + slice2[k]) as f64 / 2f64
                };
            } else if slice2.is_empty() {
                return if odd {
                    slice1[k - 1] as f64
                } else {
                    (slice1[k - 1] + slice1[k]) as f64 / 2f64
                };
            } else if k == 1 {
                return if odd {
                    if slice1[0] < slice2[0] {
                        slice1[0] as f64
                    } else {
                        slice2[0] as f64
                    }
                } else if slice1.len() == 1 && slice2.len() == 1 {
                    (slice1[0] + slice2[0]) as f64 / 2f64
                } else if slice1.len() == 1 && slice2.len() > 1 {
                    (slice2[0] + slice2[1].min(slice1[0])) as f64 / 2f64
                } else if slice1.len() > 1 && slice2.len() == 1 {
                    (slice1[0] + slice1[1].min(slice2[0])) as f64 / 2f64
                } else {
                    //len1>1 && len2>1
                    if slice1[1] <= slice2[0] {
                        (slice1[0] + slice1[1]) as f64 / 2f64
                    } else if slice2[1] <= slice1[0] {
                        (slice2[0] + slice2[1]) as f64 / 2f64
                    } else {
                        (slice1[0] + slice2[0]) as f64 / 2f64
                    }
                };
            } else {
                let len1 = slice1.len();
                let len2 = slice2.len();
                let num1 = if len1 >= half {
                    slice1[half - 1]
                } else {
                    slice1[len1 - 1]
                };
                let num2 = if len2 >= half {
                    slice2[half - 1]
                } else {
                    slice2[len2 - 1]
                };

                if num1 < num2 {
                    slice1 = if len1 >= half {
                        k -= half;
                        &slice1[half..]
                    } else {
                        k -= len1;
                        &[]
                    }
                } else {
                    slice2 = if len2 >= half {
                        k -= half;
                        &slice2[half..]
                    } else {
                        k -= len2;
                        &[]
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn find_median_sorted_arrays1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // t (array, idx in array, nums are seen before)
        let mut t = if nums1.is_empty() {
            (2usize, 0usize, 0usize)
        } else if nums2.is_empty() {
            (1usize, 0usize, 0usize)
        } else if nums1[0] > nums2[0] {
            (2usize, 0usize, 0usize)
        } else {
            (1usize, 0usize, 0usize)
        };
        let m = nums1.len();
        let n = nums2.len();
        let total = m + n;

        let num_of = |(arr_idx, num_idx, _)| {
            if arr_idx == 1 {
                nums1[num_idx]
            } else {
                nums2[num_idx]
            }
        };

        let next_of = |(arr_idx, num_idx, nums_seen)| {
            let other_nxt_idx = nums_seen - num_idx;
            let nxt_idx = num_idx + 1;
            let (arr_idx, nums, other_nums) = if arr_idx == 1 {
                (1usize, nums1.as_slice(), nums2.as_slice())
            } else {
                (2usize, nums2.as_slice(), nums1.as_slice())
            };

            if nxt_idx == nums.len() {
                (3usize - arr_idx, other_nxt_idx, nums_seen + 1)
            } else if other_nxt_idx == other_nums.len()
                || nums[nxt_idx] <= other_nums[other_nxt_idx]
            {
                (arr_idx, nxt_idx, nums_seen + 1)
            } else {
                (3usize - arr_idx, other_nxt_idx, nums_seen + 1)
            }
        };

        loop {
            if t.2 * 2 + 1 == total {
                return num_of(t) as f64;
            } else if t.2 * 2 + 2 == total {
                return (num_of(t) as f64 + num_of(next_of(t)) as f64) / 2f64;
            }
            t = next_of(t)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn odd_case() {
        use super::Solution;

        //let nums1 = [1, 3].to_vec();
        //let nums2 = [2].to_vec();
        //assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2f64);

        //let nums1 = [1, 2].to_vec();
        //let nums2 = [3].to_vec();
        //assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2f64);

        //let nums1 = [1, 3, 5].to_vec();
        //let nums2 = [2, 4].to_vec();
        //assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 3f64);

        //let nums1 = [1, 2, 4].to_vec();
        //let nums2 = [3, 5].to_vec();
        //assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 3f64);

        //let nums1 = [1].to_vec();
        //let nums2 = [2, 3, 4, 5, 6, 7].to_vec();
        //assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 4f64);

        let nums1 = [1, 2, 3, 5, 6, 7].to_vec();
        let nums2 = [4].to_vec();
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 4f64);
    }
    #[test]
    fn even_case() {
        use super::Solution;

        let nums1 = [1, 2].to_vec();
        let nums2 = [3, 4].to_vec();
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5f64);

        let nums1 = [1, 3].to_vec();
        let nums2 = [2, 4].to_vec();
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5f64);

        let nums1 = [1, 3].to_vec();
        let nums2 = [1, 4].to_vec();
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2f64);

        let nums1 = [2, 3, 6].to_vec();
        let nums2 = [1, 4, 10].to_vec();
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 3.5f64);
    }
}
