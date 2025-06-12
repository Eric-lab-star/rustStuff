fn main() {
    let prices = vec![7,1,5,3,6,4];
    let a = Solution::max_profit(prices);
    println!("{a}");

}


struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut profit = 0;

        for &price in prices.iter().skip(1) {
            if price < buy {
                buy = price; 
            } else if price - buy > profit {
                profit = price - buy;
            }
        }
        profit
        
    }
}

