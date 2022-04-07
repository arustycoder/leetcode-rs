use std::alloc::dealloc;
use std::borrow::Borrow;
use std::ops::{BitXor, Deref, DerefMut};
use std::ptr::read_volatile;
use std::task::Poll::Pending;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() || list2.is_none() {
            if list1.is_some() {
                return list1;
            }
            return list2;
        }

        let mut node1 = list1.unwrap();
        let mut node2 = list2.unwrap();
        if node1.val > node2.val {
            node2.next = Self::merge_two_lists(Some(node1), node2.next);
            Some(node2)
        } else {
            node1.next = Self::merge_two_lists(node1.next, Some(node2));
            Some(node1)
        }
    }

    fn from_array(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;

        for v in nums.iter().rev() {
            let node = Box::new(ListNode {
                next: head,
                val: *v,
            });
            head = Some(node);
        }

        head
    }

    fn dump(head: &Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut next = head.as_ref().unwrap().as_ref();
        loop {
            print!("{} ", next.val);
            if next.next.is_none() {
                break;
            }
            next = next.next.as_ref().unwrap().as_ref();
        }
        println!();
    }

    fn delete(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            while node.val == val {
                if let Some(next) = node.next {
                    node = next;
                } else {
                    return None;
                }
            }
            node.next = Self::delete(node.next, val);
            Some(node)
        } else {
            None
        }
    }

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

fn main() {
    let nums = [1, 2, 3, 4];
    let mut head = Solution::from_array(nums.as_ref());
    Solution::dump(&head);
    println!("removing 3");
    head = Solution::delete(head, 3);
    Solution::dump(&head);
    println!("removing 5");
    head = Solution::delete(head, 5);
    Solution::dump(&head);
    println!("removing 1");
    head = Solution::delete(head, 1);
    Solution::dump(&head);

    let nums = [];
    let list2 = Solution::from_array(nums.as_ref());
    let list = Solution::merge_two_lists(head, list2);
    Solution::dump(&list);
}
