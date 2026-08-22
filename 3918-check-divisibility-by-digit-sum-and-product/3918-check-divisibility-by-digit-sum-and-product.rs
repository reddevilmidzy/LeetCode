impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let x = n.to_string();
        let mut sum = 0;
        let mut pro = 1;

        for c in x.chars() {
            let val = (c as u8 - '0' as u8) as i32;
            sum += val;
            pro *= val;
        }

        n % (sum + pro) == 0
    }
}
