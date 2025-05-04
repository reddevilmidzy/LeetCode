impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut cnt = vec![0; 100];

        for domino in dominoes {
            let min_v = domino[0].min(domino[1]);
            let max_v = domino[0].max(domino[1]);

            let idx = (max_v * 10 + min_v) as usize;
            res += cnt[idx];
            cnt[idx] += 1;
        }

        res
    }
}
