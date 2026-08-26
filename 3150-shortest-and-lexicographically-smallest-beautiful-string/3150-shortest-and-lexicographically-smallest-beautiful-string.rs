impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        use std::str::from_utf8;
        let n = s.len();
        let s = s.as_bytes();

        let mut l = 0;
        let mut r = 0;
        let mut one = 0;
        let mut go = true;
        let mut res = (n + 1, n, n);
        let mut ans = "";

        while r < n {
            if go && s[r] == b'1' {
                one += 1;
            }

            if one == k && res.0 > r - l + 1 {
                ans = from_utf8(&s[l..=r]).unwrap();
                res = (r - l + 1, l, r);
            } else if one == k && res.0 == r - l + 1 && ans > from_utf8(&s[l..=r]).unwrap() {
                ans = from_utf8(&s[l..=r]).unwrap();
                res = (r - l + 1, l, r);
            }

            if one < k {
                go = true;
                r += 1;
            } else {
                go = false;
                if s[l] == b'1' {
                    one -= 1;
                }
                l += 1;
            }
        }
        ans.to_string()
    }
}
