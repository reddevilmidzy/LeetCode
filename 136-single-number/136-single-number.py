class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        num_dict = dict()
        for i in nums:
            if i not in num_dict:
                num_dict[i] = 1
            else:
                num_dict[i] = 0
        ans = 0
        for i in nums:
            if num_dict[i] == 1:
                ans = i
        return ans
        