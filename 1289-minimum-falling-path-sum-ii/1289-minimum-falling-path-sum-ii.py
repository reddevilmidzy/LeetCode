class Solution:
    def minFallingPathSum(self, grid: List[List[int]]) -> int:
        n = len(grid)

        memo = [[inf] * n for _ in range(n)]

        min_col = None
        second_min_col = None
        
        for col in range(n):
            memo[n - 1][col] = grid[n - 1][col]
            if min_col is None or memo[n - 1][col] <= memo[n - 1][min_col]:
                second_min_col = min_col
                min_col = col
            elif second_min_col is None or memo[n - 1][col] <= memo[n - 1][second_min_col]:
                second_min_col = col
         
        for row in range(n - 2, -1, -1):
            current_min_col = None
            current_second_min_col = None

            for col in range(n):
                if col != min_col:
                    memo[row][col] = grid[row][col] + memo[row + 1][min_col]
                else:
                    memo[row][col] = grid[row][col] + memo[row + 1][second_min_col]
                
                if current_min_col is None or memo[row][col] <= memo[row][current_min_col]:
                    current_second_min_col = current_min_col
                    current_min_col = col
                elif current_second_min_col is None or memo[row][col] <= memo[row][current_second_min_col]:
                    current_second_min_col = col
            
            min_col = current_min_col
            second_min_col = current_second_min_col
        
        return memo[0][min_col]
