class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        n = numRows
        dp = [[1] for _ in range(n)]
        for i in range(1, n):
            for j in range(i-1):
                dp[i].extend([dp[i-1][j]+dp[i-1][j+1]])
            dp[i].append(1)
        return dp