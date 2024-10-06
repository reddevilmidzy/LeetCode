class Solution:
    def dividePlayers(self, nums: List[int]) -> int:
        nums.sort()
        val = nums[0] + nums[-1]
        n = len(nums)
        res = 0
        for i in range(n//2):
            if val == nums[i]+nums[n-i-1]:
                res += nums[i]*nums[n-i-1]
            else:
                return -1
        return res
    