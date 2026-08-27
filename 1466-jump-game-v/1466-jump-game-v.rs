impl Solution {
    pub fn max_jumps(nums: Vec<i32>, d: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![-1; n];
        let d = d as usize;

        fn check(nums: &Vec<i32>, dp: &mut Vec<i32>, i: usize, d: usize, n: usize) -> i32 {
            if dp[i] != -1 {
                return dp[i];
            }
            let mut ans = 0;
            for j in 1..=d {
                if i + j < n && nums[i] > nums[i + j] {
                    if dp[i + j] != -1 {
                        ans = ans.max(dp[i + j]);
                    } else {
                        ans = ans.max(check(nums, dp, i + j, d, n));
                    }
                } else {
                    break;
                }
            }

            for j in 1..=d {
                if i >= j && nums[i] > nums[i - j] {
                    if dp[i - j] != -1 {
                        ans = ans.max(dp[i - j]);
                    } else {
                        ans = ans.max(check(nums, dp, i - j, d, n));
                    }
                } else {
                    break;
                }
            }
            dp[i] = ans + 1;
            return dp[i];
        }

        for i in 0..n {
            if dp[i] == -1 {
                check(&nums, &mut dp, i, d, n);
            }
        }
        dp.into_iter().max().unwrap()
    }
}
