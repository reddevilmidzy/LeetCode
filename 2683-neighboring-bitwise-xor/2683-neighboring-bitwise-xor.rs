impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut cnt = 0;
        for num in derived {
            if num == 1 {
                cnt += 1;
            }
        }
        cnt % 2 == 0
    }
}