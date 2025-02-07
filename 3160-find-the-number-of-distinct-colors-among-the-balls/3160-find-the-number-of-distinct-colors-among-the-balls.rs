use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut balls = HashMap::new();
        let mut colors = HashMap::<i32, HashSet<i32>>::new();
        let mut res = Vec::new();

        for q in queries.into_iter() {
            let b = q[0];
            let c = q[1];

            if let Some(k) = balls.get(&b) {
                let cc = colors.get_mut(k).unwrap();
                cc.remove(&b);
                if cc.is_empty() {
                    colors.remove(&k);
                }
            }
            colors.entry(c).or_default().insert(b);
            balls.insert(b, c);
            res.push(colors.len() as i32);
        }

        res
    }
}