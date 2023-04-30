use super::Solution;
use std::collections::HashMap;
use std::ptr::NonNull;

#[allow(dead_code)]

impl Solution {
    // problem 14: longest-common-prefix
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = vec![];
        for i in 0..usize::MAX {
            let mut x = None;
            for s in &strs {
                let ch = match s.as_bytes().get(i) {
                    Some(c) => c,
                    None => unsafe {
                        return String::from_utf8_unchecked(prefix);
                    },
                };
                if x.is_some() && x.unwrap() != ch {
                    unsafe { return String::from_utf8_unchecked(prefix) };
                } else if x.is_none() {
                    x = Some(ch);
                }
            }
            prefix.push(*x.unwrap());
        }

        unsafe { String::from_utf8_unchecked(prefix) }
    }

    // problem 121: best-time-to-buy-and-sell-stock
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut last_min = prices.first().unwrap();
        let mut max_profit = 0;
        for i in prices.iter().skip(1) {
            if i < last_min {
                last_min = i;
            } else {
                let profit = i - last_min;
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }
        max_profit
    }
}

// problem 146 lru-cache
struct Node {
    k: i32,
    v: i32,
    prev: Option<NonNull<Node>>,
    next: Option<NonNull<Node>>,
}

impl Node {
    pub fn new(k: i32, v: i32) -> Self {
        Self {
            k,
            v,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    index: HashMap<i32, NonNull<Node>>,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            head: None,
            tail: None,
            index: HashMap::new(),
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let node = self.index.get_mut(&key);
        let mut value = -1;
        unsafe {
            if let Some(node) = node {
                value = node.as_ref().v;
                let node = *node;
                self.remove(node);
                self.insert_front(node);
            }
        }
        value
    }

    fn put(&mut self, key: i32, value: i32) {
        let node = match self.index.get_mut(&key) {
            Some(node) => {
                unsafe { node.as_mut().v = value };
                let node = *node;
                self.remove(node);
                node
            }
            None => {
                if self.index.len() == self.capacity as usize {
                    if let Some(tail) = self.tail {
                        self.remove(tail);
                        unsafe { self.index.remove(&tail.as_ref().k) };
                    } else {
                        panic!("impossible");
                    }
                }
                Box::leak(Box::new(Node::new(key, value))).into()
            }
        };

        self.index.insert(key, node);
        self.insert_front(node);
    }

    fn insert_front(&mut self, mut node: NonNull<Node>) {
        match self.head {
            None => {
                self.head = Some(node);
                self.tail = Some(node);
            }
            Some(mut head) => {
                unsafe {
                    node.as_mut().next = Some(head);
                    node.as_mut().prev = None;

                    head.as_mut().prev = Some(node);
                }
                self.head = Some(node);
            }
        }
    }

    fn remove(&mut self, mut node: NonNull<Node>) {
        unsafe {
            match node.as_mut().prev {
                Some(mut prev) => prev.as_mut().next = node.as_mut().next,
                None => self.head = node.as_mut().next,
            }

            match node.as_mut().next {
                Some(mut next) => next.as_mut().prev = node.as_mut().prev,
                None => self.tail = node.as_mut().prev,
            };
        }
    }
}

impl Solution {
    // problem 611 valid-triangle-number
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 3 {
            return 0;
        }

        let mut nums = nums;
        nums.sort_by(|a, b| b.partial_cmp(&a).unwrap());
        let mut valid = 0;

        for i in 0..len - 2 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l <= r {
                if nums[l] + nums[r] > nums[i] {
                    valid += r - l;
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        valid as i32
    }

    // problem 56 merge-interval
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        let mut merged: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            match merged.last_mut() {
                Some(i) if i[1] >= interval[0] => {
                    if i[1] <= interval[1] {
                        i[1] = interval[1];
                    }
                }
                _ => merged.push(interval),
            }
        }
        merged
    }

