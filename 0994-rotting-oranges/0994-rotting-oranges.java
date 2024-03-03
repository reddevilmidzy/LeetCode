class Solution {
    private final int[] dy = new int[] { 1, 0, -1, 0 };
    private final int[] dx = new int[] { 0, 1, 0, -1 };

    public int orangesRotting(int[][] grid) {
        final int n = grid.length;
        final int m = grid[0].length;

        Queue<Position> queue = new LinkedList<>();
        boolean[][] visited = new boolean[n][m];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (grid[i][j] == 2) {
                    visited[i][j] = true;
                    queue.add(new Position(i, j));
                }
            }
        }
        
        int res = 0;
        while (!queue.isEmpty()) {
            Position cur = queue.poll();
            res = cur.cnt;
            for (int i = 0; i < 4; i++) {
                int ny = dy[i] + cur.y;
                int nx = dx[i] + cur.x;

                if (ny < 0 || ny >= n || nx < 0 || nx >= m) {
                    continue;
                }
                if (!visited[ny][nx] && grid[ny][nx] == 1) {
                    queue.add(new Position(ny, nx, cur.cnt + 1));
                    visited[ny][nx] = true;
                }
            }
        }

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (grid[i][j] == 1 && !visited[i][j]) {
                    return -1;
                }
            }
        }

        return res;
    }

    static class Position {
        private final int y;
        private final int x;
        private final int cnt;

        public Position(int y, int x) {
            this.y = y;
            this.x = x;
            this.cnt = 0;
        }

        public Position(int y, int x, int cnt) {
            this.y = y;
            this.x = x;
            this.cnt = cnt;
        }

    }
}