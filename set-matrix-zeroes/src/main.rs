struct Solution;
impl Solution {
    #[allow(unused)]
    fn set_zeros1(matrix: &mut Vec<Vec<i32>>) {
        let rc = matrix.len();
        let cc = matrix[0].len();
        let flag = i32::MIN;

        for r in 0..rc {
            for c in 0..cc {
                if matrix[r][c] == 0 {
                    matrix[r].iter_mut().for_each(|s| {
                        if *s != 0 {
                            *s = flag;
                        }
                    });
                    (0..rc).for_each(|r1| {
                        if matrix[r1][c] != 0 {
                            matrix[r1][c] = flag;
                        }
                    });
                    matrix[r][c] = 0;
                }
            }
        }
        for r in 0..rc {
            for c in 0..cc {
                if matrix[r][c] == flag {
                    matrix[r][c] = 0;
                }
            }
        }
    }

    fn set_zeros(matrix: &mut Vec<Vec<i32>>) {
        let rc = matrix.len();
        let cc = matrix[0].len();
        let mut flag_col0 = false;
        let mut flag_row0 = false;

        for v in matrix.iter() {
            if v[0] == 0 {
                flag_col0 = true;
                break;
            }
        }

        for c in 0..cc {
            if matrix[0][c] == 0 {
                flag_row0 = true;
                break;
            }
        }

        for r in 1..rc {
            for c in 1..cc {
                if matrix[r][c] == 0 {
                    matrix[r][0] = 0;
                    matrix[0][c] = 0;
                }
            }
        }

        for r in 1..rc {
            for c in 1..cc {
                if matrix[r][0] == 0 || matrix[0][c] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        if flag_col0 {
            matrix.iter_mut().for_each(|v| v[0] = 0);
        }

        if flag_row0 {
            matrix[0].iter_mut().for_each(|v| *v = 0);
        }
    }
}

fn main() {
    let matrix = [[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let mut matrix = matrix.iter().map(|s| s.to_vec()).collect();
    Solution::set_zeros(&mut matrix);
    println!("{:?}", matrix);

    let matrix = [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
    let mut matrix = matrix.iter().map(|s| s.to_vec()).collect();
    Solution::set_zeros(&mut matrix);
    println!("{:?}", matrix);
}
