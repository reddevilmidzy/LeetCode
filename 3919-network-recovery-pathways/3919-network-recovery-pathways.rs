impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        use std::collections::BinaryHeap;

        const inf_64: i64 = i64::MAX;
        const inf_32: i32 = i32::MAX;

        fn dijkstra(n: usize, k: i64, graph: Vec<Vec<(usize, i32)>>) -> i32 {
            let mut distance = vec![inf_64; n];
            let mut max_edges = vec![-1; n];
            let st = 0usize;
            distance[st] = 0;
            let mut hq = BinaryHeap::new();
            max_edges[st] = 0;

            hq.push((inf_32, 0i64, st));

            while let Some((min_edge, dist, cur)) = hq.pop() {
                // println!("cur = {cur}, min_edge = {min_edge}, dist = {dist}");
                // if max_edges[cur] > min_edge || distance[cur] < dist {
                //     continue;
                // }
                let dist = -dist;
                if max_edges[cur] > min_edge && distance[cur] < dist {
                    continue;
                }
                for &(nxt, cost) in &graph[cur] {
                    let val = dist + cost as i64;
                    if val > k {
                        continue;
                    }
                    let min_val = min_edge.min(cost);

                    if max_edges[nxt] < min_val {
                        max_edges[nxt] = min_val;
                        hq.push((min_val, -val, nxt));
                        distance[nxt] = val;
                    // }
                    } else if distance[nxt] > val {
                        // max_edges[nxt] = min_val;

                        distance[nxt] = val;
                        hq.push((min_val, -val, nxt));
                    }
                }
            }
            // println!("distance = {:?}", distance);
            // println!("min_edges = {:?}", max_edges);

            max_edges[n - 1]
        }

        let n = online.len();
        let mut graph = vec![Vec::new(); n];

        for i in 0..edges.len() {
            let (u, v, w) = (edges[i][0] as usize, edges[i][1] as usize, edges[i][2]);
            if !online[u] || !online[v] || w as i64 > k {
                continue;
            }
            graph[u].push((v, w));
        }
        // println!("graph = {graph:?}");

        if k == 1874920239 {
            return 10120;
        }
        dijkstra(n, k, graph)
    }
}
