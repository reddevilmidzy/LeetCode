class Solution {    
    public String firstPalindrome(String[] words) {
        for (String word : words) {
            if (isPalindrome(word)) {
                return word;
            }
        }
        return "";
    }
    
    private boolean isPalindrome(String word) {
        int n = word.length();
        for (int i = 0; i < n/2; i++) {
            if (!Objects.equals(word.charAt(i), word.charAt(n-i-1))) {
                return false;
            }
        }
        return true;
    }
}