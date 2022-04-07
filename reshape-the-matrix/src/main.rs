struct Solution;

impl Solution {
    fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat.len() * mat[0].len() != (r * c) as usize {
            mat
        } else {
            let mut matrix = vec![vec![0; c as usize]; r as usize];
            let c_len = mat[0].len();
            for (or, rv) in mat.iter().enumerate() {
                for (oc, cv) in rv.iter().enumerate() {
                    let new_r = (or * c_len + oc) / c as usize;
                    let new_c = or * c_len + oc - new_r * c as usize;
                    matrix[new_r][new_c] = *cv;
                }
            }
            matrix
        }
    }
}

fn main() {
    let mat = [[1, 2], [3, 4]];
    let r = 1;
    let c = 4;
    println!(
        "{:?}",
        Solution::matrix_reshape(mat.map(|s| s.to_vec()).to_vec(), r, c)
    );

    let mat = [[1, 2], [3, 4]];
    let r = 2;
    let c = 4;
    println!(
        "{:?}",
        Solution::matrix_reshape(mat.map(|s| s.to_vec()).to_vec(), r, c)
    );
}
