impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let mut has_vowel = false;
        let mut has_consonant = false;
        let vowel = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        for c in word.chars() {
            if c.is_alphabetic() {
                if vowel.contains(&c) {
                    has_vowel = true;
                } else {
                    has_consonant = true;
                }
            } else if !c.is_numeric() {
                return false;
            }
        }

        has_vowel && has_consonant
    }
}
