impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let n : usize = nums.len();
        let m : usize = 10_000;
        let mut cnt = vec![0; m + 1]; 

        for num in nums {
            cnt[num as usize] += 1;
        }
        
        let mut dp = vec![0; m + 1];
        dp[1] = cnt[1];
        dp[2] = dp[1].max(cnt[2] * 2);

        for i in 3..=m {
            dp[i] = (dp[i - 2] + cnt[i] * i).max(dp[i - 1]);
        }

        dp[m].max(dp[m - 1]) as i32 
    }
}