use super::Solution;

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

mod tests {

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
}
