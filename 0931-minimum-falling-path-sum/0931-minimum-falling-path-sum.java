class Solution {
    public int minFallingPathSum(int[][] matrix) {
        int n = matrix.length;
        int[][] dp = new int[n][n];
        
        for (int i = 0; i < n; i++) {
            dp[0][i] = matrix[0][i];
        }
        
        for (int i = 1; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (0 < j && j < n-1) {
                    dp[i][j] = min(dp[i-1][j-1], dp[i-1][j], dp[i-1][j+1]) + matrix[i][j];
                } else if (j == 0) {
                    dp[i][j] = min(dp[i-1][j], dp[i-1][j+1]) + matrix[i][j];
                } else {
                    dp[i][j] = min(dp[i-1][j], dp[i-1][j-1]) + matrix[i][j];
                }
            }
        }
        
        
        return min(dp[n-1]);
    }
    
    
    private int min(int x, int y) {
        return Math.min(x, y);
    }
    
    private int min(int x, int y, int z) {
        return Math.min(Math.min(x, y), z);
    }
    
    private int min(int[] arr) {
        int res = arr[0];
        for (int num : arr) {
            res = Math.min(res, num);
        }
        return res;
    }
}