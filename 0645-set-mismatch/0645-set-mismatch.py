class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        n = len(nums)
        n_set = set(nums)
        f_set = set([i for i in range(1, n+1)])
        return [Counter(nums).most_common(1)[0][0], (f_set-n_set).pop()]