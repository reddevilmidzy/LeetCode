class Solution:
    def findMaxK(self, nums: List[int]) -> int:
        s = set(nums)
        cnt = [float('inf')]*1001
        for num in s:
            if cnt[abs(num)] == float('inf'):
                cnt[abs(num)] = 0
            cnt[abs(num)] += num

        for i in range(1000, -1, -1):
            if cnt[i] == 0:
                return i
        return -1
