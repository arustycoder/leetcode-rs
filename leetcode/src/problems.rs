use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    // problem 240: search-a-2d-matrix-ii
    pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut pos = (matrix.len() as i32 - 1, 0);
        let cols = matrix.first().unwrap().len();

        while pos.0 >= 0 && pos.1 < cols {
            let v = matrix[pos.0 as usize][pos.1];
            match v.cmp(&target) {
                Ordering::Less => pos = (pos.0, pos.1 + 1),
                Ordering::Equal => {
                    return true;
                }
                Ordering::Greater => pos = (pos.0 - 1, pos.1),
            }
        }
        false
    }
}

// problem 622 design-circular-queue
struct MyCircularQueue {
    inner: Vec<i32>,
    cap: usize,
    s: usize,
    e: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let mut inner = Vec::with_capacity((k + 1) as usize);
        inner.resize((k + 1) as usize, -1);
        Self {
            inner,
            cap: (k + 1) as usize,
            s: 0,
            e: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        println!("en_queue {} {}", self.s, self.e);
        if self.is_full() {
            false
        } else {
            unsafe {
                *self.inner.get_unchecked_mut(self.e) = value;
            }
            self.e = (self.e + 1) % self.cap;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        println!("{} {}", self.s, self.e);
        if self.is_empty() {
            false
        } else {
            unsafe {
                *self.inner.get_unchecked_mut(self.s) = -1;
            }
            self.s = (self.s + 1) % self.cap;
            true
        }
    }

    fn front(&self) -> i32 {
        unsafe { *self.inner.get_unchecked(self.s) }
    }

    fn rear(&self) -> i32 {
        if !self.is_empty() {
            let e = if self.e > 0 {
                self.e - 1
            } else {
                self.e + self.cap - 1
            };
            unsafe { *self.inner.get_unchecked(e) }
        } else {
            -1
        }
    }

    fn is_empty(&self) -> bool {
        self.e == self.s
    }

    fn is_full(&self) -> bool {
        self.s == (self.e + 1) % self.cap
    }
}

#[allow(unused)]
fn dfs(graph: &Vec<Vec<i32>>, x: i32, n: i32, res: &mut Vec<Vec<i32>>, stack: &mut Vec<i32>) {
    if x == n {
        res.push(stack.clone());
        return;
    }

    for &y in &graph[x as usize] {
        stack.push(y);
        dfs(graph, y, n, res, stack);
        stack.pop();
    }
}

// problem 797 all-paths-from-source-to-target
pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut queue = VecDeque::<Vec<i32>>::new();
    let n = (graph.len() - 1) as i32;

    graph[0].iter().for_each(|&n| queue.push_back(vec![0, n]));
    while let Some(xx) = queue.pop_front() {
        println!("{xx:?}");
        if let Some(x) = xx.last() {
            if x == &n {
                ans.push(xx.clone());
            } else {
                for &y in &graph[*x as usize] {
                    queue.push_back({
                        let mut xx1 = xx.clone();
                        xx1.push(y);
                        xx1
                    })
                }
            }
        }
    }
    ans
}

fn build_tree_inner(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let inorder_map = inorder
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<std::collections::HashMap<_, _>>();

    let root_in_inorder = |x: &i32| *inorder_map.get(x).unwrap();

    if let Some(&val) = preorder.first() {
        let mut root = TreeNode::new(val);
        println!("{preorder:?} {inorder:?}");
        // index is the root
        let index = root_in_inorder(&val);
        let left_inorder = &inorder[0..index];
        let right_inorder = &inorder[index + 1..];

        // the count of left tree is index-1
        let left_preorder = &preorder[1..index + 1];
        let right_preorder = &preorder[index + 1..];

        root.left = build_tree_inner(left_preorder, left_inorder);
        root.right = build_tree_inner(right_preorder, right_inorder);

        return Some(Rc::new(RefCell::new(root)));
    }
    None
}

// problem 105 construct-binary-tree-from-preorder-and-inorder-traversal
pub fn build_tree(preoder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_inner(&preoder, &inorder)
}

#[cfg(test)]
mod tests {

    #[test]
    fn case105() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];

        assert!(super::build_tree(preorder, inorder).is_some());
    }

    #[test]
    fn case797() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let res = vec![vec![0, 1, 3], vec![0, 2, 3]];

        assert_eq!(super::all_paths_source_target(graph), res);
    }
    #[test]
    fn case1() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];

        assert!(super::Solution::search_matrix_ii(matrix, 5));
    }

    #[test]
    fn case622() {
        let mut q = super::MyCircularQueue::new(3);
        assert!(q.en_queue(1)); // 返回 true
        assert!(q.en_queue(2)); // 返回 true
        assert!(q.en_queue(3)); // 返回 true
        assert!(!q.en_queue(4)); // 返回 false，队列已满
        assert_eq!(q.rear(), 3); // 返回 3
        assert!(q.is_full()); // 返回 true
        assert!(q.de_queue()); // 返回 true
        assert!(q.en_queue(4)); // 返回 true
        assert_eq!(q.rear(), 4); // 返回 4
    }

    #[test]
    fn case622_1() {
        //["MyCircularQueue","enQueue","Rear","enQueue","deQueue","Front","deQueue","deQueue","isEmpty","deQueue","enQueue","enQueue"]
        //[[2],[4],[],[9],[],[],[],[],[],[],[6],[4]]
        let mut q = super::MyCircularQueue::new(2);
        q.en_queue(4);
        q.en_queue(9);
        q.de_queue();
        assert_eq!(q.front(), 9);
        assert!(q.de_queue());
        assert!(!q.de_queue())
    }

    #[test]
    fn case622_2() {
        let mut q = super::MyCircularQueue::new(2);
        q.en_queue(1);
        q.en_queue(3);
        q.de_queue(); //3
        q.en_queue(3); //3 3
        q.de_queue(); // 3
        q.en_queue(3); //3 3
        q.de_queue(); // 3
        assert_eq!(q.front(), 3);
    }
}
