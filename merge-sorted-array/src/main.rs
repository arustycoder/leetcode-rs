struct Solution;

impl Solution {
    #[allow(unused)]
    fn merge_sorted_array1(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        assert_eq!(nums1.len() as i32, m + n);
        assert_eq!(nums2.len() as i32, n);

        unsafe {
            let ptr1 = nums1.as_mut_ptr().add(m as usize);
            let ptr2 = nums2.as_ptr();
            std::ptr::copy(ptr2, ptr1, n as usize);
        }
        nums1.sort_unstable();
    }
    #[allow(unused)]
    fn merge_sorted_array2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        assert_eq!(nums1.len() as i32, m + n);
        assert_eq!(nums2.len() as i32, n);
        let tmp = nums1[..m as usize].to_vec();
        println!("tmp {:?}", tmp);
        let mut iter1 = tmp.iter();
        let mut iter2 = nums2.iter();
        let mut v1 = iter1.next();
        let mut v2 = iter2.next();
        for v in nums1.iter_mut() {
            if v1.is_some() && v2.is_some() {
                if v1.unwrap() > v2.unwrap() {
                    *v = *v2.unwrap();
                    v2 = iter2.next();
                } else {
                    *v = *v1.unwrap();
                    v1 = iter1.next();
                }
            } else if v1.is_some() {
                *v = *v1.unwrap();
                v1 = iter1.next();
            } else if v2.is_some() {
                *v = *v2.unwrap();
                v2 = iter2.next();
            } else {
                break;
            };
            println!("{}", v);
        }
    }

    fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        assert!(m >= 0 && n >= 0);
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        let mut tail = m + n - 1;
        while p1 >= 0 || p2 >= 0 {
            if p1 == -1 {
                let v2 = nums2[p2 as usize];
                nums1[tail as usize] = v2;
                p2 -= 1;
            } else if p2 == -1 {
                let v1 = nums1[p1 as usize];
                nums1[tail as usize] = v1;
                p1 -= 1;
            } else {
                let v1 = nums1[p1 as usize];
                let v2 = nums2[p2 as usize];
                if v1 > v2 {
                    nums1[tail as usize] = v1;
                    p1 -= 1;
                } else {
                    nums1[tail as usize] = v2;
                    p2 -= 1;
                }
            }
            tail -= 1;
        }
    }
}
fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = nums2.len() as i32;
    Solution::merge_sorted_array(&mut nums1, m, &mut nums2, n);
    println!("{:?} {:?}", nums1, nums2);

    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = nums2.len() as i32;
    Solution::merge_sorted_array(&mut nums1, m, &mut nums2, n);
    println!("{:?} {:?}", nums1, nums2);
}
