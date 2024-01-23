class Solution {
    private int res = 0;
    
    public int maxLength(List<String> arr) {
        List<String> candy = new ArrayList<>();
        for (String s : arr) {
            if (!hasDuplicate(s)) {
                candy.add(s);
            }
        }
        
        backtracking(candy, -1, 0);
        return res;
    }
    
    private boolean hasDuplicate(String val) {
        Set<Character> res = new HashSet<>();
        
        for (char c : val.toCharArray()) {
            if (!res.add(c)) {
                return true;
            }
        }
        return false;
    }
    
    private void backtracking(List<String> arr, int idx, int cur) {
        res = Math.max(res, countBit(cur));
        
        if (idx == arr.size()) {
            return;
        }
        
        for (int i = idx + 1; i < arr.size(); i++) {
            if ((cur&bitmask(arr.get(i))) == 0) {
                backtracking(arr, i, cur|bitmask(arr.get(i)));
            } else {
                backtracking(arr, i, cur);
            }
        }
    }
    
    private int bitmask(String str) {
        int res = 0;
        
        for (int i = 0; i < str.length(); i++) {
            res |= 1 << (str.charAt(i) - 'a');
        }
        return res;
    }
    
    private int countBit(int val) {
        res = 0;
        for (int i = 0; i < 26; i++) {
            if ((val & (1 <<i)) != 0) {
                res++;
            }
        }
        return res;
    }
}