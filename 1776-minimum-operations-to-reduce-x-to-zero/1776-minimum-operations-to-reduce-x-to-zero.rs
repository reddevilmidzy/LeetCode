impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let mut go = Vec::with_capacity(n + 1);
        let mut back = vec![0; n + 1];
        go.push(0);
        for i in 0..n {
            go.push(nums[i] + go[i]);
            back[n - i - 1] = back[n - i] + nums[n - i - 1];
        }
        back.reverse();
        // println!("{go:?}");
        // println!("{back:?}");
        let mut res = i32::MAX;
        for i in 0..=n {
            let v = x - go[i];
            let idx = back.partition_point(|&x| x < v);
            if idx <= n && back[idx] == v && i + idx <= n {
                res = res.min((i + idx) as i32);
            }
            // println!("v  is {v}, idx = {idx},");
        }

        if res != i32::MAX {
            res
        } else {
            -1
        }
    }
}
