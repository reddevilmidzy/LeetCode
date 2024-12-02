impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let s = search_word.as_str();
        let mut idx = 1;
        for word in sentence.split_whitespace() {
            if word.len() >= s.len() && &word[0..s.len()] == s {
                return idx;
            }
            idx += 1;
        }
        -1
    }
}