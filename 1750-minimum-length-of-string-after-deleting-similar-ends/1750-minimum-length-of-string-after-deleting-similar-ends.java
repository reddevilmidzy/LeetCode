class Solution {
    public int minimumLength(String s) {
        int n = s.length();
        int left = 0;
        int right = n-1;
        if (n == 1) {
            return 1;
        }
        while (left <= right) {
            if (s.charAt(left) == s.charAt(right)) {
                char leftChar = s.charAt(left);
                while (left < n && leftChar == s.charAt(left)) {
                    left++;
                }
                char rightChar = s.charAt(right);
                while (0 <= right && rightChar == s.charAt(right)) {
                    right--;
                }
                if (left == right) {
                    return 1;
                } else if (left > right) {
                    return 0;
                }
            } else {
                break;
            }
        }
        System.out.println(left);
        System.out.println(right);
        return right - left + 1;
    }
}