use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut nums : HashSet<i32> = HashSet::new();

        for num in arr {
            if nums.contains(&num) {
                return true;
            }
            if num % 2 == 0 {
                nums.insert(num / 2);
            }
            nums.insert(num * 2);
        }

        false
    }
}