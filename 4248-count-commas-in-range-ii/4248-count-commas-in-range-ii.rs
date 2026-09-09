impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut res: i64 = 0;
        if n < 1000 {
            return res;
        }
        let tens = (1..=6).map(|x| 1000i64.pow(x)).collect::<Vec<_>>();
        for i in 1..tens.len() {
            res += n - tens[i - 1] + 1;
            if tens[i] > n {
                break;
            }
        }
        res
    }
}
