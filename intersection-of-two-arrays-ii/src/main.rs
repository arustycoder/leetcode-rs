use std::cmp::Ordering;

struct Solution;

impl Solution {
    #[allow(unused)]
    fn intersect1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let mut nums2 = {
            let mut map = std::collections::HashMap::new();
            for v in nums2.iter() {
                let entry = map.entry(v).or_insert((0, 0));
                entry.0 += 1;
            }
            map
        };
        for k in nums1.iter() {
            if let Some(v) = nums2.get_mut(k) {
                v.1 += 1;
            }
        }
        for (&&k, (c1, c2)) in nums2.iter() {
            res.append(&mut vec![k; std::cmp::min(*c1, *c2)]);
        }
        res
    }
    fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut iter1 = nums1.iter();
        let mut iter2 = nums2.iter();
        let mut v1 = iter1.next();
        let mut v2 = iter2.next();
        while v1.is_some() && v2.is_some() {
            match v1.unwrap().cmp(v2.unwrap()) {
                Ordering::Less => v1 = iter1.next(),
                Ordering::Greater => v2 = iter2.next(),
                Ordering::Equal => {
                    res.push(*v1.unwrap());
                    v1 = iter1.next();
                    v2 = iter2.next();
                }
            }
        }

        res
    }
}

fn main() {
    let nums1 = [1, 2, 2, 1];
    let nums2 = [2, 2];
    println!(
        "{:?} {:?}\n==={:?}",
        nums1,
        nums2,
        Solution::intersect(nums1.to_vec(), nums2.to_vec())
    );
    let nums1 = [4, 9, 5];
    let nums2 = [9, 4, 9, 8, 4];
    println!(
        "{:?} {:?}\n==={:?}",
        nums1,
        nums2,
        Solution::intersect(nums1.to_vec(), nums2.to_vec())
    );
}
