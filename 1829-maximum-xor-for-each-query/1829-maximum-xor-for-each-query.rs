impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut tot = 0;
        let mut res = vec!();

        for num in &nums {
            tot ^= *num;
        }

        for num in nums.iter().rev() {
            res.push((!tot) & ((1 << maximum_bit) - 1));
            tot ^= *num;
        }
        res
    }
}
