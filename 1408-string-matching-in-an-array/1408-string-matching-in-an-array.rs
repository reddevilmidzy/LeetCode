impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let n: usize = words.len();

        for i in 0..n {
            for j in 0..n {
                if words[j].contains(words[i].as_str()) && i != j {
                    res.push(words[i].clone());
                    break;
                }
            }
        }
        res
    }
}