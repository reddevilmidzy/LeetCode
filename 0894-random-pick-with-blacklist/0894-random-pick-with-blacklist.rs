struct Solution {
    n: i32,
    pre: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(n: i32, mut black: Vec<i32>) -> Self {
        black.sort_unstable();
        let mut tmp = Vec::new();
        if !black.is_empty() {
            tmp.push(black[0] - 0);
        }
        for i in 1..black.len() {
            tmp.push(black[i] - black[i - 1] - 1);
        }
        if let Some(v) = black.last() {
            tmp.push(n - v);
        }

        let mut pre = Vec::new();
        pre.push(0);
        for i in 0..tmp.len() {
            pre.push(pre[i] + tmp[i]);
        }
        // println!("{pre:?}");

        let n = n - black.len() as i32;

        Self { n, pre }
    }

    fn pick(&self) -> i32 {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let t: i32 = rng.gen_range(0..self.n);
        // println!("t is {t}");

        let idx = self.pre.partition_point(|&x| x <= t) as i32;

        t + idx - 1
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */
