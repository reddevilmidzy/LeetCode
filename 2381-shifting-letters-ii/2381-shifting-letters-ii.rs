impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut cnt = vec![0; n+1];

        for shift in shifts {
            cnt[shift[0] as usize] += -1 + 2*shift[2];
            cnt[shift[1] as usize + 1] += 1 - 2*shift[2];
        }

        for i in 1..=n {
            cnt[i] += cnt[i-1];
        }

        let mut res = String::with_capacity(n);
        for (idx, c) in s.chars().enumerate() {
            cnt[idx] = ((cnt[idx] % 26) + 26) % 26;
            let tmp = ('a' as u8 + ((c as i32 - 'a' as i32 + cnt[idx]) % 26) as u8) as char;
            res.push(tmp);
        }

        res
    }
}