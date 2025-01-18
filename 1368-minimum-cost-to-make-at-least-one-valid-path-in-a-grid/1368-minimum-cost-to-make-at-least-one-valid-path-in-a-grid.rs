use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let INF: i32 = 1_000_000_000;
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;
        let dy = vec![0, 0, 0, 1, -1];
        let dx = vec![0, 1, -1, 0, 0];

        let mut distance = vec![vec![INF; m as usize]; n as usize];
        let mut hq = BinaryHeap::new();
        let st_y = 0;
        let st_x = 0;
        distance[st_y][st_x] = 0;
        hq.push((Reverse(distance[st_y][st_x]), st_y, st_x));

        while let Some((Reverse(dist), y, x)) = hq.pop() {
            
            if dist > distance[y][x] {
                continue;
            }

            for i in 1..=4 {
                let ny = dy[i] + y as i32;
                let nx = dx[i] + x as i32;

                if ny < 0 || nx < 0 || ny >= n || nx >= m {
                    continue;
                }
                let cost = if i as i32 == grid[y][x] { dist } else { dist + 1 };
                let ny = ny as usize;
                let nx = nx as usize;
                if cost < distance[ny][nx] {
                    distance[ny][nx] = cost;
                    hq.push((Reverse(cost), ny, nx));
                }
            }
        }

        distance[n as usize - 1][m as usize - 1]
    }
}