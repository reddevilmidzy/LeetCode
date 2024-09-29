class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        n = len(nums)
        j = 0
        cnt = 1
        pre = nums[0] - 1
        for i in range(n):
            if nums[i] == pre:
                cnt += 1
            else:
                cnt = 1
            pre = nums[i]
            if cnt <= 2:
                nums[j] = nums[i]
                j += 1

        return j
