class Solution {
    public int minSteps(String s, String t) {
        Map<Character, Integer> cnt = new HashMap<>();
        int res = 0;
        for (int i = 0; i < s.length(); i++) {
            int pre = cnt.getOrDefault(s.charAt(i), 0);
            cnt.put(s.charAt(i), pre + 1);
        }
        
        for (int i = 0; i < t.length(); i++) {
            int tmp = cnt.getOrDefault(t.charAt(i), 0);
            if (tmp > 0) {
                cnt.put(t.charAt(i), tmp-1);
                res++;
            }
        }
        return t.length() - res;
    }
}