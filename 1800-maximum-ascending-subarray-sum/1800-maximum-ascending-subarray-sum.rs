impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut res = 0;

        for i in 0..n {
            let mut tmp = nums[i];
            res = res.max(nums[i]);
            for j in (1..=i).rev() {
                if nums[j] > nums[j-1] {
                    tmp += nums[j-1];
                } else {
                    tmp = 0;
                    break;
                }
                res = res.max(tmp);
            }
        }

        res
    }
}