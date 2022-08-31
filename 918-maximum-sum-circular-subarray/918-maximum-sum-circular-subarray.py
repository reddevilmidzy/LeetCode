class Solution:
    def maxSubarraySumCircular(self, nums: List[int]) -> int:
        tot, min_sum, min_cur, max_sum, max_cur = sum(nums),nums[0],0,nums[0],0
        # 최대값을 구하는 방법은 배열에서 최대값을 찾거나, sum(배열)-최소값을 한 값이다.
        for i in nums:
            min_cur = min(min_cur+i, i)
            min_sum = min(min_sum, min_cur)
            max_cur = max(max_cur+i, i)
            max_sum = max(max_sum, max_cur)
        
            
        return max(max_sum, tot-min_sum) if max_sum > 0 else max_sum
            
        
        