class Solution {
    public int findMinArrowShots(int[][] points) {
        Arrays.sort(points, Comparator.comparingInt(o -> o[1]));
        int res = 1;
        int last = points[0][1];
        for (int[] point : points) {
            if (point[0] <= last && last <= point[1]) {
                continue;
            }
            last = point[1];
            res++;
        }
        return res;
    }
}