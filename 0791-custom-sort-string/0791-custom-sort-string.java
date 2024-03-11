class Solution {
    public String customSortString(String order, String s) {
        String result = "";
        int n = order.length();
        for (int i = 0; i < n; i++) {
            char target = order.charAt(i);
            int countChar = count(s, target);
            result += String.valueOf(target).repeat(countChar);
            s = s.replace(String.valueOf(target), "");
        }
        return result + s;
    }

    private int count(String word, char target) {
        return (int) word.chars()
            .map(ch -> (char) ch)
            .filter(ch -> ch == target)
            .count();
    }
}