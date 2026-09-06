impl Solution {
    pub fn min_cost(nums: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let n = nums.len();
        let m = nums[0].len();
        let nn = n as i32;
        let mm = m as i32;
        let kk = k as usize;
        if k == 0 && n > 1 && m > 1 {
            return -1;
        }
        // visited[i][j][4][cnt] = i j ì¹¸ì 4 ë°©í¥ì¼ë¡ ìê³  cntë² ë°ê¿ 
        let inf = i32::MAX;
        let mut distance = vec![vec![vec![vec![inf; kk + 1]; 4]; m]; n];
        let mut hq = BinaryHeap::new();

        for dir in 0..4 {
            distance[0][0][dir][0] = nums[0][0];
            // cur, cnt, dir, y, x
            hq.push(Reverse((nums[0][0], 0, dir, 0, 0)));
        }
        // ì°, í, ì, ì¢
        let d = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        while let Some(Reverse((cur, cnt, dir, y, x))) = hq.pop() {
            let yy = y as usize;
            let xx = x as usize;
            if distance[yy][xx][dir][cnt] < cur {
                continue;
            }

            for i in 0..4 {
                let dy = d[i].0;
                let dx = d[i].1;
                
                let ny = y + dy;
                let nx = x + dx;

                if ny < 0 || nx < 0 || ny >= nn || nx >= mm {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                if i == dir && cur + nums[ny][nx] < distance[ny][nx][i][cnt] {
                    distance[ny][nx][i][cnt] = cur + nums[ny][nx];
                    hq.push(Reverse((distance[ny][nx][i][cnt], cnt, i, ny as i32, nx as i32)));
                } else if i != dir && cnt + 1 <= kk && cur + nums[ny][nx] < distance[ny][nx][i][cnt + 1] {
                    distance[ny][nx][i][cnt + 1] = cur + nums[ny][nx];
                    hq.push(Reverse((distance[ny][nx][i][cnt + 1], cnt + 1, i, ny as i32, nx as i32)));
                }
            }
        }
        let mut res = inf;

        for i in 0..4 {
            println!("{:?}", distance[n - 1][m - 1][i]);
            res = res.min(*distance[n - 1][m - 1][i].iter().min().unwrap());
        }
        if res == inf {
            -1
        } else {
            res
        }
    }
}
