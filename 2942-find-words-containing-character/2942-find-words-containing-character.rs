impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..words.len() {
            if words[i].contains(x) {
                res.push(i as i32);
            }
        }
        
        res
    }
}
