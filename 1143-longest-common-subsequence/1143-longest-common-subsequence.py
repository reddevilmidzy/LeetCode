class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        te1_len = len(text1)
        m, n = len(text1), len(text2)
        if m < n: return self.longestCommonSubsequence(text2, text1) # 길이가 2가 더 크다면
        dp = [0] * (n + 1)
        for i in range(1, m + 1):
            pre = 0
            for j in range(1, n + 1):
                cur = dp[j]
                dp[j] = pre + 1 if text1[i-1] == text2[j-1] else max(dp[j-1], cur)
                pre = cur
        return dp[-1]