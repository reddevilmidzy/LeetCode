class Solution:
    def combinationSum4(self, nums: List[int], target: int) -> int:
        arr = sorted([i for i in nums if i <= target])
        dp = [0]*(target+1)
        for i in arr:
            dp[i] = 1
        
        for i in range(1, target+1):
            for j in arr:
                if i-j >0 and dp[i-j]!=0:
                    dp[i] += dp[i-j]
        print(dp)
        return dp[target]
        