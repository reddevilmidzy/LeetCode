class Solution {
    public int maxFrequencyElements(int[] nums) {
        int[] count = new int[101];
        for (int num : nums) {
            count[num]++;
        }

        int maxCount = max(count);
        int res = 0;
        for (int val : count) {
            if (val == maxCount) {
                res += val;
            }
        }
        return res;
    }

    private int max(int[] count) {
        int res = 0;
        for (int val : count) {
            res = Math.max(res, val);
        }
        return res;
    }
}