    // problem 91 decode-ways
    pub fn num_decodings(s: String) -> i32 {
        let (mut v2, mut v1, mut v) = (0, 1, 0);
        let zero = 48u8;
        let value_of = |num| num - zero;

        let array = s.as_bytes();
        // when i==array.len(), it means the whole string
        for i in 1..=array.len() {
            v = 0;
            if array[i - 1] != zero {
                v += v1;
            }
            if i > 1
                && array[i - 2] != zero
                && (value_of(array[i - 2]) * 10 + value_of(array[i - 1])) <= 26
            {
                v += v2
            }
            v2 = v1;
            v1 = v;
        }
        v
    }

    // problem 120 triangle
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut p = vec![vec![triangle[0][0]]];

        for (i, line) in triangle.iter().enumerate().skip(1) {
            p.insert(i, vec![]);
            let len = line.len();
            for (j, c) in line.iter().enumerate() {
                let v = if j == 0 {
                    p[i - 1][0]
                } else if j == len - 1 || p[i - 1][j - 1] < p[i - 1][j] {
                    p[i - 1][j - 1]
                } else {
                    p[i - 1][j]
                } + c;
                p[i].push(v);
            }
        }

        *p.last().unwrap().iter().min().unwrap()
    }

    // problem 322 coin-change
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut nums = vec![];
        // `amount+1` means it's impossible to using current coins to get this amount
        nums.resize(amount as usize + 1, amount + 1);
        nums[0] = 0;

        for i in 1..=amount {
            for s in &coins {
                if i >= *s {
                    nums[i as usize] = nums[i as usize].min(nums[(i - s) as usize] + 1);
                }
            }
        }
        nums.last()
            .map(|n| if n > &amount { -1 } else { *n })
            .unwrap()
    }

    fn backtrace(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, idx: usize, len: usize) {
        if idx == len {
            res.push(nums.clone());
            return;
        }
        for i in idx..len {
            nums.swap(i, idx);
            Self::backtrace(res, nums, idx + 1, len);
            nums.swap(i, idx);
        }
    }

    // problem 46 permutations
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = nums.len();

        Self::backtrace(&mut res, &mut nums, 0, len);

        res
    }
}

mod tests {

    #[test]
    fn case46() {
        let nums = vec![1, 2, 3];
        let permute = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        assert_eq!(super::Solution::permute(nums).len(), permute.len());
    }

    #[test]
    fn case322() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(super::Solution::coin_change(coins, amount), 3);

        assert_eq!(super::Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn case120() {
        //   2
        //  3 4
        // 6 5 7
        //4 1 8 3
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(crate::Solution::minimum_total(triangle), 11);

        let triangle = vec![vec![-10]];
        assert_eq!(crate::Solution::minimum_total(triangle), -10);
    }
    #[test]
    fn case91() {
        let s = "12".to_string();
        assert_eq!(crate::Solution::num_decodings(s), 2);
    }

    #[test]
    fn case56() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let new_intervals = vec![vec![1, 6], vec![8, 10], vec![15, 18]];

        assert_eq!(crate::Solution::merge(intervals), new_intervals);
    }

    #[test]
    fn case611() {
        let nums = vec![2, 2, 3, 4];
        assert_eq!(crate::Solution::triangle_number(nums), 3);
        let nums = vec![4, 2, 3, 4];
        assert_eq!(crate::Solution::triangle_number(nums), 4);
    }

    #[test]
    fn case1() {
        let strs = ["flower", "flow", "flight"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        assert_eq!(
            crate::Solution::longest_common_prefix(strs),
            "fl".to_string()
        );
    }

    #[test]
    fn case2() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(crate::Solution::max_profit_v2(prices), 5);
        let prices = vec![7, 6, 5, 4, 3, 2];
        assert_eq!(crate::Solution::max_profit_v2(prices), 0);
    }

    #[test]
    fn case3() {
        let mut lru = super::LRUCache::new(3);
        assert_eq!(lru.get(1), -1);
        lru.put(1, 1001);
        lru.put(2, 1002);
        lru.put(3, 1003);
        assert_eq!(lru.get(1), 1001);
        assert_eq!(lru.get(2), 1002);
        assert_eq!(lru.get(3), 1003);

        lru.put(4, 1004);
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(2), 1002);
        assert_eq!(lru.get(3), 1003);
        assert_eq!(lru.get(4), 1004);
    }
}
