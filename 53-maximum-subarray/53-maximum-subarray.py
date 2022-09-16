class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        res = nums[0]
        cur = nums[0]
        n = len(nums)
        if n==1:
            return res
        for i in range(1,n):
            cur = max(nums[i], cur+nums[i])
            res = max(cur, res)
            
        return res