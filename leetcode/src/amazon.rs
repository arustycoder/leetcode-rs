use crate::{ListNode, Solution};

#[allow(dead_code)]

impl Solution {
    // =5=
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 1. 扫描一遍，将num和下标记录到map中
        // 2. 逐个检查target-num是否在map中，如果存在且下标与自己不同，返回
        let mut map = std::collections::BTreeMap::new();
        nums.iter().enumerate().for_each(|(i, x)| {
            map.insert(*x, i);
        });
        for (i, x) in nums.into_iter().enumerate() {
            let diff = target - x;
            match map.get(&diff) {
                Some(&idx) if idx != i => {
                    return vec![i as i32, idx as i32];
                }
                _ => {}
            }
        }
        unreachable!()
    }

    // =2=
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 1. 两条链同时遍历，相加结果，添加到链末尾
        // 2. 如果有一条链到达末尾，将另外一条链加到末尾
        // 3. 进位
        let mut head = &None as *const Option<Box<ListNode>>;
        let mut tail = &mut None as *mut Option<Box<ListNode>>;
        let mut carry = 0;
        while l1.is_some() && l2.is_some() {
            let mut adder1 = l1.unwrap();
            let mut adder2 = l2.unwrap();
            adder1.val += adder2.val + carry;
            if adder1.val >= 10 {
                adder1.val -= 10;
                carry = 1;
            } else {
                carry = 0;
            }

            l1 = adder1.next.take();
            l2 = adder2.next.take();

            if unsafe { &*tail }.is_none() {
                unsafe { *tail = Some(adder1) };
                head = tail as *const Option<Box<ListNode>>;
            } else {
                unsafe {
                    (*tail).as_mut().unwrap().next = Some(adder1);
                    tail = &mut (*tail).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                }
            }
        }
        let mut l = if l1.is_none() { l2 } else { l1 };
        if unsafe { &*tail }.is_some() {
            while l.is_some() {
                l.as_mut().unwrap().val += carry;
                carry = if l.as_mut().unwrap().val >= 10 {
                    l.as_mut().unwrap().val -= 10;
                    1
                } else {
                    0
                };
                let next = l.as_mut().unwrap().next.take();
                unsafe {
                    (*tail).as_mut().unwrap().next = l;
                    tail = &mut (*tail).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                }
                l = next;
            }
            if carry == 1 {
                let node = Box::new(ListNode::new(1));
                unsafe {
                    (*tail).as_mut().unwrap().next = Some(node);
                }
            }
            unsafe { (*head).clone() }
        } else {
            l
        }
    }
    // =5=
    pub fn longest_palindrome_5(s: String) -> String {
        // 1. 从头扫描每个字符
        // 2. 计算该字符开始的最长回文字符串
        // 3. 如果长度大于，则记录下标[s,e)
        let pf = |s: &[u8]| {
            let len = s.len();
            let even = len % 2 == 0;
            let mut i = 0;
            let half = if even { len / 2 } else { len / 2 + 1 };
            while i < half && s[i] == s[len - 1 - i] {
                i += 1;
            }
            i == half
        };
        let mut max = (0, 0, 0);
        for x in 0..s.len() {
            for y in x + 1..s.len() {
                let ss = &s[x..=y];
                if pf(ss.as_bytes()) {
                    let len = y - x + 1;
                    if len > max.0 {
                        max = (len, x, y);
                    }
                }
            }
        }
        s[max.1..=max.2].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works_5() {
        let result = Solution::longest_palindrome_5("aba".to_string());
        println!("{}", result);
        let result = Solution::longest_palindrome_5("aa".to_string());
        println!("{}", result);
        let result = Solution::longest_palindrome_5("ab".to_string());
        println!("{}", result);
        let result = Solution::longest_palindrome_5("babad".to_string());
        println!("{}", result);
    }
}
