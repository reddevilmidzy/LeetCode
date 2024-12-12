impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut hq = BinaryHeap::from(gifts);

        for _ in 0..k {
            let x = hq.pop().unwrap();
            hq.push(f64::sqrt(x as f64 + 0.5) as i32);
        }

        hq.into_iter().map(|x| x as i64).sum::<i64>()    
    }
}