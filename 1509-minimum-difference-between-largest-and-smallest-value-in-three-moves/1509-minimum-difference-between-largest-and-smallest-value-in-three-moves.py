class Solution:
    def minDifference(self, nums: List[int]) -> int:
        n = len(nums)
        if n <= 4:
            return 0
        nums.sort()

        res = float('inf')
        for left in range(4):
            right = n - 4 + left
            res = min(res, nums[right] - nums[left])
        
        return res
