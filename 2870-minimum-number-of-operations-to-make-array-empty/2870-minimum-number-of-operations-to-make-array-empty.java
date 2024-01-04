class Solution {
    public int minOperations(int[] nums) {
        Map<Integer, Integer> tmp = new HashMap<>();

        for (int num : nums) {
            tmp.put(num, tmp.getOrDefault(num, 0) + 1);
        }

        int res = 0;

        for (Integer num : tmp.values()) {
            if (num < 2) {
                return -1;
            }
            res += num / 3 + Math.min(1, (num%3) * 1);
        }
        return res;
    }
}