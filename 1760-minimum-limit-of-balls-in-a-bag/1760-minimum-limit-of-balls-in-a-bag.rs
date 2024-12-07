impl Solution {
    pub fn can(nums: &Vec<i32>, k: i32, target: i32) -> bool {
        let mut cnt = 0;

        for num in nums {
            cnt += (num + target - 1) / target - 1;
        }

        cnt <= k
    }

    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let n : usize = nums.len();

        let mut left = 1;
        let mut right = 1_000_000_000;

        while left <= right {
            let mid = (left + right) / 2;

            if Self::can(&nums, max_operations, mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}
