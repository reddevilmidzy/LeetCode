class Solution:
    def numSquares(self, n: int) -> int:
        dp = [4]*(n+1)
        sq = [i**2 for i in range(1, int(n**0.5)+1)]
        dp[1] = 1
        
        for num in sq:
            dp[num] = 1
        
        for i in range(1, n+1):
            for j in sq:
                if i - j < 0: break
                dp[i] = min(dp[i], dp[i-j]+1)
        
        return dp[n]