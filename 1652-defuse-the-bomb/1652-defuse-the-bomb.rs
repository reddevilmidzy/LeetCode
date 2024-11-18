impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n : usize = code.len();
        let mut res = vec![0; n];
        if k > 0 {
            for i in 0..n {
                let mut tmp = 0;
                for j in (i+1)..=(i+k as usize) {
                    tmp += code[j % n];
                }
                res[i] = tmp;
            }
        } else if k < 0 {
            for i in 0..n {
                let mut tmp = 0;
                for j in 1..=(-k as usize) {
                    tmp += code[(n + i - j) % n];
                }
                res[i] = tmp;
            }
        }
        res
    }
}
