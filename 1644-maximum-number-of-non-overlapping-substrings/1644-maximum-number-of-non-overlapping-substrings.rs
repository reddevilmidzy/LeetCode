impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let n = s.len();
        let m = 26;
        let mut st = vec![n as i32; m];
        let mut ed = vec![-1; m];
        let mut cross = vec![vec![false; m]; m];

        let s = s.as_bytes();
        let mut i = 0;
        
        for c in s {
            let c = (c - b'a') as usize;
            st[c] = st[c].min(i);
            ed[c] = ed[c].max(i);

            for k in 0..m {
                if c == k {
                    continue;
                }
                if st[c] < ed[k] {
                    // println!("hit c = {c}, k = {k}");
                    cross[c][k] = true;
                } 
            }
            i += 1;
        }
        let mut candy = Vec::new();

        for k in 0..m {
            for i in 0..m {
                for j in 0..m {
                    if cross[i][k] && cross[k][j] {
                        cross[i][j] = true;
                    }
                }
            }
        }

        for i in 0..m {
            for j in 0..m {
                if !cross[i][j] || i == j {
                    continue;
                }
                if ed[i] != -1 {
                    // println!("i = {i}, j = {j}");
                    st[i] = st[i].min(st[j]);
                    ed[i] = ed[i].max(ed[j]);
                }
            }
        }
        for i in 0..m {
            if ed[i] != -1 {
                candy.push((st[i], ed[i], (i as u8 + b'a') as char));
            }
        }
        candy.sort_by_key(|&x| x.1);
        let mut res = Vec::new();

        let mut pre = 0;

        for (st, ed, hit) in candy {
            //  pre=0ì ì ì©ìí¤ê¸° ìí´
            if pre <= st {
                res.push(str::from_utf8(&s[st as usize..=ed as usize]).unwrap().to_string());
                pre = ed;
            }
        }
        // println!("st = {st:?}");
        // println!("ed = {ed:?}");
        // println!("{res:?}");

        res
    }
}
