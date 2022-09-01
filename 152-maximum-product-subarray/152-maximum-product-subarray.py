class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        max_cur = 1
        max_ans = nums[0]
        min_cur = 1
        
        for i in range(len(nums)):
            tmp = max_cur
            max_cur = max(nums[i], max_cur*nums[i], min_cur*nums[i])            
            min_cur = min(nums[i], min_cur*nums[i], tmp*nums[i])
            
            max_ans = max(max_ans, max_cur)            
        
        return max_ans