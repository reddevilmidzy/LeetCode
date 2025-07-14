impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut l = 0;
        let mut r = n as i32 - 1;
        while l < r {
            let mid = ((l + r) / 2);
            if nums[mid as usize] > nums[mid as usize + 1] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }
}
