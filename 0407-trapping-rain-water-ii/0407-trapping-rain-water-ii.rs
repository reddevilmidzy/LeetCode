use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        let m = height_map[0].len();

        let mut hq = BinaryHeap::new();
        let mut visited = vec![vec![false; m]; n];

        let mut res = 0;
        let mut max_h = -1;

        let dy = vec![1, -1, 0, 0];
        let dx = vec![0, 0, -1, 1];

        for y in 0..n {
            for x in 0..m {
                if y == 0 || y == n-1 || x == 0 || x == m-1 {
                    hq.push((Reverse(height_map[y][x]), y as i32, x as i32));
                    visited[y][x] = true;
                }
            }
        }

        while let Some((h, y, x)) = hq.pop() {
            
            max_h = max_h.max(h.0);
            res += max_h - h.0;

            for i in 0..4 {
                let ny = (dy[i] + y) as usize;
                let nx = (dx[i] + x) as usize;

                if ny < 0 || nx < 0 || ny >= n || nx >= m {
                    continue;
                }
                if !visited[ny][nx] {
                    hq.push((Reverse(height_map[ny][nx]), ny as i32, nx as i32));
                    visited[ny][nx] = true;
                }
            }
        }

        res
    }
}