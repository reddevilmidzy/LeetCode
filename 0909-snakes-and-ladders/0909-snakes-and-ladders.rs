use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut jump = vec![0; n * n + 1];
        let mut idx = 1;

        for i in 0..n {
            if i%2==0 {
                for j in 0..n {
                    if board[n-i-1][j] != -1 {
                        jump[idx] = board[n-i-1][j] as usize;
                    }
                    idx += 1;
                }
            } else {
                for j in (0..n).rev() {
                    if board[n-i-1][j] != -1 {
                        jump[idx] = board[n-i-1][j] as usize;
                    }
                    idx += 1;
                }
            }
        }
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited = vec![-1; n * n + 1];
        visited[1] = 0;
        queue.push_back(1);

        while let Some(cur) = queue.pop_back() {
            for i in 1..=6 {
                if cur + i > n * n {
                    continue;
                }
                let nxt = if jump[cur + i] == 0 { cur + i } else { jump[cur + i] };
                if visited[nxt] == -1 || visited[nxt] > visited[cur] + 1{
                    visited[nxt] = visited[cur] + 1;
                    queue.push_back(nxt);
                }
            }
        }

        visited[n*n]
    }
}
