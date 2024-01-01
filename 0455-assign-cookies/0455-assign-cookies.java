class Solution {
    public int findContentChildren(int[] g, int[] s) {
        Arrays.sort(g);
        Arrays.sort(s);
        int n = g.length;
        int m = s.length;
        int idx = 0;
        int res = 0;
        for (int i = 0; i < n; i++) {
            while (idx < m) {
                if (s[idx] >= g[i]) {
                    res++;
                    idx++;
                    break;
                }
                idx++;
            }
        }
        return res;
    }
}
