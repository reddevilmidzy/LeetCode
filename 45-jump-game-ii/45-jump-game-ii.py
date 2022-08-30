class Solution:
    def jump(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1: return 0 # 무조건 갈 수 있음
        elif n == 2: return 1
        l, r = 0, nums[0]
        times = 1
        while r < n - 1:
            times += 1
            nxt = max(i + nums[i] for i in range(l, r + 1))
            l, r = r, nxt
        return times