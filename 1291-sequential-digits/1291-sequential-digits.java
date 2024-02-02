class Solution {
    public List<Integer> sequentialDigits(int low, int high) {
        String s = "123456789";
        List<Integer> res = new ArrayList<>();
        
        int n = String.valueOf(low).length();
        int m = String.valueOf(high).length();
        
        for (int i = n; i <= m; i++) {
            for (int j = 0; j < 10-i; j++) {
                int val = Integer.parseInt(s.substring(j, j+i));
                if (low <= val && val <= high) {
                    res.add(val);
                }
            }
        }
        return res;
    }
}