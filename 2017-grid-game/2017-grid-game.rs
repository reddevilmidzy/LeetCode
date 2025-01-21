impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let m = grid[0].len();
        let mut pre = vec![vec![0i64; m]; 2];

        pre[0][0] = grid[0][0] as i64;
        pre[1][0] = grid[1][0] as i64;
        for i in 1..m {
            pre[0][i] = pre[0][i-1] + grid[0][i] as i64;
            pre[1][i] = pre[1][i-1] + grid[1][i] as i64;
        }

        let mut res = i64::MAX;

        for i in 0..m {
            let val = if i > 0 { pre[1][i - 1] } else { 0 };
            let val = val.max(pre[0][m-1] - pre[0][i]);
            res = res.min(val);
        }

        res
    }
}
