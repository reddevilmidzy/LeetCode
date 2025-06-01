impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let n: usize = nums.len();
        let m: usize = 2_0;
        let mut dp = vec![1; n];
        let mut cnt = vec![1; n];
        let mut max_len = 1;

        for i in 0..n {
            for j in (0..=i).rev() {
                if nums[j] < nums[i] {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        cnt[i] = 0;
                    }
                    if dp[j] + 1 == dp[i] {
                        cnt[i] += cnt[j];
                    }
                }
            }
            max_len = max_len.max(dp[i]);
        }

        for i in 0..n {
            if dp[i] == max_len {
                res += cnt[i];
            }
        }

        res
    }
}
