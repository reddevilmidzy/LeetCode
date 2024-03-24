class Solution {
    public int findDuplicate(int[] nums) {
        int tortoise = nums[0];
        int hare = nums[0];

        while (true) {
            tortoise = nums[tortoise];
            hare = nums[nums[hare]];
            if (hare == tortoise) { // 토끼 거북이 만남
                break;
            }
        }
        // 거북이 처음으로 보냄
        tortoise = nums[0];
        while (hare != tortoise) {
            tortoise = nums[tortoise];
            hare = nums[hare];
        }
        return hare;
    }
}