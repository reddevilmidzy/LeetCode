class Solution:
    def maxSubarrayLength(self, nums: list[int], k: int) -> int:
        di = defaultdict(int)
        n = len(nums)
        l,r = 0,0
        pre = -1
        res = 1
        di[nums[0]] = 0
        
        while l <= r and r < n:

            if pre != l:
                res = max(res, r - l + 1)
                pre = l

            if di[nums[r]]+1 <= k:
                di[nums[r]] += 1
                res = max(res, r - l + 1)
                r += 1
            else:
                di[nums[l]] -= 1
                l += 1
                
        if max(di.values()) <= k:
            res = max(res, r - l)
        return res
