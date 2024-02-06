class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        List<List<String>> res = new ArrayList<>();
        Map<String, List<String>> ans = new HashMap<>();
        
        for (String str : strs) {
            String cnt = createCounter(str);
            List<String> tmp = ans.getOrDefault(cnt, new ArrayList<>());
            tmp.add(str);
            ans.put(cnt, tmp);
        }

        for (List<String> val : ans.values()) {
            res.add(val);
        }
        return res;
    }
    
    private String createCounter(String str) {
        int[] res = new int[26];
        for (char c : str.toCharArray()) {
            res[c - 'a']++;
        }
        return Arrays.toString(res);
    }
}