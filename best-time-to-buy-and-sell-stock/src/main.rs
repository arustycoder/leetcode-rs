struct Solution;

impl Solution {
    #[allow(unused)]
    fn max_profit_time_consuming(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let mut highest = 0;
        let mut last_max = None;
        for x in 0..prices.len() - 1 {
            let v = prices[x];
            if last_max.is_some() && last_max.unwrap() == v {
                last_max = None;
                continue;
            }
            let &max = prices[x..].iter().max().unwrap();
            last_max = Some(max);
            if max > v {
                highest = std::cmp::max(max - v, highest);
            }
        }
        highest
    }
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut highest = 0;
        let mut last_min = prices[0];
        for &p in prices.iter().skip(1) {
            if p < last_min {
                last_min = p;
            } else {
                let profit = p - last_min;
                if profit > highest {
                    highest = profit;
                }
            }
        }
        highest
    }
}
fn main() {
    let prices = [1, 23, 4, 56, 23];
    println!("{}", Solution::max_profit(prices.to_vec()));
    let prices = [7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit(prices.to_vec()));
    let prices = [7, 6, 4, 3, 1];
    println!("{}", Solution::max_profit(prices.to_vec()));
}
