struct Solution;
impl Solution {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = vec![vec![1i32]];
        for row in 2..=num_rows {
            let array = if let Some(last_row) = triangle.last() {
                (0..row as usize)
                    .map(|s| {
                        if s == 0 || s == row as usize - 1 {
                            1
                        } else {
                            last_row[s - 1] + last_row[s]
                        }
                    })
                    .collect()
            } else {
                vec![1i32]
            };
            triangle.push(array);
        }
        triangle
    }
}
fn main() {
    println!("{:?}", Solution::generate(5));
    println!("{:?}", Solution::generate(1));
}
