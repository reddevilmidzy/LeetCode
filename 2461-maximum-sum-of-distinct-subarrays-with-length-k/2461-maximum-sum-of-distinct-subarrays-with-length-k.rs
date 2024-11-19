use std::collections::HashSet;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n : usize = nums.len();
        let mut tmp : i64 = 0;
        let mut res : i64 = 0;

        let mut vis : HashSet<i32> = HashSet::new();

        let mut l : usize = 0;
        let mut r : usize = 0;

        while r < n {
            while vis.contains(&nums[r]) {
                vis.remove(&nums[l]);
                tmp -= nums[l] as i64;
                l += 1;
            }
            vis.insert(nums[r]);
            tmp += nums[r] as i64;
            if r - l + 1 == k as usize {
                res = res.max(tmp);
                vis.remove(&nums[l]);
                tmp -= nums[l] as i64;
                l += 1;
            }
            r += 1;
        }
        res
    }
}
