class Solution {
    public int minChanges(final String s) {
        int res = 0;
        final int n = s.length();

        for (int i = 1; i < n; i += 2) {
            if (s.charAt(i - 1) != s.charAt(i)) {
                res++;
            }
        }
        return res;
    }
}