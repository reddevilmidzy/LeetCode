class Solution:
    def rob(self, nums: List[int]) -> int:
        dp = [0]*(len(nums))
        if len(nums)==1:
            return nums[0]
        elif len(nums)==2:
            return max(nums[0], nums[1])
        
        for i in range(3):
            dp[i] = nums[i]

        for i in range(2, len(nums)):
            dp[i] = max(dp[i-2]+nums[i], dp[i-1], dp[i-3]+nums[i])
        return max(dp[-1], dp[-2])