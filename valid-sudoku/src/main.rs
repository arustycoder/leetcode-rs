struct Solution;

impl Solution {
    #[allow(unused)]
    fn is_valid_sudoku1(board: Vec<Vec<char>>) -> bool {
        let judge = |v, nums: &mut Vec<bool>| match v {
            '.' => true,
            '1'..='9' => {
                if nums[v as usize - '1' as usize] {
                    false
                } else {
                    nums[v as usize - '1' as usize] = true;
                    true
                }
            }
            _ => false,
        };

        let len = board.len();
        for r in 0..len {
            let mut nums: Vec<bool> = vec![false; len];
            for c in 0..len {
                if !judge(board[r][c], &mut nums) {
                    return false;
                }
            }
        }

        for c in 0..len {
            let mut nums: Vec<bool> = vec![false; len];
            for r in 0..len {
                if !judge(board[r][c], &mut nums) {
                    return false;
                }
            }
        }

        for (sr, sc) in [
            (0usize, 0usize),
            (0, 3),
            (0, 6),
            (3, 0),
            (3, 3),
            (3, 6),
            (6, 0),
            (6, 3),
            (6, 6),
        ] {
            let mut nums: Vec<bool> = vec![false; len];
            for r in 0..3usize {
                for c in 0..3usize {
                    if !judge(board[sr + r][sc + c], &mut nums) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut r_nums: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut c_nums = r_nums.clone();
        let mut seg_nums = r_nums.clone();
        let segs = [
            (0usize, 0usize),
            (0, 3),
            (0, 6),
            (3, 0),
            (3, 3),
            (3, 6),
            (6, 0),
            (6, 3),
            (6, 6),
        ];
        let mut judge = |v, r: usize, c: usize| match v {
            '.' => true,
            '1'..='9' => {
                let v = v as usize - '1' as usize;
                let mut i = None;
                for (idx, (sr, sc)) in segs.into_iter().enumerate() {
                    if r >= sr && r < sr + 3 && c >= sc && c < sc + 3 {
                        i = Some(idx)
                    }
                }
                let i = i.unwrap();
                if r_nums[r][v] || c_nums[c][v] || seg_nums[i][v] {
                    false
                } else {
                    r_nums[r][v] = true;
                    c_nums[c][v] = true;
                    seg_nums[i][v] = true;
                    true
                }
            }
            _ => false,
        };
        for (r, rv) in board.iter().enumerate() {
            for (c, v) in rv.iter().enumerate() {
                if !judge(*v, r, c) {
                    return false;
                }
            }
        }
        true
    }
}
fn main() {
    let board = [
        ["8", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ];
    let board = board
        .iter()
        .map(|s| {
            s.into_iter()
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    println!("{}", Solution::is_valid_sudoku(board));

    let board = [
        [".", ".", "4", ".", ".", ".", "6", "3", "."],
        [".", ".", ".", ".", ".", ".", ".", ".", "."],
        ["5", ".", ".", ".", ".", ".", ".", "9", "."],
        [".", ".", ".", "5", "6", ".", ".", ".", "."],
        ["4", ".", "3", ".", ".", ".", ".", ".", "1"],
        [".", ".", ".", "7", ".", ".", ".", ".", "."],
        [".", ".", ".", "5", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", ".", "."],
    ];
    let board = board
        .iter()
        .map(|s| {
            s.into_iter()
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    println!("{}", Solution::is_valid_sudoku(board));
}
