class Solution {
    public int numberOfBeams(String[] bank) {
        int m = bank[0].length();
        int tmp;
        int pre = 0;
        int res = 0;
        for (String s : bank) {
            tmp = 0;
            for (int j = 0; j < m; j++) {
                if (s.charAt(j) == '1') {
                    tmp++;
                }
            }
            if (tmp > 0) {
                res += tmp * pre;
                pre = tmp;
            }
        }
        return res;
    }
}