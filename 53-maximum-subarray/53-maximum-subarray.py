class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        # n = len(nums)
        now = ans= nums[0] # now nums
        # ans = nums[0] # for ans nums
        
        for i in range(1, len(nums)): # 0번째 인덱스 제외
            now = max(nums[i], now+nums[i]) # 현재부터 시작하는 것과, 이전의 값과 현재를 더하는것
            ans = max(now, ans) # now 값이 갱신되어도 기억하고 있는 최대값
            
        return ans