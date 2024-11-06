impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let (mut min_num, mut max_num) = (nums[0], nums[0]);
        let mut pre = 0;
        for num in nums {
            if num.count_ones() != max_num.count_ones() {
                min_num = num;
                pre = max_num;
            }
            min_num = min_num.min(num);
            max_num = max_num.max(num);

            if min_num < pre {
                return false;
            }
        }
        true
    }
}
