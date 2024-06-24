class Solution:
    def minKBitFlips(self, nums: List[int], k: int) -> int:
        n = len(nums)
        queue = deque()
        res = 0
        for i in range(n):
            if queue and queue[0] == i-k:
                queue.popleft()
            if (nums[i]+len(queue))%2 == 0:
                if i + k > n:
                    return -1
                queue.append(i)
                res += 1
        
        return res
