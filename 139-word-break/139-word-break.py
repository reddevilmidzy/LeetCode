class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        n = len(s)
        dp = [False]*n
        for i in range(n):
            for word in wordDict:
                w = len(word)
                if word == s[i-w+1:i+1] and (dp[i-w] or i-w == -1):
                    dp[i] = True
                    
        return dp[-1]