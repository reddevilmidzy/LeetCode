class Solution {
    public int largestCombination(final int[] nums) {
        final int n = nums.length;
        final int m = 24;

        int res = 1;

        for (int bit = 0; bit <= m; bit++) {
            int tmp = 0;
            for (int num : nums) {
                if ((num & (1 << bit)) != 0) {
                    tmp++;
                }
            }
            res = Math.max(res, tmp);
        }
        return res;
    }
}