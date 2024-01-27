class Solution {
    private static final int MOD = 1_000_000_007;
    private static final int MAX = 1000;
    
    public int kInversePairs(int n, int k) {
        int[][] dp = new int[MAX+1][MAX+1];
        dp[0][0] = 1;
        
        for (int i = 1; i <= n; i++) {
            for (int j = 0; j <= k; j++) {
                for (int x = 0; x <= Math.min(j, i-1); x++) {
                    if (j - x >= 0) {
                        dp[i][j] += dp[i-1][j-x];
                        dp[i][j] %= MOD;
                    }
                }
            }
        }
        return dp[n][k];
    }
}