impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut res: Vec<i32> = Vec::with_capacity(n - k as usize + 1);

        let mut l = 0;
        let mut cnt = 1;

        for r in 0..n {
            if r > 0 && nums[r - 1] + 1 == nums[r] {
                cnt += 1;
            }

            if r - l + 1 > k as usize {
                if nums[l] + 1 == nums[l + 1] {
                    cnt -= 1;
                }
                l += 1;
            }
            if r - l + 1 == k as usize {
                res.push(if cnt == k { nums[r] } else { -1 });
            }
        }
        res
    }
}
