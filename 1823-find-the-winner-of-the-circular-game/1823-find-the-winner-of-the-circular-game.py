class Solution:
    def findTheWinner(self, n: int, k: int) -> int:
        queue = deque([i for i in range(1, n+1)])
        res = -1
        while queue:
            for _ in range(k - 1):
                x = queue.popleft()
                queue.append(x)
            res = queue.popleft()
        return res
