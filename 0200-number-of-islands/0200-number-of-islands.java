class Solution {

    private int n;
    private int m;
    private char[][] grid;

    public int numIslands(char[][] grid) {
        this.grid = grid;
        this.n = grid.length;
        this.m = grid[0].length;
        int res = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (grid[i][j] == '1') {
                    dfs(i, j);
                    res++;
                }
            }
        }
        return res;
    }

    private void dfs(int x, int y) {
        if (x >= n || y >= m || x < 0 || y < 0 || grid[x][y] != '1') {
            return;
        }
        grid[x][y] = '0';

        dfs(x-1, y);
        dfs(x, y-1);
        dfs(x+1, y);
        dfs(x, y+1);
    }
}