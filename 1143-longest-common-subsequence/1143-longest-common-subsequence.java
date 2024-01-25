class Solution {
    public int longestCommonSubsequence(String text1, String text2) {
        int n = text1.length();
        int m = text2.length();
        
        if (m < n) {
            int tmp = n;
            n = m;
            m = tmp;
            
            String val = text1;
            text1 = text2;
            text2 = val;
        }
        
        int[] dp = new int[n + 1];
        int pre;
        int cur;
        
        for (int i = 1; i <= m; i++) {
            pre = 0;
            for (int j = 1; j <= n; j++) {
                cur = dp[j];
                if (text1.charAt(j-1) == text2.charAt(i-1)) {
                    dp[j] = pre + 1;
                } else {
                    dp[j] = Math.max(dp[j-1], cur);
                }
                pre = cur;
            }
        }
        
        return dp[n];
    }
}