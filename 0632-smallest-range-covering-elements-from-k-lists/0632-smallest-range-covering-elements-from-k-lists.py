class Solution:
    def smallestRange(self, nums: List[List[int]]) -> List[int]:
        k = len(nums)
        left, right = nums[0][0], nums[0][0]
        hq = []

        for i in range(k):
            left = min(left, nums[i][0])
            right = max(right, nums[i][0])
            heapq.heappush(hq, (nums[i][0], i, 0)) # val, list index, element index

        res = [left, right]
        while True:
            val, li_idx, el_idx = heapq.heappop(hq)

            el_idx += 1
            if el_idx == len(nums[li_idx]):
                return res
            
            nxt = nums[li_idx][el_idx]
            heapq.heappush(hq, (nxt, li_idx, el_idx))
            
            right = max(right, nxt)
            left = max(left, hq[0][0])

            if right - left < res[1] - res[0]:
                res = [left,right]

        return res
