class Solution {
    

    public int maxSatisfied(final int[] customers, final int[] grumpy, final int minutes) {
        final int n = customers.length;
        final List<Integer> pre = new ArrayList<>();
        pre.add(0);
        int res = 0;
        for (int i = 0; i < n; i++) {
            if (grumpy[i] == 0) {
                res += customers[i];
                pre.add(pre.get(i));
            } else {
                pre.add(pre.get(i) + customers[i]);
            }
        }

        int mx = 0;
        for (int i = minutes; i <= n; i++) {
            mx = Math.max(mx, pre.get(i) - pre.get(i - minutes));
        }

        return res + mx;
    }
}
