impl Solution {
    pub fn largest_combination(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = 24;

        let mut res = 1;

        for bit in 0..=m {
            let mut tmp = 0;
            for &num in &nums {
                if (num & (1 << bit)) != 0 {
                    tmp += 1;
                }
            }
            res = res.max(tmp);
        }
        res
    }
}
