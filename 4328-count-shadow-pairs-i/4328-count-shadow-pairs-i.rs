impl Solution {
    pub fn shadow_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut stk = Vec::new();
        let mut res = 0i64;
        let mut min_idx = vec![n; n];
        let mut same = vec![1; n];

        for i in 0..n {
            while let Some(&cur) = stk.last() && nums[cur] >= nums[i] {
                if nums[cur] == nums[i] {
                    same[i] += same[cur];
                    break;
                }
                min_idx[stk.pop().unwrap()] = i;
            }
            stk.push(i);
        }

        // println!("{min_idx:?}");
        // println!("{same:?}");

        for i in 0..n-1 {
            res += (min_idx[i] - i - 1) as i64 - (same[i] - 1);
        }
        res -= same[n - 1] - 1;
        
        res
    }
}
