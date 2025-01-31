use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn bfs(par: i32, n: i32, r: usize, c: usize, visited: &mut Vec<Vec<i32>>, grid: &Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        visited[r][c] = par;
        queue.push_back((r as i32,c as i32));
        let dy = vec![0, 1, -1, 0];
        let dx = vec![1, 0, 0, -1];
        let mut cnt = 0;

        while let Some((y,x)) = queue.pop_front() {
            cnt += 1;
            for i in 0..4 {
                let ny = dy[i] + y;
                let nx = dx[i] + x;
                if ny < 0 || nx < 0 || ny >= n || nx >= n {
                    continue;
                }
                if visited[ny as usize][nx as usize] == -1 && grid[ny as usize][nx as usize] == 1 {
                    visited[ny as usize][nx as usize] = par;
                    queue.push_back((ny, nx));
                }
            }
        }

        cnt
    }

    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 1;
        let n = grid.len();
        let mut par = 0;
        let mut visited = vec![vec![-1; n]; n];

        let mut store = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if visited[i][j] == -1 && grid[i][j] == 1 {
                    let cnt = Self::bfs(par, n as i32, i, j, &mut visited, &grid);
                    res = res.max(cnt);
                    store.push(cnt);
                    par += 1;
                }
            }
        }

        let dy = vec![0, 1, -1, 0];
        let dx = vec![1, 0, 0, -1];

        let un = n as i32;

        for y in 0..n {
            for x in 0..n {
                if grid[y][x] == 0 {
                    let mut tmp = 1;
                    let mut land = HashSet::new();
                    for k in 0..4 {
                        let ny = dy[k] + y as i32;
                        let nx = dx[k] + x as i32;
                        if ny < 0 || nx < 0 || ny >= un || nx >= un {
                            continue;
                        }
                        let ny = ny as usize;
                        let nx = nx as usize;
                        if visited[ny][nx] != -1 {
                            land.insert(visited[ny][nx] as usize);
                        }
                    }
                    for val in land {
                        tmp += store[val];
                    }
                    res = res.max(tmp);
                }
            }
        }

        res
    }
}
