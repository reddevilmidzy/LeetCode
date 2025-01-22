use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let dy = vec![0, 1, 0, -1];
        let dx = vec![1, 0, -1, 0];

        let n = is_water.len();
        let m = is_water[0].len();

        let mut res = vec![vec![-1; m]; n];
        let mut queue = VecDeque::new();
        
        for i in 0..n {
            for j in 0..m {
                if is_water[i][j] == 1 {
                    queue.push_back((i, j));
                    res[i][j] = 0;
                }
            }
        }

        let n = n as i32;
        let m = m as i32;

        while let Some((y, x)) = queue.pop_front() {
            for i in 0..4 {
                let ny = y as i32 + dy[i];
                let nx = x as i32 + dx[i];

                if ny < 0 || nx < 0 || ny >= n || nx >= m {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                if is_water[ny][nx] == 0 && res[ny][nx] == -1 {
                    res[ny][nx] = res[y][x] + 1;
                    queue.push_back((ny, nx));
                }
            }
        }

        res
    }
}