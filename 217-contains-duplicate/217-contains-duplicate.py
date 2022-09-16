class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        arr = defaultdict(int)
        for i in nums:
            if arr[i] == 1:
                return True
            arr[i] = 1
        