class Solution {
    public void sortColors(final int[] nums) {
        final int[] cnt = new int[3];
        for (final int num : nums) {
            cnt[num]++;
        }
        int idx = 0;
        int num = 0;
        for (int val : cnt) {
            for (int i = 0; i < val; i++) {
                nums[idx++] = num;
            }
            num++;
        }
    }
}
