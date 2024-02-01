class Solution {
    public int[][] divideArray(int[] nums, int k) {
        int n = nums.length;
        Arrays.sort(nums);
        int[][] res = new int[n/3][3];
        
        for (int i = 0; i < n - 2; i += 3) {
            if (nums[i+2] - nums[i] <= k) {
                res[i/3] = new int[]{nums[i], nums[i+1], nums[i+2]};
            } else {
                return new int[0][0];
            }
        }

        return res;
    }
}