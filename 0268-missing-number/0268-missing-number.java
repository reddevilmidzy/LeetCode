class Solution {
    public int missingNumber(int[] nums) {
        int n = nums.length;
        int tot = n*(n+1)/2;
        
        for (int num : nums) {
            tot -= num;    
        }
        
        return tot;
    }
}