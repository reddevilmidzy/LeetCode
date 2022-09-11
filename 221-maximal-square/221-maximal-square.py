class Solution:
    def maximalSquare(self, matrix: List[List[str]]) -> int:
        rows = len(matrix)
        cols = len(matrix[0])
        
        if rows==1 and cols==1:
            return matrix[0][0]
        
        dp = [[0]*(cols+1) for _ in range(rows+1)]
        res = 0
        for r in range(rows):
            for c in range(cols):
                if matrix[r][c] == '1':
                    dp[r+1][c+1]= min(dp[r][c],dp[r+1][c],dp[r][c+1])+1
                    res = max(res, dp[r+1][c+1])
        return res*res