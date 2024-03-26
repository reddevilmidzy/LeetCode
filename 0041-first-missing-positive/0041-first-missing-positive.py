class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:

        # 지금 내가 알고 싶은 것은
        # mex(nums).
        # x <= 0 or x > n 은 필요 없음

        n = len(nums)
        contains_one = False

        for i in range(n):
            if nums[i] == 1:
                contains_one = True
            if nums[i] <= 0 or nums[i] > n:
                nums[i] = 1

        # 1없으면 무조건 1
        if not contains_one:
            return 1

        for i in range(n):
            val = abs(nums[i])
            if val == n:
                nums[0] = -abs(nums[0])
            else:
                nums[val] = -abs(nums[val])

        for i in range(1, n):
            if nums[i] > 0:
                return i
        if nums[0] > 0:
            return n
        return n + 1