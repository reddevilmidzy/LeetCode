impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut tmp = Vec::with_capacity(n);
        for i in 0..n {
            tmp.push((-nums[i], i));
        }
        tmp.sort_unstable();
        let mut tmp1 = Vec::with_capacity(k);
        for i in 0..k {
            tmp1.push((tmp[i].1, -tmp[i].0));
        }
        tmp1.sort_unstable();
        let mut res = Vec::with_capacity(k);
        for i in 0..k {
            res.push(tmp1[i].1);
        }
        res
    }
}
