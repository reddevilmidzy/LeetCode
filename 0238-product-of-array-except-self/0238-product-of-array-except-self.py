class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        pro = 1
        n = len(nums)
        zero = 0
        res = [0]*n
        for num in nums:
            if num:
                pro *= num
            else:
                zero += 1
        if zero >= 2:
            return res
        elif zero == 1:
            res[nums.index(0)] = pro
            return res
        for i in range(n):
            res[i] = pro//nums[i]
        return res