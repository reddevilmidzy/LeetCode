impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        let mut changed = vec![0; n];

        for i in 0..n-2 {
            if (changed[i] + nums[i]) % 2 == 0 {
                changed[i] += 1;
                changed[i+1] += 1;
                changed[i+2] += 1;
                res += 1;
            }
        }

        for i in 0..n {
            if (changed[i] + nums[i]) % 2 == 0 {
                return -1;
            }
        }

        res
    }
}
