impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        use std::collections::{VecDeque, HashMap};
        const INF: i32 = 1_000_000;
        let n = classroom.len();
        let m = classroom[0].len();
        let nn = n as i32;
        let mm = m as i32;
        let mut graph = vec![vec![0; m]; n];
        let mut queue = VecDeque::new();
        let energy = energy as usize;
        // energyë¥¼ visited ê°ì ë°ê¸°??
        // let mut visited = vec![vec![vec![vec![INF; m]; n]; energy + 1]; 1 << 11];
        
        // visited[cnt][y][x]
        let mut visited = vec![vec![vec![-1; m]; n]; 1 << 11];
        let mut litter = 0;
        let mut idx = HashMap::new();

        for i in 0..n {
            let c = classroom[i].as_bytes();
            for j in 0..m {
                if c[j] == b'S' {
                    // y,x,cnt,dist
                    queue.push_back((i, j, 0, 0));
                    visited[0][i][j] = energy as i32;
                    graph[i][j] = 0;
                } else if c[j] == b'X' {
                    graph[i][j] = -1;
                } else if c[j] == b'L' {
                    idx.insert((i, j), litter);
                    litter += 1;
                    graph[i][j] = 1;
                } else if c[j] == b'R' {
                    graph[i][j] = 2;
                }
            }
        }
        let end = (1 << litter) - 1;

        while let Some((y, x, cnt, dist)) = queue.pop_front() {
            // println!("({y},{x}), cnt = {cnt}, dis t= {dist}");
            if cnt == end {
                return dist;
            }
            if visited[cnt][y][x] == 0 {
                continue;
            }

            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;

                if ny < 0 || nx < 0 || ny >= nn || nx >= mm {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                if graph[ny][nx] == -1 {
                    continue;
                }
                if graph[ny][nx] == 2 && energy as i32 > visited[cnt][ny][nx] {
                    visited[cnt][ny][nx] = energy as i32;
                    queue.push_back((ny, nx, cnt, dist + 1));
                } else if graph[ny][nx] == 0 && visited[cnt][y][x] - 1 > visited[cnt][ny][nx] {
                    visited[cnt][ny][nx] = visited[cnt][y][x] - 1;
                    queue.push_back((ny, nx, cnt, dist + 1));
                } else if  graph[ny][nx] == 1 {
                    let l_idx = idx.get(&(ny, nx)).unwrap();
                    if visited[cnt][y][x] - 1 > visited[cnt | (1 << l_idx)][ny][nx] {
                        visited[cnt | (1 << l_idx)][ny][nx] = visited[cnt][y][x] - 1;
                        queue.push_back((ny, nx, cnt | (1 << l_idx), dist + 1));
                    }
                }
            }
        }
        -1
        
    }
}
