class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = []
        self.bt(nums, res, [], 0)
        return res

    def bt(self, nums: List[int], res: List[int], cur: List[int], idx: int) -> None:
        res.append(cur[::])

        for i in range(idx, len(nums)):
            cur.append(nums[i])
            self.bt(nums, res, cur, i + 1)
            cur.pop()
