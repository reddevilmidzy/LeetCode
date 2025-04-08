impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let m = 100;
        let n = nums.len();
        let mut have = vec![false; m + 1];

        for i in (0..n).rev() {
            if have[nums[i] as usize] {
                return (i as i32 + 3) / 3;
            }
            have[nums[i] as usize] = true;
        }
        0
    }
}
