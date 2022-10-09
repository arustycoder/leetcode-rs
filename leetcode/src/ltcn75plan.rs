use crate::{ListNode, Solution, TreeNode};
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

#[allow(dead_code)]

impl Solution {
    // 1480
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

    // 205
    #[allow(dead_code)]
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s2t = HashMap::new();
        let mut t2s = HashMap::new();
        if s.len() != t.len() {
            return false;
        }
        for (x, y) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
            let e = s2t.entry(x).or_insert(y);
            if e != &y {
                return false;
            }
            let e1 = t2s.entry(y).or_insert(x);
            if e1 != &x {
                return false;
            }
        }
        true
    }
    // 392
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }
        if s.is_empty() {
            return true;
        }
        let mut it = 0;
        let mut end = 0;
        for (i, x) in s.as_bytes().iter().enumerate() {
            while it < t.len() && t.as_bytes()[it] != *x {
                it += 1;
            }
            if it == t.len() {
                return false;
            }
            it += 1;
            end = i;
        }
        end == s.len() - 1
    }

    // =21=
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (list1, list2);
        let mut head = None;
        let mut tail = &mut head;

        while l1.is_some() || l2.is_some() {
            if l1.is_some() && l2.is_some() {
                let v1 = l1.as_ref().unwrap().val;
                let v2 = l2.as_ref().unwrap().val;
                if v1 < v2 {
                    let next = l1.as_mut().unwrap().next.take();
                    *tail = l1;
                    l1 = next;
                } else {
                    let next = l2.as_mut().unwrap().next.take();
                    *tail = l2;
                    l2 = next;
                }
            } else if l1.is_some() {
                let next = l1.as_mut().unwrap().next.take();
                *tail = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                *tail = l2;
                l2 = next;
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }

    // =206=
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 1->2->3->4
        // prev, cur 1->2->3->4
        // prev 1 cur 2->3->4
        // prev 2->1 cur 3->4
        let mut prev = None;
        let mut cur = head;
        while cur.is_some() {
            let temp = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = prev;
            prev = cur;
            cur = temp;
        }
        prev
    }

    // =876=
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        slow.clone()
    }

    // =121=
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max = 0;

        for x in prices {
            if x - min_price > max {
                max = x - min_price;
            }
            if x < min_price {
                min_price = x;
            }
        }
        max
    }

    // =409=
    pub fn longest_palindrome(s: String) -> i32 {
        // 1. 扫描每个字符，如果没出现就存入Set西
        // 2. 存在，就移除，长度+2
        // 3. set中还有，长度+1
        // 4. 返回长度
        let mut set = HashSet::new();
        let mut len = 0;
        for x in s.as_bytes() {
            let p = set.insert(x);
            if !p {
                set.remove(x);
                len += 2;
            }
        }
        if !set.is_empty() {
            len += 1;
        }
        len
    }

    // =102=
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // 1. level，current
        // 2. next
        let mut res = vec![];
        let mut stack = VecDeque::new();
        if let Some(root) = root {
            stack.push_back(root);
        } else {
            return res;
        }
        let mut level = 0;
        let mut current = 1;
        let mut next = 0;
        while let Some(node) = stack.pop_front() {
            let mut node = node.borrow_mut();
            if let Some(array) = res.get_mut(level) {
                array.push(node.val);
            } else {
                let create = vec![node.val];
                res.push(create);
            }
            if let Some(left) = node.left.take() {
                stack.push_back(left);
                next += 1;
            }
            if let Some(right) = node.right.take() {
                stack.push_back(right);
                next += 1;
            }
            current -= 1;
            if current == 0 {
                //next level
                level += 1;
                current = next;
                next = 0;
            }
        }
        res
    }

    // =704=
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut hi = nums.len() as i32;
        let mut lo = 0;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let num = nums[mid as usize];
            match num.cmp(&target) {
                Ordering::Equal => {
                    return mid;
                }
                Ordering::Greater => hi = mid,
                Ordering::Less => lo = mid + 1,
            }
        }
        -1
    }

    // =278=
    pub fn first_bad_version(&self, n: i32) -> i32 {
        // 1. g g g b g g g b b b b
        let is_bad_version = |version: i32| version < 3;

        let mut lo = 1;
        let mut hi = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if is_bad_version(mid) {
                hi = mid; // [lo, mid]
            } else {
                lo = mid + 1; //[mid+1, hi]
            }
        }
        lo
    }

    // =98=
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut last = None;
        let mut stack = VecDeque::new();
        let mut root = root;

        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow_mut().left.take();
                stack.push_back(node);
            }

            let node = stack.pop_back().unwrap();
            if let Some(last) = last {
                if node.borrow().val <= last {
                    return false;
                }
            }
            last = Some(node.borrow().val);
            root = node.borrow_mut().right.take();
        }
        true
    }

    // =235=
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        while let Some(node) = root {
            let val = node.borrow().val;
            if val > q_val && val > p_val {
                root = node.borrow_mut().left.take();
            } else if val < q_val && val < p_val {
                root = node.borrow_mut().right.take();
            } else {
                return Some(node);
            }
        }
        unreachable!()
    }

    // =733=
    fn helper(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, orig: i32, new: i32) {
        if sr < 0 || sc < 0 {
            return;
        }
        if let Some(r) = image.get_mut(sr as usize) {
            if let Some(n) = r.get_mut(sc as usize) {
                if n == &orig && n != &new {
                    *n = new;
                    Self::helper(image, sr - 1, sc, orig, new);
                    Self::helper(image, sr + 1, sc, orig, new);
                    Self::helper(image, sr, sc - 1, orig, new);
                    Self::helper(image, sr, sc + 1, orig, new);
                }
            }
        }
    }
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let orig = image[sr as usize][sc as usize];
        Self::helper(&mut image, sr, sc, orig, color);
        image
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works_392() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        assert!(Solution::is_subsequence(s, t));

        let s = String::from("axc");
        let t = String::from("ahbgdc");
        assert!(!Solution::is_subsequence(s, t));
    }
    #[test]
    fn it_works_205() {
        let s = String::from("egg");
        let t = String::from("add");
        assert!(Solution::is_isomorphic(s, t));

        let s = String::from("foo");
        let t = String::from("bar");
        assert!(!Solution::is_isomorphic(s, t));

        let s = String::from("paper");
        let t = String::from("title");
        assert!(Solution::is_isomorphic(s, t));
    }
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
