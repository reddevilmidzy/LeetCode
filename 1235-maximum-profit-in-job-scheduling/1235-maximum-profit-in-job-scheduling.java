class Solution {
    public int jobScheduling(int[] startTime, int[] endTime, int[] profit) {
        int n = profit.length;
        Job[] jobs = new Job[n];
        
        for (int i = 0; i < n; i++) {
            jobs[i] = new Job(endTime[i], startTime[i], profit[i]);
        }
        
        Arrays.sort(jobs, Comparator.comparingInt(job -> job.ed));
        
        int[] dp = new int[n+1];
        
        for (int i = 0; i < n; i++) {
            int edVal = jobs[i].ed;
            int stVal = jobs[i].st;
            int proVal = jobs[i].profit;
            
            int index = upperBound(jobs, i, stVal);
            dp[i+1] = Math.max(dp[i], dp[index] + proVal);
        }
        return dp[n];
    }
    
    private int upperBound(Job[] jobs, int endIndex, int target) {
        int low = 0;
        int high = endIndex;
        
        while (low < high) {
            int mid = (low + high) / 2;
            if (jobs[mid].ed <= target) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return low;
    }
    
    
    private static class Job {
        int ed;
        int st;
        int profit;
        
        public Job(int ed, int st, int profit) {
            this.ed = ed;
            this.st = st;
            this.profit = profit;
        }
    }
}