impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = 0;
        let mut row = vec![0; n];
        let mut col = vec![0; m];

        for y in 0..n {
            for x in 0..m {
                if grid[y][x] == 1 {
                    row[y] += 1;
                    col[x] += 1;
                }
            }
        }

        for y in 0..n {
            for x in 0..m {
                if grid[y][x] == 1 {
                    if row[y] > 1 || col[x] > 1 {
                        res += 1;
                    }
                }
            }
        }

        res
    }
}