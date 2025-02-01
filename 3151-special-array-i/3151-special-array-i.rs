impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|x| (x[0] ^ x[1]) & 1 == 1)
    }
}