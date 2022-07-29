class Solution:
    def tribonacci(self, n: int) -> int:
        dp = [0,1,1]+[0]*(n)

        for i in range(3, n+1):
            dp[i] = dp[i-3]+dp[i-2]+dp[i-1]
        return dp[n]