use std::collections::HashSet;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut res: HashSet<String> = HashSet::new();
        let n: usize = words.len();

        for i in 0..n {
            for j in i+1..n {
                if words[i].contains(words[j].as_str()) {
                    res.insert(words[j].clone());
                } else if words[j].contains(words[i].as_str()) {
                    res.insert(words[i].clone());
                }
            }
        }
        let res: Vec<String> = res.iter().map(|x| x.clone()).collect();
        res
    }
}