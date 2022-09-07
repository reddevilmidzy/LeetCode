class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        # n = rowIndex
        dp = [[1] for _ in range(rowIndex+1)]
        for i in range(1, rowIndex+1):
            for j in range(i-1):
                dp[i].extend([dp[i-1][j]+dp[i-1][j+1]])
            dp[i].append(1)
        return dp[rowIndex]