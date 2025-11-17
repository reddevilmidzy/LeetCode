impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut cnt = k;
        for num in nums {
            if num == 0 {
                cnt += 1;
            } else {
                if cnt < k {
                    return false;
                }
                cnt = 0;
            }
        }
        true
    }
}
