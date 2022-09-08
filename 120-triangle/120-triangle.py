class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        n = len(triangle)
        dp = []
        for i in range(1,n+1):
            dp.append([0]*i)
        dp[0] = triangle[0]
        for i in range(1, n):
            for j in range(len(dp[i])):
                if j==0:
                    dp[i][j] = dp[i-1][j]+triangle[i][j]
                elif j==len(dp[i])-1:
                    dp[i][j] = dp[i-1][j-1]+triangle[i][j]
                else:
                    dp[i][j] = min(dp[i-1][j-1],dp[i-1][j])+triangle[i][j]
                    
        return min(dp[n-1])