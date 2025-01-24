use std::collections::VecDeque;

impl Solution {
    pub fn is_safe(cur: usize, graph: &Vec<Vec<i32>>, state: &mut Vec<i32>) -> bool {
        if state[cur] != 0 {
            return state[cur] == 2;
        }
        state[cur] = 1;
        for &nxt in &graph[cur] {
            if state[nxt as usize] == 1 || !Self::is_safe(nxt as usize, graph, state) {
                return false;
            }
        }
        state[cur] = 2;
        true
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut state = vec![0; n];
        let mut res = Vec::new();

        for cur in 0..n {
            if Self::is_safe(cur, &graph, &mut state) {
                res.push(cur as i32);
            }
        }

        res
    }
}
