impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n : usize = prices.len();
        let mut res : Vec<i32> = vec![-1; n];

        for i in 0..n {
            for j in i+1..n {
                if prices[j] <= prices[i] {
                    res[i] = prices[i] - prices[j];
                    break;
                } 
            }
            if res[i] == -1 {
                res[i] = prices[i];
            }
        }
        res
    }
}
