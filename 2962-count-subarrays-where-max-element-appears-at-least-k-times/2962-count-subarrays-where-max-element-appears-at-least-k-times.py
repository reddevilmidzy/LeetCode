class Solution:
    def countSubarrays(self, nums: List[int], k: int) -> int:
        mv = max(nums)
        res = st = mv_win = 0

        for end in nums:
            if end == mv:
                mv_win += 1
            while mv_win==k:
                if nums[st] == mv:
                    mv_win -= 1
                st += 1
            res += st
        return res
