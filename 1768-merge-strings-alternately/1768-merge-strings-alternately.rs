impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let mut i = 0;
        let mut j = 0;
        let n = word1.len();
        let m = word2.len();

        let mut word1 = word1.bytes();
        let mut word2 = word2.bytes();

        while i < n || j < m {
            match word1.nth(0) {
                Some(c) => res.push(c as char),
                _ => {},
            };
            match word2.nth(0) {
                Some(c) => res.push(c as char),
                _ => {},
            }

            i += 1;
            j += 1;
        }
        
        res
    }
}
