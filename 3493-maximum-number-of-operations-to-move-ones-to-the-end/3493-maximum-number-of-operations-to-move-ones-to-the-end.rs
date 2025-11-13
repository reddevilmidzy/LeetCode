impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut res = 0;
        let mut one = 0;
        let mut flag = false;

        for c in s.chars() {
            if c == '1' {
                one += 1;
                flag = true;
            } else {
                if flag {
                    res += one;
                    flag = false;
                }
            }
        }
        res
    }
}
