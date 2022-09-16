class Solution:
    def numSquares(self, n: int) -> int:
        perfect = [i**2 for i in range(1,int(n**0.5)+1)]
        INF = 10
        dp = [INF]*(n+1)
        for i in perfect:
            dp[i] = 1
        
        for i in range(1, n+1):
            for j in perfect:
                if i-j>0 and dp[i-j]!=0:
                    if dp[i-j]+1 < dp[i]:
                        dp[i] = dp[i-j]+1
        return dp[n]