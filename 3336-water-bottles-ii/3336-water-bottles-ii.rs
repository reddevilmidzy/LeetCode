impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res = num_bottles;
        let mut num = 0;
        let mut emp = num_bottles;
        let mut exc = num_exchange;

        while (num + emp) >= exc {
            if emp >= exc {
                emp -= exc;
                exc += 1;
                num += 1;
            }
            res += num;
            emp += num;
            num = 0;
        }
        res
    }
}
