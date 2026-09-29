impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        use std::collections::VecDeque;
        let n = grid.len();
        let m = grid[0].len();

        if (n + m) % 2 == 0 {
            return false;
        }

        // visited[cnt][y][x] = cntë (ì ê°¯ì
        let mut visited = vec![vec![vec![false; m]; n]; m * n + 1];
        if grid[0][0] == ')' {
            return false;
        }
        visited[1][0][0] = true;
        let mut queue = VecDeque::new();
        queue.push_back((1i32, 0usize, 0usize));

        while let Some((cnt, y, x)) = queue.pop_front() {

            for (dy, dx) in [(0, 1), (1, 0)] {
                let ny = y + dy;
                let nx = x + dx;
                if ny >= n || nx >= m {
                    continue;
                }
                let nct = cnt + if grid[ny][nx] == '(' { 1 } else { -1 };
                // nctê° ì ë° ì´ì ìëë° nct > n/2 + m/2 ë©´ ì´ê²ë ìëì§ ìë
                if nct < 0 {
                    continue;
                }
                let nctu = nct as usize;

                if !visited[nctu][ny][nx] {
                    visited[nctu][ny][nx] = true;
                    queue.push_back((nct, ny, nx));
                }
            }
        }

        visited[0][n - 1][m - 1]
    }
}
