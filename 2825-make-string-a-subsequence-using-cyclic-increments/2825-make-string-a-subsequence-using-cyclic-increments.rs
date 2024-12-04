impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let n = str1.len();
        let m = str2.len();

        let mut i : usize = 0;
        let mut j : usize = 0;

        let s1 : Vec<char> = str1.chars().collect();
        let s2 : Vec<char> = str2.chars().collect();

        while i < n && j < m {
            if s1[i] == s2[j] || (s1[i] as u8 - 96) % 26 + 97 == s2[j] as u8 {
                j += 1;
            } 
            i += 1;
        }

        j == m
    }
}