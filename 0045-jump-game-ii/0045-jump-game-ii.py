class Solution:
    def jump(self, nums: list[int]) -> int:
        INF = int(1e9)
        n = len(nums)
        dp = [INF]*(n)
        dp[0] = 0
        for i in range(n):
            for j in range(1,nums[i]+1):
                if i+j < n:
                    dp[i+j] = min(dp[i+j], dp[i]+1)
                else:
                    break
        return dp[n-1]