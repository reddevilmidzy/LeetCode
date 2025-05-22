use std::collections::BinaryHeap;
impl Solution {
    pub fn max_removal(mut nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let n: usize = nums.len();
        let m: usize = queries.len();
        let mut pre = vec![0; n + 1];
        queries.sort_by(|a,b| a[0].cmp(&b[0]));

        let mut hq = BinaryHeap::new();
        let mut j = 0;
        let mut op = 0;

        for i in 0..n {
            op += pre[i];
            while j < m && queries[j][0] == i as i32 {
                hq.push(queries[j][1]);
                j += 1;
            }
            while op < nums[i] && !hq.is_empty() && *hq.peek().unwrap() >= i as i32 {
                op += 1;
                let ed = hq.pop().unwrap() as usize;
                pre[ed + 1] -= 1;
            }
            if op < nums[i] {
                return -1;
            }
        }

        hq.len() as i32
    }
}
