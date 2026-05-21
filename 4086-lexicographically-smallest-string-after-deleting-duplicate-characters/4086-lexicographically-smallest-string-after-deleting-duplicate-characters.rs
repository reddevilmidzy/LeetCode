impl Solution {
    pub fn lex_smallest_after_deletion(s: String) -> String {
        let mut res = s.clone();
        for i in (97u8..123u8).rev() {
            let c = i as char;
            res = cutting(res, c);
        }
        res    
    }
}

fn get_idx(c: char) -> usize {
    c as u8 as usize - 'a' as u8 as usize
}

fn cutting(s: String, val: char) -> String {
    let mut tmp = s.chars().collect::<Vec<char>>();
    tmp.dedup();

    let n = tmp.len();

    let mut cnt = vec![0; 26];
    let mut con = Vec::with_capacity(n);
    let mut pre_val = (s[0..1]).to_string().chars().nth(0).unwrap();
    let mut con_cnt = 0;

    for c in s.chars() {
        cnt[get_idx(c)] += 1;
        if c != pre_val {
            con.push(con_cnt);
            con_cnt = 1;
        } else {
            con_cnt += 1;
        }

        pre_val = c;
    }
    con.push(con_cnt);

    // ì´ê±´ ì»¤í ëª»í¨
    if cnt[get_idx(val)] <= 1 {
        return s;
    }

    let mut res = String::new();
    let mut left = tmp.iter().filter(|x| **x == val).count();

    let mut removed = false;

    println!("val = {val} ìì");

    for i in 0..n {
        if tmp[i] != val {
            // ê±ì¶ê°
            res.push_str(&format!("{}", tmp[i]).repeat(con[i]));
            continue;
        }
        let idx = get_idx(tmp[i]);
        left -= 1;

        if i == n - 1 && removed {
            // println!("ì´ë¯¸ ì§ì´ì  ìì");
            continue;
        } else if i + 1 < n && tmp[i] > tmp[i + 1] { // bba
            // println!("tmp[i] > tmp[i + 1], {}, {}  ê·¸ë¦¬ê³  left = {left}",tmp[i], tmp[i + 1]);
            // ë¤ì ë¨ê¸¸ê² ë¨ìì¼ë ì§ê¸ì ë¤ ì ê±°
            if left > 0 || removed {
                continue;
            }
            res.push(val);
            removed = true;
        } else if i == n - 1 {
            // ê·¼ë° ì´ê² ë¬¸ìì ë§ì§ë§ì¸ì§ íì¸í´ì¼í¨
            // println!("ë§ì§ë§ tmp");
            if !removed {
                res.push(val);
            }
        } else { // bbc
            // println!("else");
            res.push_str(&format!("{val}").repeat(con[i]));
            removed = true;
        }
    }
    
    res
}