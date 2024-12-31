use std::collections::HashSet;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let m : usize = 365;
        let mut dp : Vec<i32> = vec![1_000_000_000; m + 1];
        dp[0] = 0;
        
        let nums : HashSet<usize> = days.into_iter().map(|x| x as usize).collect();
        let cheap = costs.iter().min().unwrap();
        for i in 1..=m {
            if !nums.contains(&i) {
                dp[i] = dp[i-1];
            }

            dp[i] = dp[i].min(dp[i-1] + cheap);
            if 7 <= i {
                dp[i] = dp[i].min(dp[i-7] + costs[1]);
            }
            if 30 <= i {
                dp[i] = dp[i].min(dp[i-30] + costs[2]);
            }

        }
        dp[m]
    }
}