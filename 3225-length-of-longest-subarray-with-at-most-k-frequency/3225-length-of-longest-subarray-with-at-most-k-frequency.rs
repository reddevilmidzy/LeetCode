impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut l = 0;
        let mut r = 0;
        let mut res = 0;
        let mut over: Option<i32> = None;

        while r < n {
            if over.is_none() {
                if let Some(cur) = cnt.get(&nums[r]) && *cur == k {
                    over = Some(nums[r]);
                } else {
                    res = res.max(r - l + 1);
                }
                *cnt.entry(nums[r]).or_insert(0) += 1;
                r += 1;
            } else {
                if let Some(val) = over && val == nums[l] {
                    over = None;
                }
                *cnt.entry(nums[l]).or_insert(0) -= 1;
                l += 1;
            }
        }

        res as i32
    }
}
