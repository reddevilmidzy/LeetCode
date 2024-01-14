class Solution {
    public boolean closeStrings(String word1, String word2) {
        Map<Character, Integer> cnt1 = generateCnt(word1);
        Map<Character, Integer> cnt2 = generateCnt(word2);
        List<Integer> val1 = new ArrayList<>(cnt1.values());
        List<Integer> val2 = new ArrayList<>(cnt2.values());
        Collections.sort(val1);
        Collections.sort(val2);
        return Objects.equals(cnt1.keySet(), cnt2.keySet()) && Objects.equals(val1, val2);
    }
    
    public Map<Character, Integer> generateCnt(String word) {
        Map<Character, Integer> res = new HashMap<>();
        for (int i = 0; i < word.length(); i++) {
            int val = res.getOrDefault(word.charAt(i), 0);
            res.put(word.charAt(i), val + 1);
        }
        return res;
    }
}