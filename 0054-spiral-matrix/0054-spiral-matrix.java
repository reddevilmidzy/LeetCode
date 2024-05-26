class Solution {

    private static final int[] dy = {0, 1, 0, -1};
    private static final int[] dx = {1, 0, -1, 0};

    public List<Integer> spiralOrder(final int[][] matrix) {
        final List<Integer> res = new ArrayList<>();
        final int n = matrix.length;
        final int m = matrix[0].length;
        int y = 0;
        int x = 0;
        int cnt = n * m - 1;

        final boolean[][] visited = new boolean[n][m];
        visited[y][x] = true;
        res.add(matrix[y][x]);
        int d = 0;

        while (cnt > 0) {
            int ny = y + dy[d];
            int nx = x + dx[d];

            if (ny < 0 || nx < 0 || ny >= n || nx >= m) {
                d = (d + 1) % 4;
                continue;
            }
            if (visited[ny][nx]) {
                d = (d + 1) % 4;
                continue;
            }
            cnt--;
            visited[ny][nx] = true;
            res.add(matrix[ny][nx]);
            y = ny;
            x = nx;
        }

        return res;
    }
}
