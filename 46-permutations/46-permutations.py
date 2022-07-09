from itertools import combinations

class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        ans = []
        for i in permutations(nums, len(nums)):
            ans.append(list(i))
        return ans