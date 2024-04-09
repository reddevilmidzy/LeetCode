class Solution {
    public int timeRequiredToBuy(int[] tickets, final int k) {
        final int n = tickets.length;
        int time = 0;
        
        while (tickets[k] > 0) {
            for (int i = 0; i < n; i++) {
                if (tickets[i] == 0) {
                    continue;
                }
                tickets[i]--;
                time++;
                if (tickets[k] == 0) {
                    break;
                }
            }
        }
        return time;
    }
}