impl Solution {
    pub fn missing_integer(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = nums[0];
        for i in 1..n {
            if nums[i - 1] + 1 == nums[i] {
                sum += nums[i];
            } else {
                break;
            }
        }
        nums.sort_unstable();
        nums.dedup();
        let n = nums.len();
        let idx = nums.partition_point(|x| *x < sum);
        if idx < n && nums[idx] == sum {
            for i in idx+1..n {
                if nums[i - 1] + 1 != nums[i] {
                    return nums[i - 1] + 1;
                }
            }
            return nums[n - 1] + 1;
        }
        sum
    }
}
