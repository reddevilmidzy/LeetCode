impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut pre = vec![0; n + 1];

        for query in queries {
            pre[query[0] as usize] += 1;
            pre[query[1] as usize + 1] -= 1;
        }

        for i in 1..=n {
            pre[i] += pre[i-1];
        }

        for i in 0..n {
            if nums[i] > pre[i] {
                return false;
            }
        }

        true
    }
}
