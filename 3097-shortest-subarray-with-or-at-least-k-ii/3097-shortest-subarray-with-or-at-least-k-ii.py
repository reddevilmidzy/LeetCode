class Solution:
    def minimumSubarrayLength(self, nums: List[int], k: int) -> int:
        n = len(nums)
        m = 31
        if nums[0] >= k:
            return 1
        res = float('inf')
        one = 0
        two = 1
        pre = -1
        tmp = [0]*m
        for i in range(m):
            if nums[one] & (1 << i):
                tmp[m - i - 1] += 1

        while one <= two < n:
            for i in range(m):
                if (pre != two) and (nums[two] & (1 << i)):
                    tmp[m - i - 1] += 1

            val = sum([2**(m-i-1) for i in range(m) if tmp[i]])
            pre = two

            if val >= k:
                res = min(res, two - one + 1)
                for i in range(m):
                    if nums[one] & (1 << i):
                        tmp[m - i - 1] -= 1
                one += 1
            else:
                two += 1

        return res if res != float('inf') else -1
