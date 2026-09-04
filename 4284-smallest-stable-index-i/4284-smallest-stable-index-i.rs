impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        for i in 0..n {
            let cur = nums[..=i].iter().max().unwrap() - nums[i..].iter().min().unwrap();
            if cur <= k {
                return i as i32;
            }
        }
        
        -1
    }
}
