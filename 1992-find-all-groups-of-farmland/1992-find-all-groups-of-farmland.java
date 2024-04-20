class Solution {
    public int[][] findFarmland(final int[][] land) {
        final int n = land.length;
        final int m = land[0].length;

        final List<int[]> res = new ArrayList<>();

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (land[i][j] == 1) {
                    int y = i;
                    int x = j;
                    for (y = i; y < n && land[y][j] == 1; y++) {
                        for (x = j; x < m && land[y][x] == 1; x++) {
                            land[y][x] = 0;
                        }
                    }
                    final int[] ans = new int[]{i, j, y - 1, x - 1};
                    res.add(ans);
                }
            }
        }
        return res.stream()
                    .toArray(int[][] :: new);
    }
}
