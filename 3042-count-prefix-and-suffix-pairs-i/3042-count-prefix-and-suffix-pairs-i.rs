impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut res = 0;
        let n = words.len();
        for i in 0..n {
            for j in i+1..n {
                if Self::is_prefix_and_suffix(words[i].as_str(), words[j].as_str()) {
                    res += 1;
                }
            }
        }
        res
    }

    pub fn is_prefix_and_suffix(str1: &str, str2: &str) -> bool {
        str2.starts_with(str1) && str2.ends_with(str1)
    }
}