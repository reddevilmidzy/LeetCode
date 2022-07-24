from itertools import permutations

class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        ans = list()
        for i in permutations(nums, len(nums)):
            if i not in ans:
                ans.append(i)
            
        return ans