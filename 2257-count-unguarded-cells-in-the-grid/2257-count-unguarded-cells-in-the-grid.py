class Solution:
    def countUnguarded(self, n: int, m: int, guards: List[List[int]], walls: List[List[int]]) -> int:
        
        visited = [[0]*m for _ in range(n)]
        # visited = [[[False] * 4 for _ in range(m)] for _ in range(n)]
        dy = (1, 0, -1, 0)
        dx = (0, 1, 0, -1)

        board = [[1]*m for _ in range(n)]
        for y,x in walls:
            board[y][x] = 0
        
        gs = set((y,x) for y,x in guards)
        for guard in guards:
            # y,x = guard
            visited[guard[0]][guard[1]] = 0
            for i in range(4):
                y,x = guard
                while True:
                    ny,nx = dy[i] + y, dx[i] + x
                    if ny < 0 or nx < 0 or ny >= n or nx >= m: break
                    if not board[ny][nx] or (ny,nx) in gs: break
                    visited[ny][nx] = 1
                    y,x = ny,nx

        # print(*visited,sep='\n')
        # print('n*m', n * m, 'sum', sum(map(sum, visited)), 'guards', len(guards), 'walls', len(walls))
        return n*m - sum(map(sum, visited)) - len(guards) - len(walls)