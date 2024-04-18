class Solution {

    private static final int[] dy = {1, 0, -1, 0};
    private static final int[] dx = {0, 1, 0, -1};

    public int islandPerimeter(final int[][] grid) {
        for (int i = 0; i < grid.length; i++) {
            for (int j = 0; j < grid[0].length; j++) {
                if (grid[i][j] == 1) {
                    return bfs(grid, new Position(i, j));
                }
            }
        }
        return 0;
    }

    private int bfs(final int[][] grid, final Position start) {
        final Queue<Position> queue = new LinkedList<>();
        final int n = grid.length;
        final int m = grid[0].length;
        final boolean[][] visited = new boolean[n][m];
        int res = 0;
        visited[start.y][start.x] = true;
        queue.add(start);

        while (!queue.isEmpty()) {
            final Position cur = queue.poll();
            int cnt = 0;
            for (int i = 0; i < 4; i++) {
                final int ny = cur.y + dy[i];
                final int nx = cur.x + dx[i];

                if ((ny < 0 || nx < 0 || ny >= n || nx >= m) || (grid[ny][nx] == 0)) {
                    cnt++;
                    continue;
                } 
                if (grid[ny][nx] == 1 && !visited[ny][nx]) {
                    queue.add(new Position(ny, nx));
                    visited[ny][nx] = true;
                }
            }
            res += cnt;
        }
        return res;
    }

    record Position(int y, int x) {
    }
}
