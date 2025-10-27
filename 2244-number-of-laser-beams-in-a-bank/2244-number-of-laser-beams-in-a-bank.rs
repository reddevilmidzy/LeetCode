impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut res = 0;
        let n = bank.len();
        let mut pre = 0;
        for i in 0..n {
            let cur = bank[i].as_str().matches('1').count();
            res += pre * cur;
            if cur > 0 {
                pre = cur;
            }
        }

        res as i32
    }
}
