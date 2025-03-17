impl Solution {
    pub fn divide_array(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        for i in (0..nums.len()).step_by(2) {
            if nums[i] != nums[i+1] {
                return false;
            }
        }
        true
    }
}
