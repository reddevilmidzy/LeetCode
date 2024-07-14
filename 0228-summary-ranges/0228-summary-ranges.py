class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        res = []
        n = len(nums)
        st = nums[0]
        for i in range(1, n):
            if nums[i-1] + 1 == nums[i]:
                pass
            else:
                if st == nums[i-1]:
                    res.append(str(st))
                else:
                    res.append(f"{st}->{nums[i-1]}")
                st = nums[i]
        
        if st == nums[-1]:
            res.append(str(st))
        else:
            res.append(f"{st}->{nums[-1]}")
        return res
