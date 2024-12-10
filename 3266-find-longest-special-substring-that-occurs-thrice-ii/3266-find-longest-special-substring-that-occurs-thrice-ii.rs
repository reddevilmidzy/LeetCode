impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let st = s.as_bytes();
        let mut pre = st[0];
        let mut cnt = 1;
        let mut alp : Vec<Vec<i32>> = vec![Vec::new(); 26];
        for i in 1..s.len() {
            if pre != st[i] {
                alp[(pre - 'a' as u8) as usize].push(cnt);
                cnt = 1;
                pre = st[i];
            } else {
                cnt += 1;
            }
        }
        alp[(pre - 'a' as u8) as usize].push(cnt);
        let mut res = -1;
        for i in 0..26 {
            if alp[i].len() == 0 {
                continue;
            }
            alp[i].sort_by(|a,b| b.cmp(a));
            res = res.max(alp[i][0] - 2);
            if alp[i].len() >= 2 {
                if alp[i][0] != alp[i][1] {
                    res = res.max(alp[i][1]);
                } else {
                    res = res.max(alp[i][1]-1);
                }
            }
            if alp[i].len() >= 3 {
                res = res.max(alp[i][2]);
            }
        }
        if res == 0 {
            return -1;
        }
        res
    }
}