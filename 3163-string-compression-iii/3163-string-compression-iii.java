class Solution {
    public String compressedString(final String word) {
        final int n = word.length();
        int i = 1;
        int cnt = 1;
        char c = word.charAt(0);
        final StringBuilder sb = new StringBuilder();
        while (i < n) {
            if (word.charAt(i) == c && cnt + 1 < 10) {
                cnt++;
            } else {
                sb.append(cnt).append(c);
                cnt = 1;
                c = word.charAt(i);
            }
            i++;
        }
        sb.append(cnt).append(c);
        return sb.toString();
    }
}
