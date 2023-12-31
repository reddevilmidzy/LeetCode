class Solution:
    def updateMatrix(self, mat:List[List[int]]):
        if not mat:
            return mat
        rows, cols = len(mat), len(mat[0])
        distances = [[float('inf') for _ in range(cols)] for _ in range(rows)]
        queue = deque()

        for i in range(rows):
            for j in range(cols):
                if mat[i][j] == 0:
                    distances[i][j] = 0
                    queue.append((i, j))
                    
        d = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        
        while queue:
            x, y = queue.popleft()
            for dx, dy in d:
                nx, ny = x + dx, y + dy
                if 0 <= nx < rows and 0 <= ny < cols:
                    if distances[nx][ny] > distances[x][y] + 1:
                        distances[nx][ny] = distances[x][y] + 1
                        queue.append((nx, ny))
        return distances
