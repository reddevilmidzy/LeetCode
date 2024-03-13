class Solution {
    public int pivotInteger(int n) {
        int[] fore = new int[n];
        int[] back = new int[n];
        fore[0] = 1;
        back[0] = n;
        for (int i = 2; i <= n; i++) {
            fore[i-1] = i + fore[i-2];
            back[i - 1] = n - i + 1 + back[i-2];
        }
        for (int i = 0; i < n; i++) {
            if (fore[i] == back[n - i -1]) {
                return i + 1;
            }
        }
        return -1;
    }
}