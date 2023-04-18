use crate::Solution;
use std::cmp::Ordering;

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

mod tests {
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
}
