class Solution {
    public boolean isIsomorphic(final String s, final String t) {
        final int[] sArr = new int[26];
        final int[] tArr = new int[26];

        final int n = s.length();
        for (int i = 0; i < n; i++) {
            sArr[s.charAt(i) - 'a']++;
            tArr[t.charAt(i) - 'a']++;
        }

        final Map<Integer, Integer> sMap = new HashMap<>();
        final Map<Integer, Integer> tMap = new HashMap<>();
        
        for (int i = 0; i < 26; i++) {
            sMap.put(sArr[i], sMap.getOrDefault(sArr[i], 0) + 1);
            tMap.put(tArr[i], tMap.getOrDefault(tArr[i], 0) + 1);
        }
        return sMap.equals(tMap);
    }
}