class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        m, n = len(text1), len(text2)
        if m < n: 
            n,m = m,n
            text1,text2 = text2,text1
        dp = [0] * (n + 1)
        for i in range(1, m + 1):
            pre = 0
            for j in range(1, n + 1):
                cur = dp[j]
                dp[j] = pre + 1 if text1[i-1] == text2[j-1] else max(dp[j-1], cur)
                pre = cur
        return dp[-1]