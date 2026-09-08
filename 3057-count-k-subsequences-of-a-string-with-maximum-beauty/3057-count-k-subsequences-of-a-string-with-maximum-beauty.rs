impl Solution {
    pub fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        const MOD: i64 = 10i64.pow(9) + 7;
        const M: usize = 26;
        let mut res = 1;
        let n = s.len();
        let mut k = k as usize;
        let mut cnt: HashMap<&u8, i32> = HashMap::new();
        let s = s.as_bytes();
        for i in 0..n {
            *cnt.entry(&s[i]).or_default() += 1;
        }
        if k > 26 || cnt.len() < k {
            return 0;
        }
        let mut vals = cnt.values().collect::<Vec<_>>();
        vals.sort_by_key(|&num| -num);
        // println!("{vals:?}");

        let t = vals[k - 1];
        fn comb(n: i64, r: i64) -> i64 {
            let mut res = 1;
            let r = r.min(n - r);
            for i in 0..r {
                res = res * (n - i) / (i + 1);
            }
            res
        }
        let mut cnt = 0;
        for i in 0..k {
            let val = *vals[i] as i64;
            res *= val;
            res %= MOD;
        }
        let l = vals.partition_point(|&x| t < x);
        let r = vals.partition_point(|&x| t <= x);
        let n = (r - l) as i64;

        // println!("res = {res}, l = {l}, r = {r}, n = {n}");
        // println!("nCr = {n}C{}, commb {}",k - l,  comb(n, (k - l) as i64)); 
        res *= comb(n, (k - l) as i64);
        res %= MOD;

        res as i32
    }
}
