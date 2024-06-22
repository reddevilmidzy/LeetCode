class Solution:
    def numberOfSubarrays(self, nums: List[int], k: int) -> int:
        n = len(nums)
        res = 0
        pre = [0]
        for num in nums:
            pre.append(pre[-1] + num%2)

        for i in range(n + 1):
            for j in range(i + 1, n + 1):
                if pre[j] - pre[i] == k:
                    res += 1
                elif pre[j] - pre[i] > k:
                    break
        return res
