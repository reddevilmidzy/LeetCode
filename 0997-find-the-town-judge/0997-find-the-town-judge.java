class Solution {
    public int findJudge(int n, int[][] trust) {
        int[] arr = new int[n+1];
        Set<Integer> canNot = new HashSet<>();
        
        for (int[] per : trust) {
            int a = per[0];
            int b = per[1];
            canNot.add(a);    
            arr[b]++;
        }
        
        List<Integer> res = new ArrayList<>();
        
        for (int i = 1; i <= n; i++) {
            if (arr[i] == n-1) {
                res.add(i);
            }
        }
        
        if (res.size() == 1) {
            int val = res.get(0);
            if (!canNot.contains(val)) {
                return val;
            }
            return -1;
        }
        return -1;
    }
}