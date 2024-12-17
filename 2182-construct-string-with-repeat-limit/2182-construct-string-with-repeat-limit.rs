impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut cnt = vec![0; 26];
        let mut res = String::new();

        for c in s.bytes() {
            cnt[(c - 'a' as u8) as usize] += 1;
        }
        
        let mut lst : i32 = 25;
        while lst >= 0 {
            let c : char = (lst as u8 + 'a' as u8) as char;
            if cnt[lst as usize] <= repeat_limit {
                if !res.is_empty() {
                    let tmp = res.pop().unwrap();
                    if tmp == c {
                        break;
                    }
                    res.push(tmp);
                }
                res.push_str(c.to_string().repeat(cnt[lst as usize] as usize).as_str());
                cnt[lst as usize] = 0;
                lst -= 1;
            } else {
                while cnt[lst as usize] > 0 {
                    let mut flag = false;
                    res.push_str(c.to_string().repeat(repeat_limit.min(cnt[lst as usize]) as usize).as_str());
                    cnt[lst as usize] -= repeat_limit;
                    if lst == 0 {
                        break;
                    }

                    for i in (0..lst).rev() {
                        if cnt[lst as usize] > 0 && cnt[i as usize] > 0 {
                            let cc : char = (i as u8 + 'a' as u8) as char;
                            res.push(cc);
                            cnt[i as usize] -= 1;
                            flag = true;
                            break;
                        }
                    }
                    if !flag {
                        break;
                    }
                }
                lst -= 1;
            }
        }

        res
    }
}