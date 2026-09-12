impl Solution {
    pub fn find_circle_num(is: Vec<Vec<i32>>) -> i32 {
        let n = is.len();
        let mut graph = vec![Vec::new(); n];

        for i in 0..n {
            for j in i+1..n {
                if is[i][j] == 1 {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        let mut stk = Vec::new();
        let mut visited = vec![false; n];
        let mut res = 0;

        for i in 0..n {
            if !visited[i] {
                res += 1;
                stk.clear();
                visited[i] = true;
                stk.push(i);

                while let Some(cur) = stk.pop() {
                    for &nxt in &graph[cur] {
                        if !visited[nxt] {
                            visited[nxt] = true;
                            stk.push(nxt);
                        }
                    }
                }
            }
        }

        res
    }
}
