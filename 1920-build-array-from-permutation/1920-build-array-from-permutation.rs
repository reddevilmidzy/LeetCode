impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];

        for i in 0..n {
            res[i] = nums[nums[i] as usize];
        }
        
        res
    }
}
