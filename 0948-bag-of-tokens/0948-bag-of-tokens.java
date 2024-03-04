class Solution {
    public int bagOfTokensScore(int[] tokens, int power) {
        Arrays.sort(tokens);
        int n = tokens.length;
        int left = 0;
        int right = n-1;

        int res = 0;
        int tmp = 0;

        while (left <= right) {
            if (tokens[left] <= power) {
                power -= tokens[left];
                left++;
                tmp++;
            } else if (tokens[right] > power && tmp > 0) {
                power += tokens[right];
                tmp--;
                right--;
            } else {
                left++;
            }
            res = Math.max(res, tmp);
        }
        return res;
    }
}