class Solution {
    public int[] sortedSquares(int[] nums) {
        int n = nums.length;
        int left = 0;
        int right = n-1;
        int[] res = new int[n];
        int idx = n-1;
        while (left <= right) {
            if (Math.abs(nums[left]) < Math.abs(nums[right])) {
                res[idx] = nums[right] * nums[right];
                right--;
            } else if (Math.abs(nums[left]) >= Math.abs(nums[right])) {
                res[idx] = nums[left] * nums[left];
                left++;
            }
            idx--;
        }
        return res;
    }
}