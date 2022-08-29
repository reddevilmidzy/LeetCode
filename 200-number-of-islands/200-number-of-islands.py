class Solution:    
    def numIslands(self, grid: List[List[str]]) -> int:
        
        
        def dfs(x,y):

            if x>n or y>m or x<0 or y<0 or grid[x][y] != "1":
                return
            
            grid[x][y] = "0"

            dfs(x-1,y)
            dfs(x,y-1)
            dfs(x+1,y)
            dfs(x,y+1)
        
        n = len(grid)-1
        m = len(grid[0])-1
        ans = 0
        
        for i in range(n+1):
            for j in range(m+1):
                if grid[i][j]=="1":
                    dfs(i,j)
                    ans += 1
        return ans