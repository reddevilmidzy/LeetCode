class Solution:
    def countMatchingSubarrays(self, nums: List[int], pattern: List[int]) -> int:
        n = len(nums)
        pre = []


        for i in range(n-1):
            if nums[i] < nums[i+1]:
                pre.append(1)
            elif nums[i] > nums[i+1]:
                pre.append(-1)
            else:
                pre.append(0)
        
        # kmp
        m = len(pattern)
        fail = [0]*m
        i = 0

        for j in range(1, m):
            while i > 0 and pattern[i] != pattern[j]:
                i = fail[i-1]
            
            if pattern[i] == pattern[j]:
                i += 1
                fail[j] = i
        
        res = 0
        i = 0
        for j in range(n-1):
            while i > 0 and pre[j] != pattern[i]:
                i = fail[i-1]
            
            if pattern[i] == pre[j]:
                i += 1
                if i == len(pattern):
                    res += 1
                    i = fail[i-1]

        return res