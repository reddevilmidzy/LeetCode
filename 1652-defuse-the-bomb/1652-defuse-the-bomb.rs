impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n : usize = code.len();
        let mut res = vec![0; n];
        if k == 0 {
            return res;
        }
        let mut st : usize = 1;
        let mut ed : usize = k as usize;
        let mut tot = 0;

        if k < 0 {
            st = n + k as usize;
            ed = n - 1;
        }
        for i in st..=ed {
            tot += code[i];
        }
        for i in 0..n {
            res[i] = tot;
            tot -= code[st % n];
            tot += code[(ed + 1) % n];
            st += 1;
            ed += 1;
        }
        res
    }
}
