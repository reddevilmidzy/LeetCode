class Solution {
    public boolean isIsomorphic(final String s, final String t) {
        final int[] sArr = new int[26];
        final int[] tArr = new int[26];

        final int n = s.length();
        for (int i = 0; i < n; i++) {

        }

        return convert(s).equals(convert(t));
    }

    private List<Integer> convert(final String word) {
        final int n = word.length();
        final Map<Character, Integer> index = new HashMap<>();
        final List<Integer> result = new ArrayList<>();

        for (int i = 0; i < n; i++) {
            char ch = word.charAt(i);
            result.add(toNumber(index, ch, i));
        }
        return result;
    }

    private int toNumber(final Map<Character, Integer> index, char ch, final int i) {
        if (index.containsKey(ch)) {
            return index.get(ch);
        }
        index.put(ch, i);
        return index.keySet().size();
    }
}