class Solution:
    def deleteAndEarn(self, nums: List[int]) -> int:
        cnt = Counter(nums) # 카운터는 갯수 세서 dict 처럼만들어둠
        nums = sorted(list(set(nums))) # 중복제거후 정렬
        
        dp1, dp2 = 0,0
        for i in range(len(nums)):
            tmp = nums[i]*cnt[nums[i]] # nums[i]*갯수
            
            if i > 0 and nums[i] == nums[i-1] + 1:
                dp1, dp2 = dp2, max(tmp+dp1, dp2)
            
            else:
                dp1, dp2 = dp2, tmp+dp2
        return dp2
    
        