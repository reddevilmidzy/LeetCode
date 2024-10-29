class Solution:
    def maxMoves(self, grid: List[List[int]]) -> int:
        n = len(grid)
        m = len(grid[0])

        dp = [[-float('inf')]*(m) for _ in range(n)]

        for i in range(n):
            dp[i][0] = 0

        for x in range(m):
            for y in range(n):
                for dy, dx in ((-1, -1), (0, -1), (1, -1)):
                    ny,nx = dy+y, dx+x
                    if ny < 0 or nx < 0 or ny >= n or nx >= m: continue
                    if grid[y][x] > grid[ny][nx] and dp[y][x] < dp[ny][nx] + 1:
                        dp[y][x] = dp[ny][nx] + 1

        return max(map(max, dp))
