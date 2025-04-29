impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mv = *nums.iter().max().unwrap();
        let mut res = 0;
        let mut cnt = 0;
        let mut idx = Vec::new();
        let k = k as usize;

        for i in 0..n {
            if nums[i] == mv {
                idx.push(i as i64);
                cnt += 1;
            }
            if cnt >= k {
                res += idx[idx.len() - k] + 1;
            }
        }

        res
    }
}
