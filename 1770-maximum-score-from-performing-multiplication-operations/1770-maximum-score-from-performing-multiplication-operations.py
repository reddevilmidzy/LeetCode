class Solution:
    def maximumScore(self, nums: List[int], multipliers: List[int]) -> int:
        m=len(multipliers)
        n=len(nums)
        
        dp=[[0]*(m+1) for i in range(m+1)]
        for j in range(m-1,-1,-1):
            for left in range(j,-1,-1):
                start=multipliers[j]*nums[left]+dp[j+1][left+1]
                end=multipliers[j]*nums[n-1-(j-left)]+dp[j+1][left]
                dp[j][left] = max(start,end)
        
        return dp[0][0]