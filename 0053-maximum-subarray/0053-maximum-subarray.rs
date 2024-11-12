impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        let mut cur = 0;
        for num in nums {
            cur = num.max(cur + num);
            res = res.max(cur);
        }
        res
    }
}