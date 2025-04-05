impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for &num in &nums {
            res |= num;
        }
        res << (nums.len() as i32 - 1)
    }
}
