impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut res = 0;
        for word in words {
            if word.starts_with(&pref) {
                res += 1;
            }
        }
        res
    }
}