impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();

        let mut res = 0;
        if m%2 != 0 {
            for num in nums1 {
                res ^= num;    
            }
        }

        if n%2 != 0 {
            for num in nums2 {
                res ^= num;
            }
        }

        res
    }
}