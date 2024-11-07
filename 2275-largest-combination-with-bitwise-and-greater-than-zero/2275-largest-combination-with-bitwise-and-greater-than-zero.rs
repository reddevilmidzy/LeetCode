impl Solution {
    pub fn largest_combination(nums: Vec<i32>) -> i32 {
        (0..24)
            .map(|bit| nums.iter().filter(|&num| num & (1 << bit) != 0).count() as i32)
            .max()
            .unwrap_or(0)
    }
}
