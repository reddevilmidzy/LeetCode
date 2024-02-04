class Solution {
    public int maximizeTheProfit(int n, List<List<Integer>> offers) {
        int dp[] = new int[n+1];
        List<List<List<Integer>>> m = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            m.add(new ArrayList<List<Integer>>());
        }
        for (List<Integer> tmp : offers) {
            m.get(tmp.get(1)).add(tmp);
        }
        
        for (int e = 1; e <= n; e++) {
            dp[e] = dp[e-1];
            for (List<Integer> val : m.get(e - 1)) {
                dp[e] = Math.max(dp[e], dp[val.get(0)] + val.get(2));
            }
        }
        return dp[n];
    }
}