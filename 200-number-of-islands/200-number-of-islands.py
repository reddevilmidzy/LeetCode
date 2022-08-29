class Solution:    
    def numIslands(self, grid: List[List[str]]) -> int:
        
        
        def dfs(x,y):

            if x==n or y==m or x==-1 or y==-1:
                return
            if grid[x][y]=="1":
                grid[x][y] = "0"
                
                dfs(x-1,y)
                dfs(x,y-1)
                dfs(x+1,y)
                dfs(x,y+1)
        
        n = len(grid)
        m = len(grid[0])
        ans = 0
        
        for i in range(n):
            for j in range(m):
                if grid[i][j]=="1":
                    dfs(i,j)
                    ans += 1
        return ans