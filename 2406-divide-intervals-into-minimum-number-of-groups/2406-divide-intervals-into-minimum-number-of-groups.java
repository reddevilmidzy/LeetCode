class Solution {
    public int minGroups(int[][] intervals) {
        Arrays.sort(intervals, (a,b) -> a[0] - b[0]);
        final PriorityQueue<Integer> pq = new PriorityQueue<>();

        for (final int[] num : intervals) {
            if (!pq.isEmpty() && pq.peek() < num[0]) {
                pq.poll();
            }
            pq.offer(num[1]);
        }

        return pq.size();
    }
}