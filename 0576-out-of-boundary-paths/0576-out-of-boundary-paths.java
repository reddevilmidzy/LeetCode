class Solution {
    public int findPaths(int m, int n, int maxMove, int startRow, int startColumn) {
        int mod = 1_000_000_007;
        int[][] dp = new int[m][n];
        dp[startRow][startColumn] = 1;
        
        int cnt = 0;
        
        for (int moves = 1; moves <= maxMove; moves++) {
            int[][] tmp = new int[m][n];
            
            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (i == m-1) {
                        cnt = (cnt + dp[i][j]) % mod;
                    }
                    if (j == n-1) {
                        cnt = (cnt + dp[i][j]) % mod;
                    }
                    if (i == 0) {
                        cnt = (cnt + dp[i][j]) % mod;
                    }
                    if (j == 0) {
                        cnt = (cnt + dp[i][j]) % mod;
                    }
                    tmp[i][j] = ((((i > 0) ? dp[i-1][j] : 0) + ((i < m - 1) ? dp[i+1][j] : 0)) % mod +
                        (((j > 0) ? dp[i][j-1] : 0) + ((j < n-1) ? dp[i][j+1] : 0)) % mod) % mod;
                }
            }
            dp = tmp;
        }
        return cnt;
    }
}