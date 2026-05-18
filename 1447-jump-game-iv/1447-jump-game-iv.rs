impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::{VecDeque, HashMap};

        let n = arr.len();
        let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        for i in 0..n {
            let val = graph.entry(arr[i]).or_insert(Vec::new());
            val.push(i);
        }

        queue.push_back((0, 0)); // cur, cnt
        visited[0] = true;
        
        let nn = n as i32;

        while let Some((cur, cnt)) = queue.pop_front() {
            if cur == n - 1 {
                return cnt;
            }
            if let Some(g) = graph.get(&arr[cur]) {
                for &nxt in g {
                    if !visited[nxt] {
                        visited[nxt] = true;
                        queue.push_back((nxt, cnt + 1));
                    }
                }
                graph.remove(&arr[cur]);
            }

            for d in [-1i32, 1i32] {
                let nd = cur as i32 + d;
                if nd < 0 || nd >= nn {
                    continue;
                }
                let nxt = nd as usize;
                if !visited[nxt] {
                    visited[nxt] = true;
                    queue.push_back((nxt, cnt + 1));
                }
            }
        }
        unreachable!()
    }
}
